use crate::{
    language::VomlLanguage,
    lexer::token_type::VomlTokenType,
    parser::{State, VomlParser, element_type::VomlElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> VomlParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, VomlLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, VomlElementType::SourceFile))
    }
}
