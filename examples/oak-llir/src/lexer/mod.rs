use crate::{kind::LLvmSyntaxKind, language::LLvmLanguage};
use oak_core::{Lexer, LexerCache, LexerState, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, LLvmLanguage>;

#[derive(Clone, Copy)]
pub struct LlvmLexer;

impl Lexer<LLvmLanguage> for LlvmLexer {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<LLvmLanguage>) -> LexOutput<LLvmLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl LlvmLexer {
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            let start = state.get_position();
            let safe_point = start;

            if let Some(ch) = state.current() {
                match ch {
                    ' ' | '\t' => {
                        state.advance(1);
                        state.add_token(LLvmSyntaxKind::Whitespace, start, state.get_position());
                    }
                    '\n' | '\r' => {
                        state.advance(1);
                        state.add_token(LLvmSyntaxKind::Newline, start, state.get_position());
                    }
                    ';' => {
                        state.advance(1);
                        while let Some(ch) = state.current() {
                            if ch == '\n' || ch == '\r' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        state.add_token(LLvmSyntaxKind::Comment, start, state.get_position());
                    }
                    '%' => {
                        state.advance(1);
                        while let Some(ch) = state.current() {
                            if !ch.is_alphanumeric() && ch != '.' && ch != '_' && ch != '-' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        state.add_token(LLvmSyntaxKind::LocalVar, start, state.get_position());
                    }
                    '@' => {
                        state.advance(1);
                        while let Some(ch) = state.current() {
                            if !ch.is_alphanumeric() && ch != '.' && ch != '_' && ch != '-' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        state.add_token(LLvmSyntaxKind::GlobalVar, start, state.get_position());
                    }
                    '!' => {
                        state.advance(1);
                        while let Some(ch) = state.current() {
                            if !ch.is_alphanumeric() && ch != '.' && ch != '_' && ch != '-' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        state.add_token(LLvmSyntaxKind::Metadata, start, state.get_position());
                    }
                    '=' => {
                        state.advance(1);
                        state.add_token(LLvmSyntaxKind::Equal, start, state.get_position());
                    }
                    ',' => {
                        state.advance(1);
                        state.add_token(LLvmSyntaxKind::Comma, start, state.get_position());
                    }
                    '(' => {
                        state.advance(1);
                        state.add_token(LLvmSyntaxKind::LParen, start, state.get_position());
                    }
                    ')' => {
                        state.advance(1);
                        state.add_token(LLvmSyntaxKind::RParen, start, state.get_position());
                    }
                    '[' => {
                        state.advance(1);
                        state.add_token(LLvmSyntaxKind::LBracket, start, state.get_position());
                    }
                    ']' => {
                        state.advance(1);
                        state.add_token(LLvmSyntaxKind::RBracket, start, state.get_position());
                    }
                    '{' => {
                        state.advance(1);
                        state.add_token(LLvmSyntaxKind::LBrace, start, state.get_position());
                    }
                    '}' => {
                        state.advance(1);
                        state.add_token(LLvmSyntaxKind::RBrace, start, state.get_position());
                    }
                    '*' => {
                        state.advance(1);
                        state.add_token(LLvmSyntaxKind::Star, start, state.get_position());
                    }
                    ':' => {
                        state.advance(1);
                        state.add_token(LLvmSyntaxKind::Colon, start, state.get_position());
                    }
                    '0'..='9' | '-' => {
                        state.advance(1);
                        while let Some(ch) = state.current() {
                            if !ch.is_ascii_digit() && ch != '.' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        state.add_token(LLvmSyntaxKind::Number, start, state.get_position());
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
                        state.add_token(LLvmSyntaxKind::String, start, state.get_position());
                    }
                    _ if ch.is_alphabetic() || ch == '_' || ch == '.' => {
                        state.advance(1);
                        while let Some(ch) = state.current() {
                            if !ch.is_alphanumeric() && ch != '_' && ch != '.' && ch != '-' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        state.add_token(LLvmSyntaxKind::Keyword, start, state.get_position());
                    }
                    _ => {
                        state.advance(ch.len_utf8());
                        state.add_token(LLvmSyntaxKind::Error, start, state.get_position());
                    }
                }
            }
            else {
                break;
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }
}
