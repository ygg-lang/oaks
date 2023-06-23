use crate::{
    language::TypstLanguage,
    lexer::token_type::TypstTokenType,
    parser::{State, TypstParser, element_type::TypstElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> TypstParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, TypstLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_item(state)?
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::TypstElementType::Root))
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(TypstTokenType::Equal) => {
                let checkpoint = state.checkpoint();
                state.bump(); // =
                while state.not_at_end() && state.peek_kind() != Some(TypstTokenType::Newline) {
                    state.bump()
                }
                state.finish_at(checkpoint, crate::parser::element_type::TypstElementType::Heading);
            }
            Some(TypstTokenType::Hash) => {
                // Very basic support for #quote[...]
                let checkpoint = state.checkpoint();
                state.bump(); // #
                // Check if it's "quote" or other commands
                while state.not_at_end()
                    && state
                        .peek_kind()
                        .map(|k| {
                            matches!(
                                k,
                                TypstTokenType::Identifier
                                    | TypstTokenType::Let
                                    | TypstTokenType::If
                                    | TypstTokenType::Else
                                    | TypstTokenType::For
                                    | TypstTokenType::While
                                    | TypstTokenType::Set
                                    | TypstTokenType::Show
                                    | TypstTokenType::Import
                                    | TypstTokenType::Include
                            )
                        })
                        .unwrap_or(false)
                {
                    state.bump()
                }

                if state.peek_kind() == Some(TypstTokenType::LeftBracket) {
                    state.bump(); // [
                    let mut depth = 1;
                    while state.not_at_end() && depth > 0 {
                        if state.peek_kind() == Some(TypstTokenType::LeftBracket) {
                            depth += 1
                        }
                        else if state.peek_kind() == Some(TypstTokenType::RightBracket) {
                            depth -= 1
                        }
                        state.bump()
                    }
                }
                else {
                    // Just a simple #cmd without arguments
                    while state.not_at_end() && state.peek_kind() != Some(TypstTokenType::Newline) && state.peek_kind() != Some(TypstTokenType::Whitespace) {
                        state.bump()
                    }
                }
                state.finish_at(checkpoint, crate::parser::element_type::TypstElementType::Quote);
            }
            Some(TypstTokenType::Dollar) => {
                let checkpoint = state.checkpoint();
                state.bump(); // $
                while state.not_at_end() && state.peek_kind() != Some(TypstTokenType::Dollar) {
                    state.bump()
                }
                if state.peek_kind() == Some(TypstTokenType::Dollar) {
                    state.bump()
                }
                state.finish_at(checkpoint, crate::parser::element_type::TypstElementType::Math);
            }
            Some(TypstTokenType::Star) => {
                let checkpoint = state.checkpoint();
                state.bump(); // *
                while state.not_at_end() && state.peek_kind() != Some(TypstTokenType::Star) {
                    state.bump()
                }
                if state.peek_kind() == Some(TypstTokenType::Star) {
                    state.bump()
                }
                state.finish_at(checkpoint, crate::parser::element_type::TypstElementType::Strong);
            }
            Some(TypstTokenType::Underscore) => {
                let checkpoint = state.checkpoint();
                state.bump(); // _
                while state.not_at_end() && state.peek_kind() != Some(TypstTokenType::Underscore) {
                    state.bump()
                }
                if state.peek_kind() == Some(TypstTokenType::Underscore) {
                    state.bump()
                }
                state.finish_at(checkpoint, crate::parser::element_type::TypstElementType::Emphasis);
            }
            Some(TypstTokenType::Minus) => {
                let checkpoint = state.checkpoint();
                state.bump(); // -
                while state.not_at_end() && state.peek_kind() != Some(TypstTokenType::Newline) {
                    state.bump()
                }
                state.finish_at(checkpoint, crate::parser::element_type::TypstElementType::ListItem);
            }
            Some(TypstTokenType::Plus) => {
                let checkpoint = state.checkpoint();
                state.bump(); // +
                while state.not_at_end() && state.peek_kind() != Some(TypstTokenType::Newline) {
                    state.bump()
                }
                state.finish_at(checkpoint, crate::parser::element_type::TypstElementType::EnumItem);
            }
            Some(TypstTokenType::Backtick) => {
                let checkpoint = state.checkpoint();
                state.bump(); // `
                while state.not_at_end() && state.peek_kind() != Some(TypstTokenType::Backtick) {
                    state.bump()
                }
                if state.peek_kind() == Some(TypstTokenType::Backtick) {
                    state.bump()
                }
                state.finish_at(checkpoint, crate::parser::element_type::TypstElementType::Raw);
            }
            _ => state.bump(),
        };
        Ok(())
    }
}
