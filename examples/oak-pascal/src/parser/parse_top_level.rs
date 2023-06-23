use crate::{
    language::PascalLanguage,
    lexer::token_type::PascalTokenType,
    parser::{PascalParser, State, element_type::PascalElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl PascalParser {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, PascalLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            state.bump();
        }
        Ok(state.finish_at(checkpoint, PascalElementType::Root))
    }
}
