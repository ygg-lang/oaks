pub use element_type::ValkyrieElementType;

use crate::{ValkyrieLanguage, ValkyrieLexer};
use oak_core::{Parser, Source, TextEdit, parser::ParseCache};

/// Valkyrie parser.
pub struct ValkyrieParser<'config> {
    config: &'config ValkyrieLanguage,
}

impl<'config> Parser<ValkyrieLanguage> for ValkyrieParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ValkyrieLanguage>) -> oak_core::parser::ParseOutput<'a, ValkyrieLanguage> {
        oak_core::parser::parse_with_lexer(&ValkyrieLexer::new(self.config), text, edits, cache, |state| {
            let checkpoint = state.sink.checkpoint();
            while state.not_at_end() {
                state.advance();
            }
            let root = state.sink.finish_node(checkpoint, ValkyrieElementType::Root);
            Ok(root)
        })
    }
}

impl<'config> ValkyrieParser<'config> {
    /// Create a new Valkyrie parser.
    pub fn new(config: &'config ValkyrieLanguage) -> Self {
        Self { config }
    }
}

/// Element type definitions for the Valkyrie parser.
pub mod element_type;

/// String segment parser for interpolation and escape handling.
pub mod parse_string_segments;

pub use parse_string_segments::parse_string_segments;
