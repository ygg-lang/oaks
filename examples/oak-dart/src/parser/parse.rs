use crate::{
    kind::DartSyntaxKind,
    language::DartLanguage,
    parser::{DartParser, State},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> DartParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DartLanguage>, OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() {
            state.bump();
        }

        Ok(state.finish_at(cp, DartSyntaxKind::Root))
    }
}
