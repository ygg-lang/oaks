use crate::{
    kind::MsilSyntaxKind,
    language::MsilLanguage,
    parser::{MsilParser, State},
};
use oak_core::{GreenNode, OakError};

impl<'config> MsilParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, MsilLanguage>, OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() {
            state.bump();
        }
        Ok(state.finish_at(cp, MsilSyntaxKind::Root))
    }
}
