use crate::{
    kind::MarkdownSyntaxKind,
    parser::{MarkdownParser, State},
};
use oak_core::{Arc, GreenNode, OakError, source::Source};

impl<'config> MarkdownParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<Arc<GreenNode<MarkdownSyntaxKind>>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        state.finish_at(checkpoint, MarkdownSyntaxKind::Root);
        Ok(state.builder.last_node().expect("Failed to build Root node"))
    }
}
