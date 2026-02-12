#![doc = include_str!("readme.md")]
//! Lexer implementation for the VON language.

use oak_core::{
    Lexer, LexerState, Source, TextEdit,
    lexer::{LexOutput, LexerCache},
};

/// Token types for the VON language.
pub mod token_type;
use crate::language::VonLanguage;
pub use token_type::{VonToken, VonTokenType};

pub(crate) type State<'a, S> = LexerState<'a, S, VonLanguage>;

/// A lexer for the VON language.
#[derive(Clone, Debug)]
pub struct VonLexer<'config> {
    config: &'config VonLanguage,
}

impl<'config> VonLexer<'config> {
    /// Creates a new `VonLexer` with the given configuration.
    pub fn new(config: &'config VonLanguage) -> Self {
        Self { config }
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
            state.add_token(VonTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a newline.
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(VonTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(VonTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a comment.
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // Single-line comment #
        if let Some('#') = state.peek() {
            state.advance(1);

            // Read until end of line
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(VonTokenType::Comment, start_pos, state.get_position());
            return true;
        }
        false
    }

    /// Lexes a string literal or raw string.
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Check for raw string raw"..."
        let mut is_raw = false;
        if let Some('r') = state.peek() {
            if let Some('a') = state.peek_next_n(1) {
                if let Some('w') = state.peek_next_n(2) {
                    if let Some(c) = state.peek_next_n(3) {
                        if c == '"' || c == '\'' {
                            is_raw = true;
                            // Note: don't advance directly here, let the subsequent logic handle quotes
                        }
                    }
                }
            }
        }

        let quote = if is_raw {
            state.peek_next_n(3).unwrap()
        }
        else {
            match state.peek() {
                Some(c) if c == '"' || c == '\'' => c,
                _ => return false,
            }
        };

        if is_raw {
            state.advance(3);
        }

        let mut quote_count = 0;
        while let Some(c) = state.peek() {
            if c == quote {
                quote_count += 1;
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        // "" or '' are empty strings
        if quote_count == 2 {
            state.add_token(VonTokenType::StringLiteral, start, state.get_position());
            return true;
        }

        if quote_count == 0 {
            state.set_position(start);
            return false;
        }

        let mut current_consecutive = 0;
        let mut escaped = false;

        while let Some(c) = state.peek() {
            if !is_raw && escaped {
                escaped = false;
                state.advance(c.len_utf8());
                current_consecutive = 0;
                continue;
            }

            if !is_raw && c == '\\' && quote_count == 1 {
                escaped = true;
                state.advance(1);
                current_consecutive = 0;
                continue;
            }

            if c == quote {
                current_consecutive += 1;
                state.advance(c.len_utf8());
                if current_consecutive == quote_count {
                    state.add_token(VonTokenType::StringLiteral, start, state.get_position());
                    return true;
                }
            }
            else {
                current_consecutive = 0;
                state.advance(c.len_utf8());
            }
        }

        // Unclosed string, mark as error but still treat as string for syntax highlighting
        state.add_token(VonTokenType::Error, start, state.get_position());
        true
    }

    /// Handles number literals.
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            // Number must start with a digit, negative sign, or dot (followed by digit)
            let is_number_start = ch.is_ascii_digit() || (ch == '-' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit())) || (ch == '.' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()));

            if !is_number_start {
                return false;
            }

            if ch == '-' {
                state.advance(1);
            }

            // Integer part
            if let Some(first) = state.peek() {
                if first.is_ascii_digit() {
                    while let Some(digit) = state.peek() {
                        if digit.is_ascii_digit() || digit == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }

            // Check for dot
            if let Some('.') = state.peek() {
                let mut lookahead = 1;
                while let Some(c) = state.peek_next_n(lookahead) {
                    if c == '_' {
                        lookahead += 1;
                    }
                    else {
                        break;
                    }
                }
                if let Some(next_ch) = state.peek_next_n(lookahead) {
                    if next_ch.is_ascii_digit() {
                        state.advance(1); // Skip dot
                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_digit() || digit == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
            }

            // Check for exponent
            if let Some(e) = state.peek() {
                if e == 'e' || e == 'E' {
                    // Ensure exponent is followed by digits (or sign + digits)
                    let mut lookahead = 1;
                    if let Some(sign) = state.peek_next_n(lookahead) {
                        if sign == '+' || sign == '-' {
                            lookahead += 1;
                        }
                    }

                    let has_digits = state.peek_next_n(lookahead).map_or(false, |c| c.is_ascii_digit() || (c == '_' && state.peek_next_n(lookahead + 1).map_or(false, |n| n.is_ascii_digit())));

                    if has_digits {
                        state.advance(1); // Skip e/E

                        // Optional sign
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }

                        // Exponent digits
                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_digit() || digit == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
            }

            // Only considered a number if at least one digit or negative sign followed by digit is consumed
            // Also check that it's not immediately followed by a letter, which might be an identifier (e.g. version)
            if state.get_position() > start_pos {
                if let Some(next) = state.peek() {
                    if next.is_ascii_alphabetic() || next == '_' {
                        state.set_position(start_pos);
                        return false;
                    }
                }
                state.add_token(VonTokenType::NumberLiteral, start_pos, state.get_position());
                return true;
            }
            false
        }
        else {
            false
        }
    }

    /// Handles identifiers and keywords.
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                // If it's 'r', it might be 'raw', need to check if it's the start of a raw string
                if ch == 'r' {
                    if let Some('a') = state.peek_next_n(1) {
                        if let Some('w') = state.peek_next_n(2) {
                            if let Some(c) = state.peek_next_n(3) {
                                if c == '"' || c == '\'' {
                                    // This is a raw string, handled by lex_string
                                    return false;
                                }
                            }
                        }
                    }
                }

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_kind = match text.as_ref() {
                    "true" | "false" => VonTokenType::BoolLiteral,
                    "null" => VonTokenType::NullLiteral,
                    _ => VonTokenType::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handles operators and punctuation.
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '[' => {
                    state.advance(1);
                    VonTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    VonTokenType::RightBracket
                }
                '{' => {
                    state.advance(1);
                    VonTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    VonTokenType::RightBrace
                }
                ',' => {
                    state.advance(1);
                    VonTokenType::Comma
                }
                ':' => {
                    state.advance(1);
                    VonTokenType::Colon
                }
                '=' => {
                    state.advance(1);
                    VonTokenType::Eq
                }
                _ => return false,
            };
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<VonLanguage> for VonLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], _cache: &'a mut impl LexerCache<VonLanguage>) -> LexOutput<VonLanguage> {
        let mut state = State::new(source);
        while state.not_at_end() {
            if self.skip_whitespace(&mut state) {
                continue;
            }
            if self.lex_newline(&mut state) {
                continue;
            }
            if self.lex_comment(&mut state) {
                continue;
            }
            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }
            if self.lex_number(&mut state) {
                continue;
            }
            if self.lex_string(&mut state) {
                continue;
            }
            if self.lex_operator(&mut state) {
                continue;
            }

            // If no match, treat as error and skip one character
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(VonTokenType::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        state.finish(Ok(()))
    }
}
