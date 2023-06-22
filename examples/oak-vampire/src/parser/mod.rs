use crate::{kind::VampireSyntaxKind, language::VampireLanguage, lexer::VampireLexer};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, VampireLanguage, S>;

pub struct VampireParser<'config> {
    pub(crate) config: &'config VampireLanguage,
}

impl<'config> VampireParser<'config> {
    pub fn new(config: &'config VampireLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, VampireLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.bump();
        }

        let root = state.finish_at(checkpoint, VampireSyntaxKind::Root.into());
        Ok(root)
    }
}

impl<'config> Parser<VampireLanguage> for VampireParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VampireLanguage>) -> ParseOutput<'a, VampireLanguage> {
        let lexer = VampireLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
