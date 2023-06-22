use crate::{kind::IniSyntaxKind, language::IniLanguage, parser::IniParser};
use oak_core::{errors::OakError, parser::ParserState, source::Source};

type State<'a, S> = ParserState<'a, IniLanguage, S>;

impl<'config> IniParser<'config> {
    pub(crate) fn parse_table<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        let kind = if state.at(IniSyntaxKind::DoubleLeftBracket) {
            state.expect(IniSyntaxKind::DoubleLeftBracket)?;
            self.parse_key(state)?;
            state.expect(IniSyntaxKind::DoubleRightBracket)?;
            IniSyntaxKind::ArrayOfTables
        }
        else {
            state.expect(IniSyntaxKind::LeftBracket)?;
            self.parse_key(state)?;
            state.expect(IniSyntaxKind::RightBracket)?;
            IniSyntaxKind::Table
        };

        // Sections can have key-values following them
        while state.not_at_end() && !state.at(IniSyntaxKind::LeftBracket) && !state.at(IniSyntaxKind::DoubleLeftBracket) {
            self.skip_trivia(state);
            if !state.not_at_end() || state.at(IniSyntaxKind::LeftBracket) || state.at(IniSyntaxKind::DoubleLeftBracket) {
                break;
            }
            self.parse_key_value(state)?;
        }

        state.finish_at(checkpoint, kind.into());
        Ok(())
    }

    pub(crate) fn parse_key_value<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        self.parse_key(state)?;

        self.skip_trivia(state);
        state.expect(IniSyntaxKind::Equal)?;
        self.skip_trivia(state);

        self.parse_value(state)?;

        state.finish_at(checkpoint, IniSyntaxKind::KeyValue.into());
        Ok(())
    }

    fn parse_key<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        // Support dotted keys: a.b.c
        loop {
            if state.at(IniSyntaxKind::Identifier) {
                state.bump();
            }
            else if state.at(IniSyntaxKind::String) {
                state.bump();
            }
            else {
                let err = oak_core::errors::OakError::expected_token("identifier or string", state.tokens.index(), state.source_url());
                state.errors.push(err);
                return Err(state.errors.last().unwrap().clone());
            }

            self.skip_trivia(state);
            if state.at(IniSyntaxKind::Dot) {
                state.bump();
                self.skip_trivia(state);
            }
            else {
                break;
            }
        }
        state.finish_at(checkpoint, IniSyntaxKind::Key.into());
        Ok(())
    }

    fn parse_value<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        let kind = state.peek_kind().ok_or_else(|| {
            let err = oak_core::errors::OakError::expected_token("value", state.tokens.index(), state.source_url());
            state.errors.push(err);
            state.errors.last().unwrap().clone()
        })?;

        match kind {
            IniSyntaxKind::String | IniSyntaxKind::Integer | IniSyntaxKind::Float | IniSyntaxKind::Boolean | IniSyntaxKind::DateTime => {
                state.bump();
            }
            IniSyntaxKind::LeftBracket => {
                self.parse_array(state)?;
            }
            IniSyntaxKind::LeftBrace => {
                self.parse_inline_table(state)?;
            }
            _ => {
                let err = oak_core::errors::OakError::expected_token("value", state.tokens.index(), state.source_url());
                state.errors.push(err);
                return Err(state.errors.last().unwrap().clone());
            }
        }

        state.finish_at(checkpoint, IniSyntaxKind::Value.into());
        Ok(())
    }

    fn parse_array<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(IniSyntaxKind::LeftBracket)?;
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(IniSyntaxKind::RightBracket) {
            self.parse_value(state)?;
            self.skip_trivia(state);
            if state.at(IniSyntaxKind::Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }

        state.expect(IniSyntaxKind::RightBracket)?;
        state.finish_at(checkpoint, IniSyntaxKind::Array.into());
        Ok(())
    }

    fn parse_inline_table<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(IniSyntaxKind::LeftBrace)?;
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(IniSyntaxKind::RightBrace) {
            self.parse_key_value(state)?;
            self.skip_trivia(state);
            if state.at(IniSyntaxKind::Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }

        state.expect(IniSyntaxKind::RightBrace)?;
        state.finish_at(checkpoint, IniSyntaxKind::InlineTable.into());
        Ok(())
    }

    pub(crate) fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.at(IniSyntaxKind::Whitespace) || state.at(IniSyntaxKind::Comment) || state.at(IniSyntaxKind::Newline) {
            state.bump();
        }
    }
}
