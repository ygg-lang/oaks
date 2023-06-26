use crate::{ast::*, language::VLangLanguage, lexer::token_type::VLangTokenType, parser::element_type::VLangElementType};
use core::range::Range;
use oak_core::{
    GreenNode, Parser, Source, TokenType,
    builder::{BuildOutput, Builder},
    source::TextEdit,
    tree::red_tree::{RedNode, RedTree},
};

/// V language builder
pub struct VLangBuilder<'config> {
    language: &'config VLangLanguage,
}

impl<'config> VLangBuilder<'config> {
    /// Creates a new VLangBuilder
    pub fn new(language: &'config VLangLanguage) -> Self {
        Self { language }
    }

    fn build_root(&self, green: &GreenNode<VLangLanguage>, source: &str) -> VRoot {
        let red = RedNode::new(green, 0);
        let mut module_name = String::new();
        let mut imports = Vec::new();
        let mut items = Vec::new();

        // Simple implementation to build the root node
        // This will be expanded as the parser becomes more sophisticated
        for child in red.children() {
            if let RedTree::Node(node) = child {
                // For now, just collect basic information
                // In a real implementation, this would parse the actual structure
                items.push(self.build_item(node, source));
            }
        }

        VRoot { module_name, imports, items }
    }

    fn build_item(&self, node: RedNode<VLangLanguage>, source: &str) -> VItem {
        // Simple implementation - in a real implementation, this would parse the actual structure
        VItem::Function(VFunction { name: "unknown".to_string(), is_pub: false, receiver: None, params: Vec::new(), return_type: None, body: Vec::new() })
    }

    fn get_text<'a>(&self, span: Range<usize>, source: &'a str) -> &'a str {
        let start = span.start;
        let end = span.end;
        if start > source.len() || end > source.len() || start > end {
            return "";
        }
        &source[start..end]
    }
}

impl<'config> Builder<VLangLanguage> for VLangBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::parser::ParseCache<VLangLanguage>) -> BuildOutput<VLangLanguage> {
        let parser = crate::parser::VLangParser::new(self.language);
        let output = parser.parse(text, edits, cache);

        let source_str = text.get_text_in(Range { start: 0, end: text.length() });
        let result = output.result.map(|green| self.build_root(green, &source_str));

        oak_core::errors::OakDiagnostics { result, diagnostics: output.diagnostics }
    }
}
