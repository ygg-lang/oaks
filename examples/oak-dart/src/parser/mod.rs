/// Element types for the Dart language.
pub mod element_type;

use crate::{language::DartLanguage, lexer::DartLexer, parser::element_type::DartElementType};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, DartLanguage, S>;

/// A parser for the Dart language.
pub struct DartParser<'a> {
    pub(crate) _language: &'a DartLanguage,
}

impl<'a> DartParser<'a> {
    /// Creates a new DartParser with the given language configuration.
    pub fn new(language: &'a DartLanguage) -> Self {
        Self { _language: language }
    }
}

impl<'p> Parser<DartLanguage> for DartParser<'p> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<DartLanguage>) -> ParseOutput<'a, DartLanguage> {
        let lexer = DartLexer::new(self._language);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();

            while state.not_at_end() {
                state.bump();
            }

            Ok(state.finish_at(cp, DartElementType::Root))
        })
    }
}
