use crate::{
    language::JasminLanguage,
    lexer::token_type::JasminTokenType,
    parser::{JasminParser, State, element_type::JasminElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> JasminParser<'config> {
    pub(crate) fn parse_top_level<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        if state.not_at_end() {
            state.bump();
        }
        Ok(())
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(kind) = state.peek_kind() {
            if matches!(kind, JasminTokenType::Whitespace | JasminTokenType::Newline | JasminTokenType::Comment) {
                state.bump();
            }
            else {
                break;
            }
        }
    }
}
