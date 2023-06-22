use crate::{
    language::JasmLanguage,
    parser::{JasmParser, State},
    syntax::JasmSyntaxKind,
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> JasmParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, JasmLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, JasmSyntaxKind::Root))
    }
}
