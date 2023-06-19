use oak_core::{lexer::{Lexer, LexerState}, source::Source};
use crate::{kind::LlvmKind, language::LlvmLanguage};

pub struct LlvmLexer;

impl Lexer<LlvmLanguage> for LlvmLexer {
    fn lex_incremental<S: Source>(&self, state: &mut LexerState<S, LlvmLanguage>) {
        while state.not_at_end() {
            let start = state.get_position();
            
            if let Some(ch) = state.current() {
                match ch {
                    ' ' | '\t' => {
                        state.advance(1);
                        state.add_token(LlvmKind::Whitespace, start, state.get_position());
                    },
                    '\n' | '\r' => {
                        state.advance(1);
                        state.add_token(LlvmKind::Newline, start, state.get_position());
                    },
                    ';' => {
                        // Line comment
                        state.advance(1);
                        while let Some(c) = state.current() {
                            if c == '\n' || c == '\r' {
                                break;
                            }
                            state.advance(c.len_utf8());
                        }
                        state.add_token(LlvmKind::Comment, start, state.get_position());
                    },
                    '"' => {
                        // String literal
                        state.advance(1);
                        while let Some(c) = state.current() {
                            if c == '"' {
                                state.advance(1);
                                break;
                            }
                            state.advance(c.len_utf8());
                        }
                        state.add_token(LlvmKind::String, start, state.get_position());
                    },
                    '0'..='9' => {
                        // Number
                        while let Some(c) = state.current() {
                            if !c.is_ascii_digit() && c != '.' {
                                break;
                            }
                            state.advance(c.len_utf8());
                        }
                        state.add_token(LlvmKind::Number, start, state.get_position());
                    },
                    'a'..='z' | 'A'..='Z' | '_' | '%' | '@' => {
                        // Identifier
                        while let Some(c) = state.current() {
                            if !c.is_alphanumeric() && c != '_' && c != '.' {
                                break;
                            }
                            state.advance(c.len_utf8());
                        }
                        state.add_token(LlvmKind::Identifier, start, state.get_position());
                    },
                    _ => {
                        // Unknown character, mark as error
                        state.advance(ch.len_utf8());
                        state.add_token(LlvmKind::Error, start, state.get_position());
                    }
                }
            } else {
                break;
            }
        }
        
        // Add EOF token
        let pos = state.get_position();
        state.add_token(LlvmKind::Eof, pos, pos);
    }
}