#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::StylusLanguage, lexer::token_type::StylusTokenType};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

pub(crate) type State<'a, S> = LexerState<'a, S, StylusLanguage>;

/// Stylus lexer implementation.
#[derive(Clone, Debug)]
pub struct StylusLexer<'config> {
    config: &'config StylusLanguage,
}

impl<'config> StylusLexer<'config> {
    /// Create a new Stylus lexer
    pub fn new(config: &'config StylusLanguage) -> Self {
        Self { config }
    }

    /// Skip whitespace characters (excluding newlines)
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(StylusTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handle newlines
    fn lex_newline<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(StylusTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(StylusTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handle comments
    fn lex_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' { break } else { state.advance(ch.len_utf8()) }
            }

            state.add_token(StylusTokenType::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handle string literals
    fn lex_string<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                // Check if it is a multi-line string (three quotes)
                let mut quote_count = 0;

                // Count consecutive quotes
                while let Some(ch) = state.peek_next_n(quote_count) {
                    if ch == quote { quote_count += 1 } else { break }
                }

                if quote_count >= 3 {
                    // Multi-line string
                    state.advance(3); // Skip the starting three quotes

                    while let Some(ch) = state.peek() {
                        if ch == quote {
                            // Check for closing three quotes
                            let mut end_quote_count = 0;

                            while let Some(check_ch) = state.peek_next_n(end_quote_count) {
                                if check_ch == quote { end_quote_count += 1 } else { break }
                            }

                            if end_quote_count >= 3 {
                                state.advance(3); // Skip the closing three quotes
                                break;
                            }
                            else {
                                state.advance(1)
                            }
                        }
                        else if ch == '\\' && quote == '"' {
                            // Handle escape characters (only in basic strings)
                            state.advance(1);
                            if let Some(_) = state.peek() {
                                state.advance(1)
                            }
                        }
                        else {
                            state.advance(ch.len_utf8())
                        }
                    }

                    state.add_token(StylusTokenType::String, start_pos, state.get_position());
                    true
                }
                else {
                    // Single-line string
                    state.advance(1); // Skip starting quote

                    while let Some(ch) = state.peek() {
                        if ch == quote {
                            state.advance(1); // Skip closing quote
                            break;
                        }
                        else if ch == '\n' || ch == '\r' {
                            break; // String cannot span across lines
                        }
                        else if ch == '\\' && quote == '"' {
                            // Handle escape characters (only in double-quoted strings)
                            state.advance(1);
                            if let Some(_) = state.peek() {
                                state.advance(1)
                            }
                        }
                        else {
                            state.advance(ch.len_utf8())
                        }
                    }

                    state.add_token(StylusTokenType::String, start_pos, state.get_position());
                    true
                }
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// Handle number literals
    fn lex_number<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();
        let mut is_float = false;

        // Handle single sign
        if let Some(ch) = state.peek() {
            if ch == '+' || ch == '-' {
                state.advance(1)
            }
        }

        // Handle hex numbers (if allowed)
        if self.config.allow_hex_numbers {
            if state.peek() == Some('0') {
                if let Some('x') | Some('X') = state.peek_next_n(1) {
                    state.advance(2); // Skip "0x"

                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() || ch == '_' { state.advance(1) } else { break }
                    }

                    state.add_token(StylusTokenType::Number, start_pos, state.get_position());
                    return true;
                }
            }
        }

        // Handle decimal numbers
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '_' { state.advance(1) } else { break }
        }

        // Handle decimals
        if let Some('.') = state.peek() {
            if let Some(next_ch) = state.peek_next_n(1) {
                if next_ch.is_ascii_digit() {
                    is_float = true;
                    state.advance(1); // decimal point

                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' { state.advance(1) } else { break }
                    }
                }
            }
        }

        // Handle scientific notation
        if let Some('e') | Some('E') = state.peek() {
            is_float = true;
            state.advance(1);

            if let Some('+') | Some('-') = state.peek() {
                state.advance(1);
            }

            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() || ch == '_' { state.advance(1) } else { break }
            }
        }

        let token_kind = if is_float { StylusTokenType::Number } else { StylusTokenType::Number };

        state.add_token(token_kind, start_pos, state.get_position());
        true
    }

    /// Handle identifiers or keywords
    fn lex_identifier_or_keyword<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' { state.advance(ch.len_utf8()) } else { break }
                }

                let end_pos = state.get_position();
                let text = state.get_text_in(oak_core::Range { start: start_pos, end: end_pos });
                let token_kind = self.keyword_or_identifier(&text);
                state.add_token(token_kind, start_pos, state.get_position());
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

    /// Determine if it is a keyword or an identifier
    fn keyword_or_identifier(&self, text: &str) -> StylusTokenType {
        match text {
            // CSS color keywords
            "red" | "blue" | "green" | "white" | "black" | "transparent" => StylusTokenType::Color,
            // Others are identifiers
            _ => StylusTokenType::Identifier,
        }
    }

    /// Handle delimiters and operators
    fn lex_delimiter<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '{' => StylusTokenType::LeftBrace,
                '}' => StylusTokenType::RightBrace,
                '(' => StylusTokenType::LeftParen,
                ')' => StylusTokenType::RightParen,
                ':' => StylusTokenType::Colon,
                ';' => StylusTokenType::Semicolon,
                ',' => StylusTokenType::Comma,
                '.' => StylusTokenType::Dot,
                '#' => StylusTokenType::Hash,
                '&' => StylusTokenType::Ampersand,
                '+' => StylusTokenType::Plus,
                '-' => StylusTokenType::Minus,
                '*' => StylusTokenType::Star,
                '/' => StylusTokenType::Slash,
                '%' => StylusTokenType::Percent,
                '=' => StylusTokenType::Equal,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> StylusLexer<'config> {
    /// Run the lexer
    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // Try various lexing rules.
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

            if let Some(ch) = state.peek() {
                if ch.is_ascii_digit() || ch == '+' || ch == '-' {
                    if self.lex_number(state) {
                        continue;
                    }
                }
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // If no rules match, skip current character and mark as error
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(StylusTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        state.add_eof();
        Ok(())
    }
}

impl<'config> Lexer<StylusLanguage> for StylusLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<StylusLanguage>) -> LexOutput<StylusLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}
