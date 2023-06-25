//! Voml AST builder.

use crate::{
    ast::VRoot,
    language::VomlLanguage,
    lexer::token_type::VomlTokenType,
    parser::{VomlParser, element_type::VomlElementType},
};
use oak_core::{
    Builder, GreenNode, Parser, RedNode,
    builder::BuildOutput,
    parser::{ParseCache, session::ParseSession},
    source::Source,
};

/// A builder for creating a Voml AST.
pub struct VomlBuilder<'config> {
    config: &'config VomlLanguage,
}

impl<'config> VomlBuilder<'config> {
    /// Creates a new `VomlBuilder` with the given configuration.
    pub fn new(config: &'config VomlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<VomlLanguage> for VomlBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl ParseCache<VomlLanguage>) -> BuildOutput<VomlLanguage> {
        let parser = VomlParser::new(self.config);
        let parse_output = parser.parse(source, edits, cache);

        let result = match parse_output.result {
            Ok(green_tree) => {
                let red_tree = RedNode::new(green_tree, 0);
                Ok(self.build_root(&red_tree))
            }
            Err(e) => Err(e),
        };

        BuildOutput::<VomlLanguage> { result, diagnostics: parse_output.diagnostics }
    }
}

impl<'config> VomlBuilder<'config> {
    fn build_root<'a>(&self, _root: &RedNode<'a, VomlLanguage>) -> VRoot {
        // Current parser is a stub, so we return a default VRoot
        VRoot { module_name: String::new(), imports: Vec::new(), items: Vec::new() }
    }
}
