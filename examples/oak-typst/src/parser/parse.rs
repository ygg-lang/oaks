use crate::{
    kind::TypstSyntaxKind,
    language::TypstLanguage,
    parser::{State, TypstParser},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> TypstParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, TypstLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_item(state)?;
        }

        Ok(state.finish_at(checkpoint, TypstSyntaxKind::Root))
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(TypstSyntaxKind::Equal) => {
                let checkpoint = state.checkpoint();
                state.bump(); // =
                while state.not_at_end() && state.peek_kind() != Some(TypstSyntaxKind::Newline) {
                    state.bump();
                }
                state.finish_at(checkpoint, TypstSyntaxKind::Heading);
            }
            Some(TypstSyntaxKind::Hash) => {
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
                                TypstSyntaxKind::Identifier
                                    | TypstSyntaxKind::Let
                                    | TypstSyntaxKind::If
                                    | TypstSyntaxKind::Else
                                    | TypstSyntaxKind::For
                                    | TypstSyntaxKind::While
                                    | TypstSyntaxKind::Set
                                    | TypstSyntaxKind::Show
                                    | TypstSyntaxKind::Import
                                    | TypstSyntaxKind::Include
                            )
                        })
                        .unwrap_or(false)
                {
                    state.bump();
                }

                if state.peek_kind() == Some(TypstSyntaxKind::LeftBracket) {
                    state.bump(); // [
                    let mut depth = 1;
                    while state.not_at_end() && depth > 0 {
                        if state.peek_kind() == Some(TypstSyntaxKind::LeftBracket) {
                            depth += 1;
                        }
                        else if state.peek_kind() == Some(TypstSyntaxKind::RightBracket) {
                            depth -= 1;
                        }
                        state.bump();
                    }
                }
                else {
                    // Just a simple #cmd without arguments
                    while state.not_at_end() && state.peek_kind() != Some(TypstSyntaxKind::Newline) && state.peek_kind() != Some(TypstSyntaxKind::Whitespace) {
                        state.bump();
                    }
                }
                state.finish_at(checkpoint, TypstSyntaxKind::Quote);
            }
            Some(TypstSyntaxKind::Dollar) => {
                let checkpoint = state.checkpoint();
                state.bump(); // $
                while state.not_at_end() && state.peek_kind() != Some(TypstSyntaxKind::Dollar) {
                    state.bump();
                }
                if state.peek_kind() == Some(TypstSyntaxKind::Dollar) {
                    state.bump();
                }
                state.finish_at(checkpoint, TypstSyntaxKind::Math);
            }
            Some(TypstSyntaxKind::Star) => {
                let checkpoint = state.checkpoint();
                state.bump(); // *
                while state.not_at_end() && state.peek_kind() != Some(TypstSyntaxKind::Star) {
                    state.bump();
                }
                if state.peek_kind() == Some(TypstSyntaxKind::Star) {
                    state.bump();
                }
                state.finish_at(checkpoint, TypstSyntaxKind::Strong);
            }
            Some(TypstSyntaxKind::Underscore) => {
                let checkpoint = state.checkpoint();
                state.bump(); // _
                while state.not_at_end() && state.peek_kind() != Some(TypstSyntaxKind::Underscore) {
                    state.bump();
                }
                if state.peek_kind() == Some(TypstSyntaxKind::Underscore) {
                    state.bump();
                }
                state.finish_at(checkpoint, TypstSyntaxKind::Emphasis);
            }
            Some(TypstSyntaxKind::Minus) => {
                let checkpoint = state.checkpoint();
                state.bump(); // -
                while state.not_at_end() && state.peek_kind() != Some(TypstSyntaxKind::Newline) {
                    state.bump();
                }
                state.finish_at(checkpoint, TypstSyntaxKind::ListItem);
            }
            Some(TypstSyntaxKind::Plus) => {
                let checkpoint = state.checkpoint();
                state.bump(); // +
                while state.not_at_end() && state.peek_kind() != Some(TypstSyntaxKind::Newline) {
                    state.bump();
                }
                state.finish_at(checkpoint, TypstSyntaxKind::EnumItem);
            }
            Some(TypstSyntaxKind::Backtick) => {
                let checkpoint = state.checkpoint();
                state.bump(); // `
                while state.not_at_end() && state.peek_kind() != Some(TypstSyntaxKind::Backtick) {
                    state.bump();
                }
                if state.peek_kind() == Some(TypstSyntaxKind::Backtick) {
                    state.bump();
                }
                state.finish_at(checkpoint, TypstSyntaxKind::Raw);
            }
            _ => {
                state.bump();
            }
        }
        Ok(())
    }
}
