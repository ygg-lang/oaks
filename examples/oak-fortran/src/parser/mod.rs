/// Element type definitions for Fortran parser.
pub mod element_type;

use crate::{language::FortranLanguage, parser::element_type::FortranElementType};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, FortranLanguage, S>;

/// Fortran parser implementation.
pub struct FortranParser<'config> {
    pub(crate) config: &'config FortranLanguage,
}

impl<'config> FortranParser<'config> {
    /// Create a new Fortran parser.
    pub fn new(config: &'config FortranLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<FortranLanguage> for FortranParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<FortranLanguage>) -> ParseOutput<'a, FortranLanguage> {
        let lexer = crate::lexer::FortranLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                state.advance()
            }

            Ok(state.finish_at(checkpoint, crate::parser::element_type::FortranElementType::Root))
        })
    }
}
