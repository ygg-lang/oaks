use crate::{
    language::ValaLanguage,
    lexer::token_type::ValaTokenType,
    parser::{State, ValaParser, element_type::ValaElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> ValaParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValaLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, ValaElementType::SourceFile))
    }
}
