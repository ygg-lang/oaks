pub mod element_type;

use crate::{language::RakuLanguage, lexer::RakuLexer, parser::element_type::RakuElementType};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

/// Parser for Raku.
pub struct RakuParser {
    _language: RakuLanguage,
}

impl RakuParser {
    /// Creates a new `RakuParser`.
    pub fn new(language: RakuLanguage) -> Self {
        Self { _language: language }
    }

    fn parse_root<'a, S: Source + ?Sized>(&self, state: &mut ParserState<'a, RakuLanguage, S>) -> Result<&'a GreenNode<'a, RakuLanguage>, OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() {
            state.bump();
        }
        Ok(state.finish_at(cp, RakuElementType::Root))
    }
}

impl Parser<RakuLanguage> for RakuParser {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<RakuLanguage>) -> ParseOutput<'a, RakuLanguage> {
        let lexer = RakuLexer::new();
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root(state))
    }
}
