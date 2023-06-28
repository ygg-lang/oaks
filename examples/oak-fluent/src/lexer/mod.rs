/// Fluent lexer module.
pub mod token_type;

pub use token_type::FluentTokenKind;

use oak_core::{
    LexOutput, Lexer, LexerCache,
    language::Language,
    source::{Source, TextEdit},
};

use crate::language::FluentLanguage;

/// Fluent lexer.
#[derive(Debug, Clone, Default)]
pub struct FluentLexer;

impl Lexer<FluentLanguage> for FluentLexer {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl LexerCache<FluentLanguage>) -> LexOutput<FluentLanguage> {
        let mut state = oak_core::lexer::LexerState::new_with_cache(text, text.length(), cache);

        while state.not_at_end() {
            let safe_point = state.get_position();

            match state.current() {
                Some(' ') | Some('\t') | Some('\r') | Some('\n') => {
                    // Whitespace
                    let start = state.get_position();
                    while state.not_at_end() {
                        match state.current() {
                            Some(' ') | Some('\t') | Some('\r') | Some('\n') => {
                                let _ = state.bump();
                            }
                            _ => break,
                        }
                    }
                    let end = state.get_position();
                    state.add_token(FluentTokenKind::Whitespace, start, end);
                }
                Some('=') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::Equals, state.get_position() - 1, state.get_position());
                }
                Some('[') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::LeftBracket, state.get_position() - 1, state.get_position());
                }
                Some(']') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::RightBracket, state.get_position() - 1, state.get_position());
                }
                Some('{') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::LeftBrace, state.get_position() - 1, state.get_position());
                }
                Some('}') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::RightBrace, state.get_position() - 1, state.get_position());
                }
                Some(',') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::Comma, state.get_position() - 1, state.get_position());
                }
                Some('.') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::Period, state.get_position() - 1, state.get_position());
                }
                Some(':') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::Colon, state.get_position() - 1, state.get_position());
                }
                Some('-') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::Hyphen, state.get_position() - 1, state.get_position());
                }
                Some('_') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::Underscore, state.get_position() - 1, state.get_position());
                }
                Some('@') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::At, state.get_position() - 1, state.get_position());
                }
                Some('#') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::Hash, state.get_position() - 1, state.get_position());
                }
                Some('$') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::Dollar, state.get_position() - 1, state.get_position());
                }
                Some('|') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::Pipe, state.get_position() - 1, state.get_position());
                }
                Some('*') => {
                    let _ = state.bump();
                    state.add_token(FluentTokenKind::Asterisk, state.get_position() - 1, state.get_position());
                }
                Some('"') => {
                    // String literal
                    let start = state.get_position();
                    let _ = state.bump(); // Skip opening quote
                    while state.not_at_end() && state.current() != Some('"') {
                        if state.current() == Some('\\') && state.peek() == Some('"') {
                            let _ = state.bump(); // Skip backslash
                            let _ = state.bump(); // Skip quote
                        }
                        else {
                            let _ = state.bump();
                        }
                    }
                    if state.current() == Some('"') {
                        let _ = state.bump(); // Skip closing quote
                    }
                    let end = state.get_position();
                    state.add_token(FluentTokenKind::StringLiteral, start, end);
                }
                Some('0'..='9') => {
                    // Number literal
                    let start = state.get_position();
                    while state.not_at_end() && state.current().unwrap().is_digit(10) {
                        let _ = state.bump();
                    }
                    let end = state.get_position();
                    state.add_token(FluentTokenKind::NumberLiteral, start, end);
                }
                Some('a'..='z') | Some('A'..='Z') => {
                    // Identifier
                    let start = state.get_position();
                    while state.not_at_end() {
                        let c = state.current().unwrap();
                        if c.is_alphanumeric() || c == '_' || c == '-' {
                            let _ = state.bump();
                        }
                        else {
                            break;
                        }
                    }
                    let end = state.get_position();
                    state.add_token(FluentTokenKind::Identifier, start, end);
                }
                Some('/') if state.peek() == Some('/') => {
                    // Comment
                    let start = state.get_position();
                    while state.not_at_end() && state.current() != Some('\n') {
                        let _ = state.bump();
                    }
                    let end = state.get_position();
                    state.add_token(FluentTokenKind::Comment, start, end);
                }
                Some(_) => {
                    // Error
                    let start = state.get_position();
                    let _ = state.bump();
                    let end = state.get_position();
                    state.add_token(FluentTokenKind::Error, start, end);
                }
                None => break,
            }

            state.advance_if_dead_lock(safe_point);
        }

        state.add_eof();
        state.finish_with_cache(Ok(()), cache)
    }
}
