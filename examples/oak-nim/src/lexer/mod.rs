#![doc = include_str!("readme.md")]
/// Token types for the Nim language.
pub mod token_type;

use crate::{language::NimLanguage, lexer::token_type::NimTokenType};
use oak_core::{Lexer, LexerCache, LexerState, lexer::LexOutput, source::Source};
use std::borrow::Cow;

type State<'s, S> = LexerState<'s, S, NimLanguage>;

/// A lexer for the Nim language.
#[derive(Clone, Debug)]
pub struct NimLexer<'config> {
    config: &'config NimLanguage,
}

impl<'config> NimLexer<'config> {
    /// Creates a new Nim lexer.
    pub fn new(config: &'config NimLanguage) -> Self {
        Self { config }
    }

    /// Skips whitespace
    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(NimTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles newlines
    fn lex_newline<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(NimTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(NimTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles comments
    fn lex_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // Check if it's a doc comment ##
            if let Some('#') = state.peek() {
                state.advance(1);
            }

            // Read until the end of the line
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            let kind = NimTokenType::CommentToken;

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles string literals
    fn lex_string<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                if ch == '\\' {
                    state.advance(1);
                    if let Some(c) = state.peek() {
                        state.advance(c.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(NimTokenType::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles character literals
    fn lex_char<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            if let Some('\\') = state.peek() {
                state.advance(1);
                if let Some(c) = state.peek() {
                    state.advance(c.len_utf8());
                }
            }
            else if let Some(c) = state.peek() {
                if c != '\'' {
                    state.advance(c.len_utf8());
                }
            }

            if let Some('\'') = state.peek() {
                state.advance(1);
            }

            state.add_token(NimTokenType::CharLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles numbers
    fn lex_number<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // Simple float handling
                let mut is_float = false;
                if let Some('.') = state.peek() {
                    state.advance(1);
                    is_float = true;
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }

                let kind = if is_float { NimTokenType::FloatLiteral } else { NimTokenType::IntLiteral };
                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// Handles identifiers and keywords
    fn lex_identifier<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let kind = match text {
                    Cow::Borrowed("and") => NimTokenType::AndKeyword,
                    Cow::Borrowed("or") => NimTokenType::OrKeyword,
                    Cow::Borrowed("not") => NimTokenType::NotKeyword,
                    Cow::Borrowed("if") => NimTokenType::IfKeyword,
                    Cow::Borrowed("else") => NimTokenType::ElseKeyword,
                    Cow::Borrowed("elif") => NimTokenType::ElifKeyword,
                    Cow::Borrowed("while") => NimTokenType::WhileKeyword,
                    Cow::Borrowed("for") => NimTokenType::ForKeyword,
                    Cow::Borrowed("proc") => NimTokenType::ProcKeyword,
                    Cow::Borrowed("func") => NimTokenType::FuncKeyword,
                    Cow::Borrowed("var") => NimTokenType::VarKeyword,
                    Cow::Borrowed("let") => NimTokenType::LetKeyword,
                    Cow::Borrowed("const") => NimTokenType::ConstKeyword,
                    Cow::Borrowed("type") => NimTokenType::TypeKeyword,
                    Cow::Borrowed("import") => NimTokenType::ImportKeyword,
                    Cow::Borrowed("from") => NimTokenType::FromKeyword,
                    Cow::Borrowed("include") => NimTokenType::IncludeKeyword,
                    Cow::Borrowed("return") => NimTokenType::ReturnKeyword,
                    Cow::Borrowed("yield") => NimTokenType::YieldKeyword,
                    Cow::Borrowed("break") => NimTokenType::BreakKeyword,
                    Cow::Borrowed("continue") => NimTokenType::ContinueKeyword,
                    Cow::Borrowed("try") => NimTokenType::TryKeyword,
                    Cow::Borrowed("except") => NimTokenType::ExceptKeyword,
                    Cow::Borrowed("finally") => NimTokenType::FinallyKeyword,
                    Cow::Borrowed("raise") => NimTokenType::RaiseKeyword,
                    Cow::Borrowed("case") => NimTokenType::CaseKeyword,
                    Cow::Borrowed("of") => NimTokenType::OfKeyword,
                    Cow::Borrowed("when") => NimTokenType::WhenKeyword,
                    Cow::Borrowed("is") => NimTokenType::IsKeyword,
                    Cow::Borrowed("in") => NimTokenType::InKeyword,
                    Cow::Borrowed("nil") => NimTokenType::NilKeyword,
                    _ => NimTokenType::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// Handles operators
    fn lex_operator<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            match ch {
                '+' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Plus, start_pos, state.get_position());
                    true
                }
                '-' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Minus, start_pos, state.get_position());
                    true
                }
                '*' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Star, start_pos, state.get_position());
                    true
                }
                '/' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Slash, start_pos, state.get_position());
                    true
                }
                '=' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimTokenType::EqualEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimTokenType::Equal, start_pos, state.get_position());
                    }
                    true
                }
                '!' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimTokenType::NotEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimTokenType::Error, start_pos, state.get_position());
                    }
                    true
                }
                '<' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimTokenType::LessEqual, start_pos, state.get_position());
                    }
                    else if state.peek() == Some('<') {
                        state.advance(1);
                        state.add_token(NimTokenType::LeftShift, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimTokenType::Less, start_pos, state.get_position());
                    }
                    true
                }
                '>' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimTokenType::GreaterEqual, start_pos, state.get_position());
                    }
                    else if state.peek() == Some('>') {
                        state.advance(1);
                        state.add_token(NimTokenType::RightShift, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimTokenType::Greater, start_pos, state.get_position());
                    }
                    true
                }
                '(' => {
                    state.advance(1);
                    state.add_token(NimTokenType::LeftParen, start_pos, state.get_position());
                    true
                }
                ')' => {
                    state.advance(1);
                    state.add_token(NimTokenType::RightParen, start_pos, state.get_position());
                    true
                }
                '[' => {
                    state.advance(1);
                    state.add_token(NimTokenType::LeftBracket, start_pos, state.get_position());
                    true
                }
                ']' => {
                    state.advance(1);
                    state.add_token(NimTokenType::RightBracket, start_pos, state.get_position());
                    true
                }
                '{' => {
                    state.advance(1);
                    state.add_token(NimTokenType::LeftBrace, start_pos, state.get_position());
                    true
                }
                '}' => {
                    state.advance(1);
                    state.add_token(NimTokenType::RightBrace, start_pos, state.get_position());
                    true
                }
                ',' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Comma, start_pos, state.get_position());
                    true
                }
                ';' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Semicolon, start_pos, state.get_position());
                    true
                }
                ':' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Colon, start_pos, state.get_position());
                    true
                }
                '.' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Dot, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// Runs the lexer on the given state.
    pub fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            if self.skip_whitespace(state) || self.lex_newline(state) || self.lex_comment(state) || self.lex_string(state) || self.lex_char(state) || self.lex_number(state) || self.lex_identifier(state) || self.lex_operator(state) {
                continue;
            }

            // If no patterns match, add an error token
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(NimTokenType::Error, start_pos, state.get_position());
            }
        }
        Ok(())
    }
}

impl<'config> Lexer<NimLanguage> for NimLexer<'config> {
    fn lex<'s, S: Source + ?Sized>(&self, source: &'s S, _edits: &[oak_core::source::TextEdit], cache: &'s mut impl LexerCache<NimLanguage>) -> LexOutput<NimLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}
