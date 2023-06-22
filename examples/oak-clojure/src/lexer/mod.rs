pub mod token_type;
pub use token_type::ClojureTokenType;

use crate::ClojureLanguage;
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::LexOutput,
    source::{Source, TextEdit},
};

#[derive(Clone, Debug)]
pub struct ClojureLexer<'config> {
    _config: &'config ClojureLanguage,
}

type State<'a, S> = LexerState<'a, S, ClojureLanguage>;

impl<'config> Lexer<ClojureLanguage> for ClojureLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<ClojureLanguage>) -> LexOutput<ClojureLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ClojureLexer<'config> {
    pub fn new(config: &'config ClojureLanguage) -> Self {
        Self { _config: config }
    }
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let start = state.get_position();
            let safe_point = start;

            match state.peek() {
                Some(c) if c.is_whitespace() => {
                    self.lex_whitespace(state);
                }
                Some(';') => {
                    self.lex_comment(state);
                }
                Some('"') => {
                    self.lex_string(state);
                }
                Some('\\') => {
                    self.lex_character(state);
                }
                Some(c) if c.is_ascii_digit() => {
                    self.lex_number(state);
                }
                Some(':') => {
                    self.lex_keyword(state);
                }
                Some('#') => {
                    self.lex_dispatch(state);
                }
                Some('(') => {
                    state.advance(1);
                    state.add_token(ClojureTokenType::ListStart, start, state.get_position());
                }
                Some(')') => {
                    state.advance(1);
                    state.add_token(ClojureTokenType::ListEnd, start, state.get_position());
                }
                Some('[') => {
                    state.advance(1);
                    state.add_token(ClojureTokenType::VectorStart, start, state.get_position());
                }
                Some(']') => {
                    state.advance(1);
                    state.add_token(ClojureTokenType::VectorEnd, start, state.get_position());
                }
                Some('{') => {
                    state.advance(1);
                    state.add_token(ClojureTokenType::MapStart, start, state.get_position());
                }
                Some('}') => {
                    state.advance(1);
                    state.add_token(ClojureTokenType::MapEnd, start, state.get_position());
                }
                Some('\'') | Some('`') => {
                    state.advance(1);
                    state.add_token(ClojureTokenType::Quote, start, state.get_position());
                }
                Some('~') => {
                    state.advance(1);
                    if state.peek() == Some('@') {
                        state.advance(1);
                        state.add_token(ClojureTokenType::UnquoteSplice, start, state.get_position());
                    }
                    else {
                        state.add_token(ClojureTokenType::Unquote, start, state.get_position());
                    }
                }
                Some('^') => {
                    state.advance(1);
                    state.add_token(ClojureTokenType::Meta, start, state.get_position());
                }
                Some(_) => {
                    self.lex_symbol(state);
                }
                None => break,
            }

            state.advance_if_dead_lock(safe_point);
        }
        Ok(())
    }
}

impl<'config> ClojureLexer<'config> {
    fn lex_whitespace<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, ClojureLanguage>) {
        let start = state.get_position();
        while let Some(c) = state.peek() {
            if c.is_whitespace() {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }
        state.add_token(ClojureTokenType::Whitespace, start, state.get_position());
    }

    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, ClojureLanguage>) {
        let start = state.get_position();
        state.advance(1); // Skip ';'

        while let Some(c) = state.peek() {
            if c == '\n' {
                break;
            }
            state.advance(c.len_utf8());
        }

        state.add_token(ClojureTokenType::Comment, start, state.get_position());
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, ClojureLanguage>) {
        let start = state.get_position();
        state.advance(1); // Skip opening quote

        while let Some(c) = state.peek() {
            if c == '"' {
                state.advance(1);
                break;
            }
            else if c == '\\' {
                state.advance(1); // Skip escape character
                if let Some(escaped) = state.peek() {
                    state.advance(escaped.len_utf8()); // Skip escaped character
                }
            }
            else {
                state.advance(c.len_utf8());
            }
        }

        state.add_token(ClojureTokenType::StringLiteral, start, state.get_position());
    }

    fn lex_character<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, ClojureLanguage>) {
        let start = state.get_position();
        state.advance(1); // Skip '\'

        if let Some(c) = state.peek() {
            state.advance(c.len_utf8());
        }

        state.add_token(ClojureTokenType::CharacterLiteral, start, state.get_position());
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, ClojureLanguage>) {
        let start = state.get_position();

        while let Some(c) = state.peek() {
            if c.is_ascii_digit() || c == '.' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        state.add_token(ClojureTokenType::NumberLiteral, start, state.get_position());
    }

    fn lex_keyword<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, ClojureLanguage>) {
        let start = state.get_position();
        state.advance(1); // Skip ':'

        while let Some(c) = state.peek() {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == '?' || c == '!' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        state.add_token(ClojureTokenType::KeywordLiteral, start, state.get_position());
    }

    fn lex_dispatch<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, ClojureLanguage>) {
        let start = state.get_position();
        state.advance(1); // Skip '#'

        match state.peek() {
            Some('{') => {
                state.advance(1);
                state.add_token(ClojureTokenType::SetStart, start, state.get_position());
            }
            Some('(') => {
                state.advance(1);
                state.add_token(ClojureTokenType::AnonFnStart, start, state.get_position());
            }
            Some('"') => {
                self.lex_regex(state, start);
            }
            _ => {
                state.add_token(ClojureTokenType::Dispatch, start, state.get_position());
            }
        }
    }

    fn lex_regex<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, ClojureLanguage>, start: usize) {
        state.advance(1); // Skip '"'

        while let Some(c) = state.peek() {
            if c == '"' {
                state.advance(1);
                break;
            }
            else if c == '\\' {
                state.advance(1); // Skip escape character
                if let Some(escaped) = state.peek() {
                    state.advance(escaped.len_utf8()); // Skip escaped character
                }
            }
            else {
                state.advance(c.len_utf8());
            }
        }

        state.add_token(ClojureTokenType::RegexLiteral, start, state.get_position());
    }

    fn lex_symbol<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, ClojureLanguage>) {
        let start = state.get_position();

        while let Some(c) = state.peek() {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == '?' || c == '!' || c == '*' || c == '+' || c == '/' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        state.add_token(ClojureTokenType::Symbol, start, state.get_position());
    }
}
