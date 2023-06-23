use crate::{
    language::PurescriptLanguage,
    lexer::token_type::PurescriptTokenType,
    parser::{PurescriptParser, State, element_type::PurescriptElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> PurescriptParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, PurescriptLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::PurescriptElementType::SourceFile))
    }
}
