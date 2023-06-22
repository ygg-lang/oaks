use crate::{
    kind::TwigSyntaxKind,
    language::TwigLanguage,
    parser::{State, TwigParser},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> TwigParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, TwigLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            state.bump();
        }
        Ok(state.finish_at(checkpoint, TwigSyntaxKind::Root))
    }
}
