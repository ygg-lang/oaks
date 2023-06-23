use crate::{
    language::SolidityLanguage,
    lexer::token_type::SolidityTokenType,
    parser::{SolidityParser, State, element_type::SolidityElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> SolidityParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, SolidityLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::SolidityElementType::SourceFile))
    }
}
