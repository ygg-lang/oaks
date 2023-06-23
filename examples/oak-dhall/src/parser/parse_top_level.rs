use crate::{
    language::DHallLanguage,
    lexer::token_type::DHallTokenType,
    parser::{DHallParser, State, element_type::DHallElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> DHallParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DHallLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::DHallElementType::Root))
    }
}
