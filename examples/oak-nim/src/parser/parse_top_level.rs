use crate::{
    language::NimLanguage,
    lexer::token_type::NimTokenType,
    parser::{NimParser, State, element_type::NimElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> NimParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, NimLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            state.advance()
        }
        Ok(state.finish_at(checkpoint, crate::parser::element_type::NimElementType::Root))
    }
}
