use crate::{
    language::JasmLanguage,
    lexer::token_type::JasmTokenType,
    parser::{JasmParser, State, element_type::JasmElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> JasmParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, JasmLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance()
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::JasmElementType::Root))
    }
}
