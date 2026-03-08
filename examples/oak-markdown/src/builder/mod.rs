use crate::{
    ast::*,
    language::MarkdownLanguage,
    parser::{MarkdownParser, element_type::MarkdownElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// AST builder for the Markdown language.
#[derive(Clone)]
pub struct MarkdownBuilder<'config> {
    /// Language configuration.
    config: &'config MarkdownLanguage,
}

impl<'config> MarkdownBuilder<'config> {
    /// Creates a new MarkdownBuilder with the given configuration.
    pub fn new(config: &'config MarkdownLanguage) -> Self {
        Self { config }
    }

    /// Builds the AST root node from the green tree.
    fn build_root(&self, green_tree: &GreenNode<MarkdownLanguage>, source: &SourceText) -> Result<MarkdownRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);

        let mut blocks = Vec::new();
        for child in red_root.children() {
            if let RedTree::Node(node) = child {
                if let Some(block) = self.build_block(node, source) {
                    blocks.push(block)
                }
            }
        }

        Ok(MarkdownRoot { blocks })
    }

    /// Builds block-level elements.
    fn build_block(&self, node: RedNode<MarkdownLanguage>, source: &SourceText) -> Option<Block> {
        let kind = node.element_type();

        match kind {
            MarkdownElementType::Heading1 | MarkdownElementType::Heading2 | MarkdownElementType::Heading3 | MarkdownElementType::Heading4 | MarkdownElementType::Heading5 | MarkdownElementType::Heading6 => {
                let level = match kind {
                    MarkdownElementType::Heading1 => 1,
                    MarkdownElementType::Heading2 => 2,
                    MarkdownElementType::Heading3 => 3,
                    MarkdownElementType::Heading4 => 4,
                    MarkdownElementType::Heading5 => 5,
                    MarkdownElementType::Heading6 => 6,
                    _ => 1,
                };
                let content = self.collect_text(node, source);
                Some(Block::Heading(Heading { level, content, span: node.span() }))
            }
            MarkdownElementType::Paragraph => {
                let content = self.collect_text(node, source);
                Some(Block::Paragraph(Paragraph { content, span: node.span() }))
            }
            MarkdownElementType::CodeBlock => {
                let (language, content) = self.extract_code_block(node, source);
                Some(Block::CodeBlock(CodeBlock { language, content, span: node.span() }))
            }
            MarkdownElementType::List => {
                let (is_ordered, items) = self.extract_list(node, source);
                Some(Block::List(List { is_ordered, items, span: node.span() }))
            }
            MarkdownElementType::Blockquote => {
                let content = self.extract_blockquote_content(node, source);
                Some(Block::Blockquote(Blockquote { content, span: node.span() }))
            }
            MarkdownElementType::HorizontalRule => Some(Block::HorizontalRule(HorizontalRule { span: node.span() })),
            MarkdownElementType::Table => {
                let (header, rows) = self.extract_table(node, source);
                Some(Block::Table(Table { header, rows, span: node.span() }))
            }
            _ => None,
        }
    }

    /// Collects text content from a node and its children.
    fn collect_text(&self, node: RedNode<MarkdownLanguage>, source: &SourceText) -> String {
        let mut text = String::new();
        for child in node.children() {
            match child {
                RedTree::Node(child_node) => {
                    text.push_str(&self.collect_text(child_node, source));
                }
                RedTree::Leaf(_) => {
                    text.push_str(&child.text(source));
                }
            }
        }
        text
    }

    /// Extracts code block content and language.
    fn extract_code_block(&self, node: RedNode<MarkdownLanguage>, source: &SourceText) -> (Option<String>, String) {
        let mut language = None;
        let mut content = String::new();
        let mut in_content = false;

        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                let kind = child_node.element_type();
                match kind {
                    MarkdownElementType::CodeLanguage => {
                        language = Some(self.collect_text(child_node, source).trim().to_string());
                    }
                    MarkdownElementType::Text | MarkdownElementType::Whitespace | MarkdownElementType::Newline => {
                        if in_content {
                            content.push_str(&child_node.text(source));
                        }
                    }
                    MarkdownElementType::CodeFence => {
                        in_content = !in_content;
                    }
                    _ => {}
                }
            }
        }

        (language.filter(|s| !s.is_empty()), content.trim().to_string())
    }

    /// Extracts list items.
    fn extract_list(&self, node: RedNode<MarkdownLanguage>, source: &SourceText) -> (bool, Vec<ListItem>) {
        let mut is_ordered = false;
        let mut items = Vec::new();

        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                let kind = child_node.element_type();
                if kind == MarkdownElementType::ListItem {
                    let list_item = self.build_list_item(child_node, source);

                    if items.is_empty() {
                        let text = child_node.text(source);
                        is_ordered = text.trim_start().chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false);
                    }

                    items.push(list_item);
                }
            }
        }

        (is_ordered, items)
    }

    /// Extracts blockquote content.
    fn extract_blockquote_content(&self, node: RedNode<MarkdownLanguage>, source: &SourceText) -> Vec<Block> {
        let mut content = Vec::new();

        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                if let Some(block) = self.build_block(child_node, source) {
                    content.push(block);
                }
            }
        }

        if content.is_empty() {
            let text = self.collect_text(node, source);
            if !text.trim().is_empty() {
                content.push(Block::Paragraph(Paragraph { content: text.trim().to_string(), span: node.span() }));
            }
        }

        content
    }

    /// Extracts table content.
    fn extract_table(&self, node: RedNode<MarkdownLanguage>, source: &SourceText) -> (TableRow, Vec<TableRow>) {
        let mut header = TableRow { cells: Vec::new(), span: node.span() };
        let mut rows = Vec::new();
        let mut is_header = true;

        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                let kind = child_node.element_type();
                match kind {
                    MarkdownElementType::TableRow => {
                        let cells = self.extract_table_cells(child_node, source);
                        let row = TableRow { cells, span: child_node.span() };
                        if is_header {
                            header = row;
                            is_header = false;
                        }
                        else {
                            rows.push(row);
                        }
                    }
                    MarkdownElementType::TableSeparator => {}
                    _ => {}
                }
            }
        }

        (header, rows)
    }

    /// Extracts table cells from a row.
    fn extract_table_cells(&self, node: RedNode<MarkdownLanguage>, source: &SourceText) -> Vec<TableCell> {
        let mut cells = Vec::new();

        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                let kind = child_node.element_type();
                if kind == MarkdownElementType::TableCell {
                    let content = self.collect_text(child_node, source);
                    cells.push(TableCell { content: content.trim().to_string(), span: child_node.span() });
                }
            }
        }

        cells
    }

    /// Builds inline-level elements.
    fn build_inline(&self, node: RedNode<MarkdownLanguage>, source: &SourceText) -> Option<Inline> {
        let kind = node.element_type();
        let text = self.collect_text(node, source);

        match kind {
            MarkdownElementType::Text => Some(Inline::Text(text)),
            MarkdownElementType::Emphasis => Some(Inline::Italic(text)),
            MarkdownElementType::Strong => Some(Inline::Bold(text)),
            MarkdownElementType::Strikethrough => Some(Inline::Text(text)),
            MarkdownElementType::InlineCode => Some(Inline::Code(text)),
            MarkdownElementType::Link => Some(Inline::Link { text: text, url: String::new(), title: None }),
            MarkdownElementType::Image => Some(Inline::Image { alt: text, url: String::new(), title: None }),
            _ => None,
        }
    }

    fn build_list_item(&self, node: RedNode<MarkdownLanguage>, source: &SourceText) -> crate::ast::ListItem {
        let mut content = Vec::new();
        let mut is_task = false;
        let mut is_checked = None;

        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                let kind = child_node.element_type();
                if kind == MarkdownElementType::TaskMarker {
                    is_task = true;
                    let marker_text = child_node.text(source);
                    is_checked = Some(marker_text.contains('x') || marker_text.contains('X'));
                }
                else if let Some(block) = self.build_block(child_node, source) {
                    content.push(block);
                }
            }
        }

        if content.is_empty() {
            let text = node.text(source).to_string();
            if !text.trim().is_empty() {
                let display_text = if text.starts_with("- ") || text.starts_with("* ") {
                    text[2..].to_string()
                }
                else if text.len() > 3 && text.chars().next().unwrap().is_ascii_digit() && text.contains(". ") {
                    if let Some(pos) = text.find(". ") { text[pos + 2..].to_string() } else { text }
                }
                else {
                    text
                };

                content.push(crate::ast::Block::Paragraph(crate::ast::Paragraph { content: display_text.trim().to_string(), span: node.span() }))
            }
        }

        crate::ast::ListItem { content, is_task, is_checked, span: node.span() }
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
