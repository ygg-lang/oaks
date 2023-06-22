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
                Some(Block::Heading(crate::ast::Heading { level, content: source.get_text_in(node.span()).to_string(), span: node.span() }))
            }
            Paragraph => Some(Block::Paragraph(crate::ast::Paragraph { content: source.get_text_in(node.span()).to_string(), span: node.span() })),
            CodeBlock => Some(Block::CodeBlock(crate::ast::CodeBlock {
                language: None, // TODO: 提取语言标识符
                content: source.get_text_in(node.span()).to_string(),
                span: node.span(),
            })),
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
            Blockquote => Some(Block::Blockquote(crate::ast::Blockquote { content: node.children().filter_map(|child| if let RedTree::Node(child_node) = child { self.build_block(child_node, source) } else { None }).collect(), span: node.span() })),
            HorizontalRule => Some(Block::HorizontalRule(crate::ast::HorizontalRule { span: node.span() })),
            Table => {
                // TODO: 实现表格构建
                None
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

        crate::ast::ListItem {
            content,
            is_task: false,   // TODO: 检测任务列表
            is_checked: None, // TODO: 检测勾选状态
            span: node.span(),
        }
    }
}

impl<'config> Builder<MarkdownLanguage> for MarkdownBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<MarkdownLanguage>) -> oak_core::builder::BuildOutput<MarkdownLanguage> {
        let parser = MarkdownParser::new(self.config);
        let mut parse_session = oak_core::parser::session::ParseSession::<MarkdownLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_session);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
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
