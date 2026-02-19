use oak_core::{
    Range,
    lexer::{LexOutput, Lexer as CoreLexer, LexerCache, LexerState},
    source::{Source, TextEdit},
};

use crate::language::RacketLanguage;
mod token_type;
pub use token_type::TokenType;

/// Lexer for Racket source code.
pub struct Lexer;

impl CoreLexer<RacketLanguage> for Lexer {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl LexerCache<RacketLanguage>) -> LexOutput<RacketLanguage> {
        let mut state = LexerState::new_with_cache(text, edits.last().map(|edit| edit.span.start).unwrap_or(0), cache);

        while state.not_at_end() {
            let safe_point = state.get_position();

            let whitespace_range = state.skip_ascii_whitespace();
            if whitespace_range.end > whitespace_range.start {
                state.add_token(TokenType::Whitespace, whitespace_range.start, whitespace_range.end);
                continue;
            }

            if state.scan_line_comment(TokenType::Comment, "//") {
                continue;
            }

            if let Some(ch) = state.peek() {
                match ch {
                    '(' => {
                        state.advance(1);
                        state.add_token(TokenType::LParen, safe_point, state.get_position());
                    }
                    ')' => {
                        state.advance(1);
                        state.add_token(TokenType::RParen, safe_point, state.get_position());
                    }
                    '[' => {
                        state.advance(1);
                        state.add_token(TokenType::LBracket, safe_point, state.get_position());
                    }
                    ']' => {
                        state.advance(1);
                        state.add_token(TokenType::RBracket, safe_point, state.get_position());
                    }
                    '{' => {
                        state.advance(1);
                        state.add_token(TokenType::LBrace, safe_point, state.get_position());
                    }
                    '}' => {
                        state.advance(1);
                        state.add_token(TokenType::RBrace, safe_point, state.get_position());
                    }
                    ',' => {
                        state.advance(1);
                        state.add_token(TokenType::Comma, safe_point, state.get_position());
                    }
                    '.' => {
                        state.advance(1);
                        state.add_token(TokenType::Dot, safe_point, state.get_position());
                    }
                    ':' => {
                        state.advance(1);
                        state.add_token(TokenType::Colon, safe_point, state.get_position());
                    }
                    ';' => {
                        state.advance(1);
                        state.add_token(TokenType::Semicolon, safe_point, state.get_position());
                    }
                    '+' => {
                        state.advance(1);
                        state.add_token(TokenType::Plus, safe_point, state.get_position());
                    }
                    '-' => {
                        state.advance(1);
                        state.add_token(TokenType::Minus, safe_point, state.get_position());
                    }
                    '*' => {
                        state.advance(1);
                        state.add_token(TokenType::Multiply, safe_point, state.get_position());
                    }
                    '/' => {
                        state.advance(1);
                        state.add_token(TokenType::Divide, safe_point, state.get_position());
                    }
                    '%' => {
                        state.advance(1);
                        state.add_token(TokenType::Modulo, safe_point, state.get_position());
                    }
                    '=' => {
                        state.advance(1);
                        state.add_token(TokenType::Equals, safe_point, state.get_position());
                    }
                    '!' => {
                        state.advance(1);
                        if state.starts_with("=") {
                            state.advance(1);
                            state.add_token(TokenType::NotEquals, safe_point, state.get_position());
                        }
                        else {
                            state.add_token(TokenType::Not, safe_point, state.get_position());
                        }
                    }
                    '<' => {
                        state.advance(1);
                        if state.starts_with("=") {
                            state.advance(1);
                            state.add_token(TokenType::LessThanOrEqual, safe_point, state.get_position());
                        }
                        else {
                            state.add_token(TokenType::LessThan, safe_point, state.get_position());
                        }
                    }
                    '>' => {
                        state.advance(1);
                        if state.starts_with("=") {
                            state.advance(1);
                            state.add_token(TokenType::GreaterThanOrEqual, safe_point, state.get_position());
                        }
                        else {
                            state.add_token(TokenType::GreaterThan, safe_point, state.get_position());
                        }
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let start = state.get_position();
                        state.advance(1);
                        state.skip_ascii_ident_continue();
                        let end = state.get_position();
                        let range = Range { start, end };
                        let identifier = state.get_text_in(range).to_string();

                        let token_type = match identifier.as_str() {
                            "for" => TokenType::For,
                            "in" => TokenType::In,
                            _ => TokenType::Identifier,
                        };

                        state.add_token(token_type, start, end);
                    }
                    '0'..='9' => {
                        let start = state.get_position();
                        state.advance(1);
                        while state.not_at_end() {
                            if let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() || ch == '.' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                            else {
                                break;
                            }
                        }
                        let end = state.get_position();
                        state.add_token(TokenType::Number, start, end);
                    }
                    '"' => {
                        let start = state.get_position();
                        state.advance(1);
                        while state.not_at_end() {
                            if let Some(ch) = state.peek() {
                                if ch != '"' {
                                    state.advance(1);
                                }
                                else {
                                    state.advance(1);
                                    break;
                                }
                            }
                            else {
                                break;
                            }
                        }
                        let end = state.get_position();
                        state.add_token(TokenType::String, start, end);
                    }
                    _ => {
                        state.advance(1);
                        state.add_token(TokenType::Identifier, safe_point, state.get_position());
                    }
                }
            }

            state.advance_if_dead_lock(safe_point);
        }

        state.add_eof();
        state.finish_with_cache(Ok(()), cache)
    }
}
