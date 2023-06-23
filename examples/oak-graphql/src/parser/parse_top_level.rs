use crate::{
    language::GraphQLLanguage,
    parser::{GraphQLParser, State, element_type::GraphQLElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> GraphQLParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, GraphQLLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance()
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::GraphQLElementType::SourceFile))
    }
}
