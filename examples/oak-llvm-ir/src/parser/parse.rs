use crate::{kind::LLvmSyntaxKind, language::LLvmLanguage, parser::LlirParser};
use oak_core::{GreenNode, OakError, parser::ParserState};

impl<'config> LlirParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, LLvmLanguage, S>) -> Result<&'a GreenNode<'a, LLvmLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, LLvmSyntaxKind::Root))
    }
}
