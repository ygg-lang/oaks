/// Delphi element type definitions.
pub mod element_type;

use crate::language::DelphiLanguage;
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type _State<'a, S> = oak_core::parser::ParserState<'a, DelphiLanguage, S>;

/// Delphi parser implementation.
pub struct DelphiParser<'config> {
    pub(crate) config: &'config DelphiLanguage,
}

impl<'config> DelphiParser<'config> {
    /// Creates a new `DelphiParser`.
    pub fn new(config: &'config DelphiLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<DelphiLanguage> for DelphiParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<DelphiLanguage>) -> ParseOutput<'a, DelphiLanguage> {
        let lexer = crate::lexer::DelphiLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                state.advance()
            }

            Ok(state.finish_at(checkpoint, crate::parser::element_type::DelphiElementType::Program))
        })
    }
}
