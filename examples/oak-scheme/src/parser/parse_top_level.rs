use crate::{
    language::SchemeLanguage,
    lexer::token_type::SchemeTokenType,
    parser::{SchemeParser, State, element_type::SchemeElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> SchemeParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, SchemeLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance()
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::SchemeElementType::SourceFile))
    }
}
