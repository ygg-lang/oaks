use crate::{
    language::ScssLanguage,
    parser::{ScssParser, State, element_type::ScssElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> ScssParser<'config> {
    /// Internal method to parse the root of an SCSS file.
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ScssLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance()
        }

        Ok(state.finish_at(checkpoint, ScssElementType::SourceFile))
    }
}
