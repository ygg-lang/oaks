use crate::{
    language::ErlangLanguage,
    lexer::token_type::ErlangTokenType,
    parser::{ErlangParser, State, element_type::ErlangElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> ErlangParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ErlangLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            state.bump()
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::ErlangElementType::Root))
    }
}
