#![doc = include_str!("readme.md")]
//! Lexer implementation for the Koka language.

/// Token types for the Koka lexer.
pub mod token_type;

use crate::{language::KokaLanguage, lexer::token_type::KokaTokenType};
use oak_core::{
    Lexer, LexerState, OakError, Source, TextEdit,
    lexer::{LexOutput, LexerCache},
};

pub(crate) type State<'a, S> = LexerState<'a, S, KokaLanguage>;

trait LexerStateExt {
    fn eat(&mut self, ch: char) -> bool;
}

impl<'a, S: Source + ?Sized> LexerStateExt for State<'a, S> {
    fn eat(&mut self, ch: char) -> bool {
        if self.peek() == Some(ch) {
            self.advance(ch.len_utf8());
            true
        }
        else {
            false
        }
    }
}

/// A lexer for the Koka language.
#[derive(Clone)]
pub struct KokaLexer<'config> {
    config: &'config KokaLanguage,
}

impl<'config> KokaLexer<'config> {
    /// Creates a new Koka lexer.
    pub fn new(config: &'config KokaLanguage) -> Self {
        Self { config }
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(KokaTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles newlines.
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(KokaTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(KokaTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles comments.
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                // Single-line comment
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(KokaTokenType::Comment, start_pos, state.get_position());
                true
            }
            else if let Some('*') = state.peek() {
                // Multi-line comment
                state.advance(1);
                let mut depth = 1;
                while depth > 0 && state.not_at_end() {
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        if let Some('*') = state.peek() {
                            state.advance(1);
                            depth += 1;
                        }
                    }
                    else if let Some('*') = state.peek() {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            depth -= 1;
                        }
                    }
                    else if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(KokaTokenType::Comment, start_pos, state.get_position());
                true
            }
            else {
                // Backtrack, this is a division operator
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// Handles string literals.
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            // Check if it's a triple-quoted string
            if let Some('"') = state.peek() {
                state.advance(1);
                if let Some('"') = state.peek() {
                    // Triple-quoted string
                    state.advance(1);
                    while state.not_at_end() {
                        if let Some('"') = state.peek() {
                            state.advance(1);
                            if let Some('"') = state.peek() {
                                state.advance(1);
                                if let Some('"') = state.peek() {
                                    state.advance(1);
                                    break;
                                }
                            }
                        }
                        else if let Some(ch) = state.peek() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    state.add_token(KokaTokenType::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else {
                    // Empty string ""
                    state.add_token(KokaTokenType::StringLiteral, start_pos, state.get_position());
                    return true;
                }
            }

            // Normal string
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8())
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // String cannot span lines
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(KokaTokenType::StringLiteral, start_pos, state.get_position());
            true
        }
        else if let Some('\'') = state.peek() {
            // Character literal
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(KokaTokenType::CharLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles number literals
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // Handles decimal point
                if let Some('.') = state.peek() {
                    if let Some(next) = state.peek_next_n(1) {
                        if next.is_ascii_digit() {
                            state.advance(1);
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() || ch == '_' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                // Handles exponent part
                if let Some('e') | Some('E') = state.peek() {
                    state.advance(1);
                    if let Some('+') | Some('-') = state.peek() {
                        state.advance(1);
                    }
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                state.add_token(KokaTokenType::NumberLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handles identifiers and keywords
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            // Koka identifiers can start with a letter or underscore, and contain letters, digits, underscores, or hyphens (except at start/end)
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        // In Koka, hyphens are allowed in identifiers
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let kind = match text.as_ref() {
                    "alias" => KokaTokenType::Alias,
                    "as" => KokaTokenType::As,
                    "control" => KokaTokenType::Control,
                    "effect" => KokaTokenType::Effect,
                    "else" => KokaTokenType::Else,
                    "exists" => KokaTokenType::Exists,
                    "false" => KokaTokenType::BooleanLiteral,
                    "fixed" => KokaTokenType::Fixed,
                    "forall" => KokaTokenType::Forall,
                    "fun" => KokaTokenType::Fun,
                    "handler" => KokaTokenType::Handler,
                    "if" => KokaTokenType::If,
                    "import" => KokaTokenType::Import,
                    "is" => KokaTokenType::Is,
                    "linear" => KokaTokenType::Linear,
                    "match" => KokaTokenType::Match,
                    "module" => KokaTokenType::Module,
                    "pub" => KokaTokenType::Pub,
                    "resume" => KokaTokenType::Resume,
                    "return" => KokaTokenType::Return,
                    "struct" => KokaTokenType::Struct,
                    "true" => KokaTokenType::BooleanLiteral,
                    "type" => KokaTokenType::Type,
                    "val" => KokaTokenType::Val,
                    "var" => KokaTokenType::Var,
                    "with" => KokaTokenType::With,
                    _ => KokaTokenType::Identifier,
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

    /// Handles special characters and operators
    fn lex_special_char<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => {
                    state.advance(1);
                    KokaTokenType::LParen
                }
                ')' => {
                    state.advance(1);
                    KokaTokenType::RParen
                }
                '{' => {
                    state.advance(1);
                    KokaTokenType::LBrace
                }
                '}' => {
                    state.advance(1);
                    KokaTokenType::RBrace
                }
                '[' => {
                    state.advance(1);
                    KokaTokenType::LBracket
                }
                ']' => {
                    state.advance(1);
                    KokaTokenType::RBracket
                }
                ',' => {
                    state.advance(1);
                    KokaTokenType::Comma
                }
                ';' => {
                    state.advance(1);
                    KokaTokenType::Semi
                }
                ':' => {
                    state.advance(1);
                    if state.eat('=') { KokaTokenType::ColonAssign } else { KokaTokenType::Colon }
                }
                '.' => {
                    state.advance(1);
                    if state.eat('.') { KokaTokenType::Range } else { KokaTokenType::Dot }
                }
                '+' => {
                    state.advance(1);
                    if state.eat('=') { KokaTokenType::PlusAssign } else { KokaTokenType::Plus }
                }
                '-' => {
                    state.advance(1);
                    if state.eat('>') {
                        KokaTokenType::Arrow
                    }
                    else if state.eat('=') {
                        KokaTokenType::MinusAssign
                    }
                    else {
                        KokaTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if state.eat('=') { KokaTokenType::StarAssign } else { KokaTokenType::Star }
                }
                '/' => {
                    state.advance(1);
                    if state.eat('=') { KokaTokenType::SlashAssign } else { KokaTokenType::Slash }
                }
                '%' => {
                    state.advance(1);
                    if state.eat('=') { KokaTokenType::PercentAssign } else { KokaTokenType::Percent }
                }
                '=' => {
                    state.advance(1);
                    if state.eat('=') { KokaTokenType::EqEq } else { KokaTokenType::Assign }
                }
                '!' => {
                    state.advance(1);
                    if state.eat('=') { KokaTokenType::NotEq } else { KokaTokenType::Exclamation }
                }
                '~' => {
                    state.advance(1);
                    KokaTokenType::Tilde
                }
                '^' => {
                    state.advance(1);
                    KokaTokenType::Caret
                }
                '<' => {
                    state.advance(1);
                    if state.eat('=') { KokaTokenType::LtEq } else { KokaTokenType::LAngle }
                }
                '>' => {
                    state.advance(1);
                    if state.eat('=') { KokaTokenType::GtEq } else { KokaTokenType::RAngle }
                }
                '&' => {
                    state.advance(1);
                    if state.eat('&') { KokaTokenType::AndAnd } else { KokaTokenType::Ampersand }
                }
                '|' => {
                    state.advance(1);
                    if state.eat('|') { KokaTokenType::OrOr } else { KokaTokenType::Pipe }
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<KokaLanguage> for KokaLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<KokaLanguage>) -> LexOutput<KokaLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> KokaLexer<'config> {
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_special_char(state) {
                continue;
            }

            // If no rules match, skip the current character and mark it as an error
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(KokaTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }
}
