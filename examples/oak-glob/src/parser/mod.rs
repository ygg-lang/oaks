pub mod element_type;

use oak_core::{
    parser::{ParseCache, Parser, parse_with_lexer},
    source::{Source, TextEdit},
};

use crate::{language::GlobLanguage, lexer::GlobLexer};

/// Parser for glob pattern syntax.
pub struct GlobParser;

impl Parser<GlobLanguage> for GlobParser {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<GlobLanguage>) -> oak_core::parser::ParseOutput<'a, GlobLanguage> {
        let lexer = GlobLexer::default();
        parse_with_lexer(&lexer, text, edits, cache, |st| {
            // For now, just return an error since we don't have a proper parser implementation
            Err(oak_core::errors::OakError::custom_error("Not implemented"))
        })
    }
}

impl Default for GlobParser {
    fn default() -> Self {
        Self
    }
}

pub use element_type::GlobElementType;
