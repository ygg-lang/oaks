use crate::parser::{ClojureElementType, ClojureParser, State};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> ClojureParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::language::ClojureLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, ClojureElementType::SourceFile))
    }
}
