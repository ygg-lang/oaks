use crate::{
    language::GsglLanguage,
    parser::{GsglParser, State},
    syntax::GsglSyntaxKind,
};
use oak_core::{GreenNode, OakError, source::Source};

impl GsglParser {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, GsglLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, GsglSyntaxKind::Root))
    }
}
