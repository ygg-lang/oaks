use crate::{
    language::JavadocLanguage,
    lexer::token_type::JavadocTokenType,
    parser::{JavadocParser, State, element_type::JavadocElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> JavadocParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, JavadocLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::JavadocElementType::Root))
    }
}
