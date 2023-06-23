use crate::{
    language::DjangoLanguage,
    lexer::token_type::DjangoTokenType,
    parser::{DjangoParser, State, element_type::DjangoElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> DjangoParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DjangoLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            state.advance()
        }
        Ok(state.finish_at(checkpoint, crate::parser::element_type::DjangoElementType::Root))
    }
}
