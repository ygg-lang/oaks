use crate::{language::PythonLanguage, lexer::PythonTokenType};
use oak_core::{Source, parser::ParserState};

/// Python parser state.
pub(crate) type State<'a, S> = ParserState<'a, PythonLanguage, S>;

impl<'config> super::PythonParser<'config> {
    pub(crate) fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(kind) = state.peek_kind() {
            if kind == PythonTokenType::Whitespace || kind == PythonTokenType::Comment || kind == PythonTokenType::Newline {
                state.bump();
            }
            else {
                break;
            }
        }
    }
}
