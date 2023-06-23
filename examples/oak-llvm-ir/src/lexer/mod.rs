#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::LLvmLanguage, lexer::token_type::LLvmTokenType};
use oak_core::{Lexer, LexerCache, LexerState, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, LLvmLanguage>;

#[derive(Clone, Debug)]
pub struct LLvmLexer<'config> {
    _config: &'config LLvmLanguage,
}

impl<'config> Lexer<LLvmLanguage> for LLvmLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<LLvmLanguage>) -> LexOutput<LLvmLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> LLvmLexer<'config> {
    pub fn new(config: &'config LLvmLanguage) -> Self {
        Self { _config: config }
    }
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            let start = state.get_position();
            let safe_point = start;

            if let Some(ch) = state.current() {
                match ch {
                    ' ' | '\t' => {
                        state.advance(1);
                        state.add_token(LLvmTokenType::Whitespace, start, state.get_position());
                    }
                    '\n' | '\r' => {
                        state.advance(1);
                        state.add_token(LLvmTokenType::Newline, start, state.get_position());
                    }
                    ';' => {
                        state.advance(1);
                        while let Some(ch) = state.current() {
                            if ch == '\n' || ch == '\r' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        state.add_token(LLvmTokenType::Comment, start, state.get_position());
                    }
                    '%' => {
                        state.advance(1);
                        while let Some(ch) = state.current() {
                            if !ch.is_alphanumeric() && ch != '.' && ch != '_' && ch != '-' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        state.add_token(LLvmTokenType::LocalVar, start, state.get_position());
                    }
                    '@' => {
                        state.advance(1);
                        while let Some(ch) = state.current() {
                            if !ch.is_alphanumeric() && ch != '.' && ch != '_' && ch != '-' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        state.add_token(LLvmTokenType::GlobalVar, start, state.get_position());
                    }
                    '!' => {
                        state.advance(1);
                        while let Some(ch) = state.current() {
                            if !ch.is_alphanumeric() && ch != '.' && ch != '_' && ch != '-' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        state.add_token(LLvmTokenType::Metadata, start, state.get_position());
                    }
                    '=' => {
                        state.advance(1);
                        state.add_token(LLvmTokenType::Equal, start, state.get_position());
                    }
                    ',' => {
                        state.advance(1);
                        state.add_token(LLvmTokenType::Comma, start, state.get_position());
                    }
                    '(' => {
                        state.advance(1);
                        state.add_token(LLvmTokenType::LParen, start, state.get_position());
                    }
                    ')' => {
                        state.advance(1);
                        state.add_token(LLvmTokenType::RParen, start, state.get_position());
                    }
                    '[' => {
                        state.advance(1);
                        state.add_token(LLvmTokenType::LBracket, start, state.get_position());
                    }
                    ']' => {
                        state.advance(1);
                        state.add_token(LLvmTokenType::RBracket, start, state.get_position());
                    }
                    '{' => {
                        state.advance(1);
                        state.add_token(LLvmTokenType::LBrace, start, state.get_position());
                    }
                    '}' => {
                        state.advance(1);
                        state.add_token(LLvmTokenType::RBrace, start, state.get_position());
                    }
                    '*' => {
                        state.advance(1);
                        state.add_token(LLvmTokenType::Star, start, state.get_position());
                    }
                    ':' => {
                        state.advance(1);
                        state.add_token(LLvmTokenType::Colon, start, state.get_position());
                    }
                    '0'..='9' | '-' => {
                        state.advance(1);
                        while let Some(ch) = state.current() {
                            if !ch.is_ascii_digit() && ch != '.' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        state.add_token(LLvmTokenType::Number, start, state.get_position());
                    }
                    '"' => {
                        state.advance(1);
                        while let Some(ch) = state.current() {
                            if ch == '"' {
                                state.advance(1);
                                break;
                            }
                            if ch == '\\' {
                                state.advance(1);
                            }
                            let len = state.current().map(|c| c.len_utf8()).unwrap_or(0);
                            state.advance(len);
                        }
                        state.add_token(LLvmTokenType::String, start, state.get_position());
                    }
                    _ if ch.is_alphabetic() || ch == '_' || ch == '.' => {
                        state.advance(1);
                        while let Some(ch) = state.current() {
                            if !ch.is_alphanumeric() && ch != '_' && ch != '.' && ch != '-' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        state.add_token(LLvmTokenType::Keyword, start, state.get_position());
                    }
                    _ => {
                        state.advance(ch.len_utf8());
                        state.add_token(LLvmTokenType::Error, start, state.get_position());
                    }
                }
            }
            else {
                break;
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }
}
