use crate::{
    language::DelphiLanguage,
    lexer::token_type::DelphiTokenType,
    parser::{DelphiParser, State, element_type::DelphiElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> DelphiParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DelphiLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance()
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::DelphiElementType::Program))
    }
}
