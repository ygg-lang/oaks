use crate::{
    language::DotLanguage,
    lexer::token_type::DotTokenType,
    parser::{DotParser, State, element_type::DotElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> DotParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DotLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.bump();
        }

        Ok(state.finish_at(checkpoint, DotElementType::Root))
    }
}
