/// CST element type definitions.
pub mod element_type;

use oak_core::{
    parser::{ParseCache, Parser, parse_with_lexer},
    source::{Source, TextEdit},
};

use crate::{language::MetisLanguage, lexer::MetisLexer};

/// Parser for Metis island language (grammar stub).
pub struct MetisParser;

impl Default for MetisParser {
    fn default() -> Self {
        Self
    }
}

impl Parser<MetisLanguage> for MetisParser {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<MetisLanguage>) -> oak_core::parser::ParseOutput<'a, MetisLanguage> {
        let lexer = MetisLexer::default();
        parse_with_lexer(&lexer, text, edits, cache, |_st| Err(oak_core::errors::OakError::custom_error("Metis parser not implemented yet")))
    }
}

pub use element_type::MetisElementType;
