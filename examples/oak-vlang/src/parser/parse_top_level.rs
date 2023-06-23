use crate::{
    language::VLangLanguage,
    lexer::token_type::VLangTokenType,
    parser::{State, VLangParser, element_type::VLangElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> VLangParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, VLangLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, VLangElementType::SourceFile))
    }
}
