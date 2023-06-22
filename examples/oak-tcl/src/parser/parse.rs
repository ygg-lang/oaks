use crate::{
    kind::TclSyntaxKind,
    language::TclLanguage,
    parser::{State, TclParser},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> TclParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, TclLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, TclSyntaxKind::Root))
    }
}
