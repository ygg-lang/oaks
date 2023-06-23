use crate::{
    language::ElmLanguage,
    lexer::token_type::ElmTokenType,
    parser::{ElmParser, State, element_type::ElmElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> ElmParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ElmLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::ElmElementType::Root))
    }
}
