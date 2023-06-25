use crate::{
    ast::*,
    language::MarkdownLanguage,
    parser::{MarkdownParser, element_type::MarkdownElementType},
};
use oak_core::{Builder, BuilderCache, ElementType, GreenNode, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, UniversalElementRole, source::Source};

/// AST builder for the Markdown language.
#[derive(Clone)]
pub struct MarkdownBuilder<'config> {
    /// Language configuration.
    config: &'config MarkdownLanguage,
}

#[allow(unused)]
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
        let role = node.kind::<MarkdownElementType>().role();
        match role {
            UniversalElementRole::Container => {
                // Determine if it's a list, quote, or other container
                None
            }
            UniversalElementRole::Statement => {
                // Handle paragraphs, headings, etc.
                None
            }
            _ => None,
        }
    }

    /// Builds inline-level elements.
    fn build_inline(&self, node: RedNode<MarkdownLanguage>, source: &SourceText) -> Option<Inline> {
        None
    }

    fn build_list_item(&self, node: RedNode<MarkdownLanguage>, source: &SourceText) -> crate::ast::ListItem {
        let mut content = Vec::new();
        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                if let Some(block) = self.build_block(child_node, source) {
                    content.push(block)
                }
            }
        }

        // If no nested blocks but has text content, wrap it as a paragraph
        if content.is_empty() {
            let text = source.get_text_in(node.span()).to_string();
            if !text.trim().is_empty() {
                // Simple cleanup: remove possible list marker prefixes
                let display_text = if text.starts_with("- ") || text.starts_with("* ") {
                    text[2..].to_string()
                }
                else if text.len() > 3 && text.chars().next().unwrap().is_ascii_digit() && text.contains(". ") {
                    // Handle ordered list markers like "1. "
                    if let Some(pos) = text.find(". ") { text[pos + 2..].to_string() } else { text }
                }
                else {
                    text
                };

                content.push(crate::ast::Block::Paragraph(crate::ast::Paragraph { content: display_text.trim().to_string(), span: node.span() }))
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
