use crate::{
    language::IniLanguage,
    lexer::token_type::IniTokenType,
    parser::{IniParser, element_type::IniElementType},
};
use oak_core::{errors::OakError, parser::ParserState, source::Source};

type State<'a, S> = ParserState<'a, IniLanguage, S>;

impl<'config> IniParser<'config> {
    pub(crate) fn parse_table<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        let kind = if state.at(IniTokenType::DoubleLeftBracket) {
            state.expect(IniTokenType::DoubleLeftBracket)?;
            self.parse_key(state)?;
            state.expect(IniTokenType::DoubleRightBracket)?;
            IniElementType::ArrayOfTables
        }
        else {
            state.expect(IniTokenType::LeftBracket)?;
            self.parse_key(state)?;
            state.expect(IniTokenType::RightBracket)?;
            IniElementType::Table
        };

        // Sections can have key-values following them
        while state.not_at_end() && !state.at(IniTokenType::LeftBracket) && !state.at(IniTokenType::DoubleLeftBracket) {
            self.skip_trivia(state);
            if !state.not_at_end() || state.at(IniTokenType::LeftBracket) || state.at(IniTokenType::DoubleLeftBracket) {
                break;
            }
            self.parse_key_value(state)?;
        }

        state.finish_at(checkpoint, kind);
        Ok(())
    }

    pub(crate) fn parse_key_value<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        self.parse_key(state)?;

        self.skip_trivia(state);
        state.expect(IniTokenType::Equal)?;
        self.skip_trivia(state);

        self.parse_value(state)?;

        state.finish_at(checkpoint, crate::parser::element_type::IniElementType::KeyValue);
        Ok(())
    }

    fn parse_key<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        // Support dotted keys: a.b.c
        loop {
            if state.at(IniTokenType::Identifier) {
                state.bump();
            }
            else if state.at(IniTokenType::String) {
                state.bump();
            }
            else {
                let err = oak_core::errors::OakError::expected_token("identifier or string", state.tokens.index(), state.source_id());
                state.errors.push(err);
                return Err(state.errors.last().unwrap().clone());
            }

            self.skip_trivia(state);
            if state.at(IniTokenType::Dot) {
                state.bump();
                self.skip_trivia(state);
            }
            else {
                break;
            }
        }
        state.finish_at(checkpoint, crate::parser::element_type::IniElementType::Key);
        Ok(())
    }

    fn parse_value<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        let kind = state.peek_kind().ok_or_else(|| {
            let err = oak_core::errors::OakError::expected_token("value", state.tokens.index(), state.source_id());
            state.errors.push(err);
            state.errors.last().unwrap().clone()
        })?;

        match kind {
            IniTokenType::String | IniTokenType::Integer | IniTokenType::Float | IniTokenType::Boolean | IniTokenType::DateTime => {
                state.bump();
            }
            IniTokenType::LeftBracket => {
                self.parse_array(state)?;
            }
            IniTokenType::LeftBrace => {
                self.parse_inline_table(state)?;
            }
            _ => {
                let err = oak_core::errors::OakError::expected_token("value", state.tokens.index(), state.source_id());
                state.errors.push(err);
                return Err(state.errors.last().unwrap().clone());
            }
        }

        state.finish_at(checkpoint, crate::parser::element_type::IniElementType::Value);
        Ok(())
    }

    fn parse_array<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(IniTokenType::LeftBracket)?;
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(IniTokenType::RightBracket) {
            self.parse_value(state)?;
            self.skip_trivia(state);
            if state.at(IniTokenType::Comma) {
                state.bump();
                self.skip_trivia(state)
            }
        }

        state.expect(IniTokenType::RightBracket)?;
        state.finish_at(checkpoint, crate::parser::element_type::IniElementType::Array);
        Ok(())
    }

    fn parse_inline_table<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(IniTokenType::LeftBrace)?;
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(IniTokenType::RightBrace) {
            self.parse_key_value(state)?;
            self.skip_trivia(state);
            if state.at(IniTokenType::Comma) {
                state.bump();
                self.skip_trivia(state)
            }
        }

        state.expect(IniTokenType::RightBrace)?;
        state.finish_at(checkpoint, crate::parser::element_type::IniElementType::InlineTable);
        Ok(())
    }

    pub(crate) fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.at(IniTokenType::Whitespace) || state.at(IniTokenType::Comment) || state.at(IniTokenType::Newline) {
            state.bump()
        }
    }
}
