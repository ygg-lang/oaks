use crate::{
    kind::LeanSyntaxKind,
    parser::{LeanParser, State},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> LeanParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::language::LeanLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        let root = state.finish_at(checkpoint, LeanSyntaxKind::Root.into());
        Ok(root)
    }
}
