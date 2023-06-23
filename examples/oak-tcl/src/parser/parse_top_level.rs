use crate::{
    language::TclLanguage,
    lexer::token_type::TclTokenType,
    parser::{State, TclParser, element_type::TclElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> TclParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, TclLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            if state.at(TclTokenType::Newline) || state.at(TclTokenType::Semicolon) {
                state.bump();
                continue;
            }

            if state.at(TclTokenType::Whitespace) {
                state.bump();
                continue;
            }

            if state.at(TclTokenType::Comment) {
                state.bump();
                continue;
            }

            self.parse_command(state)?
        }

        Ok(state.finish_at(checkpoint, TclElementType::Root))
    }

    fn parse_command<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() && !state.at(TclTokenType::Newline) && !state.at(TclTokenType::Semicolon) {
            if state.at(TclTokenType::Whitespace) {
                state.bump();
                continue;
            }

            if state.at(TclTokenType::Comment) {
                break;
            }

            self.parse_word(state)?;
        }

        state.finish_at(checkpoint, TclElementType::Command);
        Ok(())
    }

    fn parse_word<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        if state.at(TclTokenType::Dollar) {
            state.bump(); // $
            state.expect(TclTokenType::Identifier)?;
            state.finish_at(checkpoint, TclElementType::VariableWord);
        }
        else if state.at(TclTokenType::LeftBracket) {
            state.bump(); // [
            // Internal script
            while state.not_at_end() && !state.at(TclTokenType::RightBracket) {
                if state.at(TclTokenType::Newline) || state.at(TclTokenType::Semicolon) || state.at(TclTokenType::Whitespace) {
                    state.bump();
                    continue;
                }
                self.parse_command(state)?;
            }
            state.expect(TclTokenType::RightBracket)?;
            state.finish_at(checkpoint, TclElementType::ScriptWord);
        }
        else {
            // Simple word or StringLiteral or BracedString (all currently tokens)
            // If it's a StringLiteral, it might be braced or quoted.
            let kind = state.current().map(|t| t.kind);
            state.bump();

            let node_kind = match kind {
                Some(TclTokenType::StringLiteral) => TclTokenType::SimpleWord,
                _ => TclTokenType::SimpleWord,
            };
            state.finish_at(checkpoint, TclElementType::from(node_kind));
        }

        Ok(())
    }
}
