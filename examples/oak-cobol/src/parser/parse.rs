use crate::parser::{CobolElementType, CobolParser, State};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> CobolParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::language::CobolLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, CobolElementType::SourceFile))
    }
}
