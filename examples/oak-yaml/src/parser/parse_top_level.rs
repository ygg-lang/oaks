use crate::{
    language::YamlLanguage,
    parser::{State, YamlParser},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> YamlParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, YamlLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::YamlElementType::Root))
    }
}
