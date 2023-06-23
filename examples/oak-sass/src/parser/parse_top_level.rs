use crate::{
    language::SassLanguage,
    lexer::token_type::SassTokenType,
    parser::{SassParser, State, element_type::SassElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> SassParser<'config> {
    /// Internal method to parse the root of a Sass file.
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, SassLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance()
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::SassElementType::SourceFile))
    }
}
