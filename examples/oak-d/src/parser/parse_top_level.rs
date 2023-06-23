use crate::{
    language::DLanguage,
    lexer::token_type::DTokenType,
    parser::{DParser, State, element_type::DElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> DParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::DElementType::Root))
    }
}
