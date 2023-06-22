use crate::{ast::*, language::MarkdownLanguage, parser::MarkdownParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// Markdown 语言的 AST 构建器
#[derive(Clone)]
pub struct MarkdownBuilder<'config> {
    /// 语言配置
    config: &'config MarkdownLanguage,
}

impl<'config> MarkdownBuilder<'config> {
    /// 创建新的 Markdown 构建器
    pub fn new(config: &'config MarkdownLanguage) -> Self {
        Self { config }
    }

    /// 从语法树构建 AST 根节点
    fn build_root(&self, green_tree: &GreenNode<MarkdownLanguage>, source: &SourceText) -> Result<MarkdownRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);

        let mut blocks = Vec::new();
        for child in red_root.children() {
            if let RedTree::Node(node) = child {
                if let Some(block) = self.build_block(node, source) {
                    blocks.push(block);
                }
            }
        }

        Ok(MarkdownRoot { blocks })
    }

    /// 构建块级元素
    fn build_block(&self, node: RedNode<MarkdownLanguage>, source: &SourceText) -> Option<Block> {
        use crate::kind::MarkdownSyntaxKind::*;

        let kind = node.green.kind;
        match kind {
            Heading1 | Heading2 | Heading3 | Heading4 | Heading5 | Heading6 => {
                let level = match kind {
                    Heading1 => 1,
                    Heading2 => 2,
                    Heading3 => 3,
                    Heading4 => 4,
                    Heading5 => 5,
                    Heading6 => 6,
                    _ => unreachable!(),
                };
                let text = source.get_text_in(node.span());
                let content = text.trim_start_matches('#').trim_start().to_string();
                Some(Block::Heading(crate::ast::Heading { level, content, span: node.span() }))
            }
            Paragraph => Some(Block::Paragraph(crate::ast::Paragraph { content: source.get_text_in(node.span()).to_string(), span: node.span() })),
            CodeBlock => {
                let mut language = None;
                let mut content = String::new();

                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == CodeLanguage {
                                language = Some(source.get_text_in(leaf.span).trim().to_string());
                            }
                            else if leaf.kind != CodeFence {
                                content.push_str(&source.get_text_in(leaf.span));
                            }
                        }
                        RedTree::Node(child_node) => {
                            // 检查子节点是否包含语言标识
                            for sub_child in child_node.children() {
                                if let RedTree::Leaf(sub_leaf) = sub_child {
                                    if sub_leaf.kind == CodeLanguage {
                                        language = Some(source.get_text_in(sub_leaf.span).trim().to_string());
                                    }
                                    else if sub_leaf.kind != CodeFence {
                                        content.push_str(&source.get_text_in(sub_leaf.span));
                                    }
                                }
                                else if let RedTree::Node(sub_node) = sub_child {
                                    content.push_str(&source.get_text_in(sub_node.span()));
                                }
                            }
                        }
                    }
                }

                Some(Block::CodeBlock(crate::ast::CodeBlock { language, content: content.trim().to_string(), span: node.span() }))
            }
            UnorderedList | OrderedList => {
                let mut items = Vec::new();
                for child in node.children() {
                    if let RedTree::Node(child_node) = child {
                        if child_node.green.kind == ListItem {
                            items.push(self.build_list_item(child_node, source));
                        }
                    }
                }
                Some(Block::List(crate::ast::List { is_ordered: kind == OrderedList, items, span: node.span() }))
            }
            Blockquote => {
                let mut content_text = String::new();
                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => {
                            if leaf.kind != BlockquoteMarker {
                                content_text.push_str(&source.get_text_in(leaf.span));
                            }
                        }
                        RedTree::Node(child_node) => {
                            content_text.push_str(&source.get_text_in(child_node.span()));
                        }
                    }
                }

                // 简单的引用处理：将其内容作为段落
                Some(Block::Blockquote(crate::ast::Blockquote { content: vec![Block::Paragraph(crate::ast::Paragraph { content: content_text.trim().to_string(), span: node.span() })], span: node.span() }))
            }
            HorizontalRule => Some(Block::HorizontalRule(crate::ast::HorizontalRule { span: node.span() })),
            Table => {
                let text = source.get_text_in(node.span());
                let lines: Vec<&str> = text.lines().collect();
                if lines.is_empty() {
                    return None;
                }

                let parse_row = |line: &str| -> crate::ast::TableRow {
                    let cells = line
                        .split('|')
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| crate::ast::TableCell {
                            content: s.trim().to_string(),
                            span: node.span(), // 简化处理
                        })
                        .collect();
                    crate::ast::TableRow { cells, span: node.span() }
                };

                let header = parse_row(lines[0]);
                let mut rows = Vec::new();
                for line in lines.iter().skip(1) {
                    if line.contains("---") {
                        continue;
                    }
                    if line.trim().is_empty() {
                        continue;
                    }
                    rows.push(parse_row(line));
                }

                Some(Block::Table(crate::ast::Table { header, rows, span: node.span() }))
            }
            HtmlTag => {
                // TODO: 实现 HTML 构建
                None
            }
            _ => None,
        }
    }

    fn build_list_item(&self, node: RedNode<MarkdownLanguage>, source: &SourceText) -> crate::ast::ListItem {
        let mut content = Vec::new();
        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                if let Some(block) = self.build_block(child_node, source) {
                    content.push(block);
                }
            }
        }

        // 如果没有嵌套块，但有文本内容，将其包装为段落
        if content.is_empty() {
            let text = source.get_text_in(node.span()).to_string();
            if !text.trim().is_empty() {
                // 简单的清理：移除可能的列表标记前缀
                let display_text = if text.starts_with("- ") || text.starts_with("* ") {
                    text[2..].to_string()
                }
                else if text.len() > 3 && text.chars().next().unwrap().is_ascii_digit() && text.contains(". ") {
                    // 处理有序列表标记，如 "1. "
                    if let Some(pos) = text.find(". ") { text[pos + 2..].to_string() } else { text }
                }
                else {
                    text
                };

                content.push(crate::ast::Block::Paragraph(crate::ast::Paragraph { content: display_text.trim().to_string(), span: node.span() }));
            }
        }

        crate::ast::ListItem { content, is_task: false, is_checked: None, span: node.span() }
    }
}

impl<'config> Builder<MarkdownLanguage> for MarkdownBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<MarkdownLanguage>) -> oak_core::builder::BuildOutput<MarkdownLanguage> {
        let parser = MarkdownParser::new(self.config);
        let mut parse_session = oak_core::parser::session::ParseSession::<MarkdownLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_session);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
                    Ok(ast_root) => oak_core::OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        oak_core::OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(parse_error) => oak_core::OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
