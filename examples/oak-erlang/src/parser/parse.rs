use crate::{
    kind::ErlangSyntaxKind,
    language::ErlangLanguage,
    parser::{ErlangParser, State},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> ErlangParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ErlangLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            state.bump();
        }

        Ok(state.finish_at(checkpoint, ErlangSyntaxKind::Root))
    }
}
