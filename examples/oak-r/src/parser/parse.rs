use crate::{
    kind::RSyntaxKind,
    language::RLanguage,
    parser::{RParser, State},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> RParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, RLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            state.bump();
        }
        Ok(state.finish_at(checkpoint, RSyntaxKind::Root))
    }
}
