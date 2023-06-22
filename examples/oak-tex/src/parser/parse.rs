use crate::{
    kind::TexSyntaxKind,
    language::TexLanguage,
    parser::{State, TexParser},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> TexParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, TexLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, TexSyntaxKind::Root))
    }
}
