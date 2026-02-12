use crate::{language::PythonLanguage, lexer::PythonTokenType, parser::PythonElementType as ET};
use oak_core::{Source, parser::ParserState};

/// Python parser state.
pub(crate) type State<'a, S> = ParserState<'a, PythonLanguage, S>;

impl<'config> super::PythonParser<'config> {
    pub(crate) fn parse_comprehension<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        use PythonTokenType::*;
        let cp = state.checkpoint();

        self.skip_trivia(state);
        if !state.eat(ForKeyword) {
            return;
        }

        // target
        self.parse_expression(state, 0);

        self.skip_trivia(state);
        state.expect(InKeyword).ok();

        // iter
        self.parse_expression(state, 0);

        // nested for/if
        loop {
            self.skip_trivia(state);
            let is_async = state.eat(AsyncKeyword);
            if is_async {
                self.skip_trivia(state);
            }

            if state.at(ForKeyword) {
                state.bump();
                self.parse_expression(state, 0);
                self.skip_trivia(state);
                state.expect(InKeyword).ok();
                self.parse_expression(state, 0);
            }
            else if state.at(IfKeyword) {
                if is_async {
                    // async if is not valid in Python
                }
                state.bump();
                self.parse_expression(state, 0);
            }
            else {
                break;
            }
        }

        state.finish_at(cp, ET::Comprehension);
    }
}
