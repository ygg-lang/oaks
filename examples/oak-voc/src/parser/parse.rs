use crate::{
    kind::VocSyntaxKind,
    language::VocLanguage,
    parser::{State, VocParser},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> VocParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, VocLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, VocSyntaxKind::SourceFile))
    }
}
