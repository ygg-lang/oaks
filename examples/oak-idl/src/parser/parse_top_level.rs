use crate::{
    language::IdlLanguage,
    lexer::token_type::IdlTokenType,
    parser::{IdlParser, State, element_type::IdlElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> IdlParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, IdlLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.bump();
        }

        Ok(state.finish_at(checkpoint, IdlElementType::Module))
    }
}
