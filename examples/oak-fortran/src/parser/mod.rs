use crate::{kind::FortranSyntaxKind, language::FortranLanguage};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, FortranLanguage, S>;

pub struct FortranParser<'config> {
    pub(crate) _config: &'config FortranLanguage,
}

impl<'config> FortranParser<'config> {
    pub fn new(config: &'config FortranLanguage) -> Self {
        Self { _config: config }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, FortranLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, FortranSyntaxKind::Root.into()))
    }
}

impl<'config> Parser<FortranLanguage> for FortranParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<FortranLanguage>) -> ParseOutput<'a, FortranLanguage> {
        let lexer = crate::lexer::FortranLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
