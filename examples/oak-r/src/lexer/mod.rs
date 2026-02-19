#![doc = include_str!("readme.md")]
/// Token type definitions for the R language.
pub mod token_type;
pub use token_type::RTokenType;

use crate::language::RLanguage;
use oak_core::{Lexer, LexerCache, LexerState, Range, lexer::LexOutput, source::Source};

type State<'s, S> = LexerState<'s, S, RLanguage>;

/// Lexer for the R programming language.
///
/// This lexer tokenizes R source code into a sequence of tokens
/// that can be used by the parser to build a syntax tree.
#[derive(Clone)]
pub struct RLexer<'config> {
    config: &'config RLanguage,
}

impl<'config> Lexer<RLanguage> for RLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &mut impl LexerCache<RLanguage>) -> LexOutput<RLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> RLexer<'config> {
    /// Creates a new RLexer with the given language configuration.
    pub fn new(config: &'config RLanguage) -> Self {
        Self { config }
    }

    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            if self.lex_other(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point)
        }
        Ok(())
    }

    /// Skip whitespace
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.current() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
                return true;
            }
        }
        false
    }

    /// Handle comments
    fn lex_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some('#') = state.current() {
            let start_pos = state.get_position();
            state.advance(1); // Skip '#'

            // Read until end of line
            while let Some(ch) = state.current() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }

            state.add_token(RTokenType::Comment, start_pos, state.get_position());
            return true;
        }
        false
    }

    /// Handle string literals
    fn lex_string_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(quote) = state.current() {
            if quote == '"' || quote == '\'' {
                let start_pos = state.get_position();
                state.advance(1); // Skip quote

                while let Some(ch) = state.current() {
                    if ch == quote {
                        state.advance(1); // Skip closing quote
                        state.add_token(RTokenType::StringLiteral, start_pos, state.get_position());
                        return true;
                    }
                    if ch == '\\' {
                        state.advance(1);
                        if let Some(escaped) = state.current() {
                            state.advance(escaped.len_utf8());
                            continue;
                        }
                    }
                    state.advance(ch.len_utf8())
                }

                // Unclosed string
                state.add_token(RTokenType::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handle number literals
    fn lex_number_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.current() {
            if ch.is_ascii_digit() || (ch == '.' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit())) {
                let start_pos = state.get_position();
                let mut has_dot = false;

                while let Some(c) = state.current() {
                    if c.is_ascii_digit() {
                        state.advance(1)
                    }
                    else if c == '.' && !has_dot {
                        has_dot = true;
                        state.advance(1)
                    }
                    else if (c == 'e' || c == 'E') && !state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit() || c == '+' || c == '-') {
                        break;
                    }
                    else if c == 'e' || c == 'E' {
                        state.advance(1);
                        if let Some(next) = state.current() {
                            if next == '+' || next == '-' {
                                state.advance(1)
                            }
                        }
                        while let Some(digit) = state.current() {
                            if digit.is_ascii_digit() { state.advance(1) } else { break }
                        }
                        break;
                    }
                    else if c == 'L' {
                        state.advance(1);
                        state.add_token(RTokenType::IntegerLiteral, start_pos, state.get_position());
                        return true;
                    }
                    else if c == 'i' {
                        state.advance(1);
                        state.add_token(RTokenType::FloatLiteral, start_pos, state.get_position());
                        return true;
                    }
                    else {
                        break;
                    }
                }

                let kind = if has_dot { RTokenType::FloatLiteral } else { RTokenType::IntegerLiteral };
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handle identifiers or keywords
    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.current() {
            if ch.is_alphabetic() || ch == '.' || ch == '_' {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());

                while let Some(c) = state.current() {
                    if c.is_alphanumeric() || c == '.' || c == '_' { state.advance(c.len_utf8()) } else { break }
                }

                let text = state.get_text_in(Range { start: start_pos, end: state.get_position() });
                let kind = match text.as_ref() {
                    "if" => RTokenType::If,
                    "else" => RTokenType::Else,
                    "for" => RTokenType::For,
                    "in" => RTokenType::In,
                    "while" => RTokenType::While,
                    "repeat" => RTokenType::Repeat,
                    "next" => RTokenType::Next,
                    "break" => RTokenType::Break,
                    "function" => RTokenType::Function,
                    "TRUE" => RTokenType::True,
                    "FALSE" => RTokenType::False,
                    "NULL" => RTokenType::Null,
                    "Inf" => RTokenType::Inf,
                    "NaN" => RTokenType::NaN,
                    "NA" => RTokenType::NA,
                    "NA_integer_" => RTokenType::NaInteger,
                    "NA_real_" => RTokenType::NaReal,
                    "NA_complex_" => RTokenType::NaComplex,
                    "NA_character_" => RTokenType::NaCharacter,
                    _ => RTokenType::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handle operators
    fn lex_operators<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();
        if let Some(ch) = state.current() {
            match ch {
                '<' => {
                    state.advance(1);
                    if let Some('-') = state.current() {
                        state.advance(1);
                        state.add_token(RTokenType::LeftArrow, start_pos, state.get_position());
                        return true;
                    }
                    if let Some('<') = state.current() {
                        state.advance(1);
                        if let Some('-') = state.current() {
                            state.advance(1);
                            state.add_token(RTokenType::DoubleLeftArrow, start_pos, state.get_position());
                            return true;
                        }
                    }
                    if let Some('=') = state.current() {
                        state.advance(1);
                        state.add_token(RTokenType::LessEqual, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RTokenType::Less, start_pos, state.get_position());
                    return true;
                }
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.current() {
                        state.advance(1);
                        if let Some('>') = state.current() {
                            state.advance(1);
                            state.add_token(RTokenType::DoubleRightArrow, start_pos, state.get_position());
                            return true;
                        }
                        state.add_token(RTokenType::RightArrow, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RTokenType::Minus, start_pos, state.get_position());
                    return true;
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        state.add_token(RTokenType::EqualEqual, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RTokenType::Equal, start_pos, state.get_position());
                    return true;
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        state.add_token(RTokenType::NotEqual, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RTokenType::Not, start_pos, state.get_position());
                    return true;
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        state.add_token(RTokenType::GreaterEqual, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RTokenType::Greater, start_pos, state.get_position());
                    return true;
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.current() {
                        state.advance(1);
                        state.add_token(RTokenType::AndAnd, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RTokenType::And, start_pos, state.get_position());
                    return true;
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.current() {
                        state.advance(1);
                        state.add_token(RTokenType::OrOr, start_pos, state.get_position());
                        return true;
                    }
                    if let Some('>') = state.current() {
                        state.advance(1);
                        state.add_token(RTokenType::Pipe, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RTokenType::Or, start_pos, state.get_position());
                    return true;
                }
                '%' => {
                    state.advance(1);
                    while let Some(c) = state.current() {
                        state.advance(c.len_utf8());
                        if c == '%' {
                            state.add_token(RTokenType::Operator, start_pos, state.get_position());
                            return true;
                        }
                    }
                    // Unclosed operator
                    state.add_token(RTokenType::Operator, start_pos, state.get_position());
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    /// Handle single-character tokens
    fn lex_single_char_tokens<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.current() {
            let start_pos = state.get_position();
            let kind = match ch {
                '(' => Some(RTokenType::LeftParen),
                ')' => Some(RTokenType::RightParen),
                '[' => Some(RTokenType::LeftBracket),
                ']' => Some(RTokenType::RightBracket),
                '{' => Some(RTokenType::LeftBrace),
                '}' => Some(RTokenType::RightBrace),
                ',' => Some(RTokenType::Comma),
                ';' => Some(RTokenType::Semicolon),
                '+' => Some(RTokenType::Plus),
                '*' => Some(RTokenType::Star),
                '/' => Some(RTokenType::Slash),
                '^' => Some(RTokenType::Caret),
                '$' => Some(RTokenType::Dollar),
                '@' => Some(RTokenType::At),
                '~' => Some(RTokenType::Tilde),
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.current() {
                        state.advance(1);
                        if let Some(':') = state.current() {
                            state.advance(1);
                            Some(RTokenType::TripleColon)
                        }
                        else {
                            Some(RTokenType::DoubleColon)
                        }
                    }
                    else {
                        return {
                            state.add_token(RTokenType::Colon, start_pos, state.get_position());
                            true
                        };
                    }
                }
                '?' => Some(RTokenType::Question),
                _ => None,
            };

            if let Some(k) = kind {
                if !matches!(k, RTokenType::TripleColon | RTokenType::DoubleColon) {
                    state.advance(1);
                }
                state.add_token(k, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handle other characters
    fn lex_other<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.current() {
            let start_pos = state.get_position();
            let len = ch.len_utf8();
            state.advance(len);
            state.add_token(RTokenType::Error, start_pos, state.get_position());
            return true;
        }
        false
    }
}
