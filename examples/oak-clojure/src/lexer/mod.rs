use crate::{ClojureLanguage, ClojureSyntaxKind};
use oak_core::{
    errors::OakError,
    lexer::{LexOutput, Lexer, LexerState},
    source::Source,
    tree::IncrementalCache,
};

pub struct ClojureLexer;

impl Lexer<ClojureLanguage> for ClojureLexer {
    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<ClojureLanguage>,
    ) -> LexOutput<ClojureLanguage> {
        let mut state = LexerState::new_with_cache(source, _changed, _cache);
        while state.not_at_end() {
            let start = state.get_position();

            match state.current() {
                Some(c) if c.is_whitespace() => {
                    self.lex_whitespace(&mut state);
                }
                Some(';') => {
                    self.lex_comment(&mut state);
                }
                Some('"') => {
                    self.lex_string(&mut state);
                }
                Some('\\') => {
                    self.lex_character(&mut state);
                }
                Some(c) if c.is_ascii_digit() => {
                    self.lex_number(&mut state);
                }
                Some(':') => {
                    self.lex_keyword(&mut state);
                }
                Some('#') => {
                    self.lex_dispatch(&mut state);
                }
                Some('(') => {
                    state.advance(1);
                    state.add_token(ClojureSyntaxKind::ListStart, start, state.get_position());
                }
                Some(')') => {
                    state.advance(1);
                    state.add_token(ClojureSyntaxKind::ListEnd, start, state.get_position());
                }
                Some('[') => {
                    state.advance(1);
                    state.add_token(ClojureSyntaxKind::VectorStart, start, state.get_position());
                }
                Some(']') => {
                    state.advance(1);
                    state.add_token(ClojureSyntaxKind::VectorEnd, start, state.get_position());
                }
                Some('{') => {
                    state.advance(1);
                    state.add_token(ClojureSyntaxKind::MapStart, start, state.get_position());
                }
                Some('}') => {
                    state.advance(1);
                    state.add_token(ClojureSyntaxKind::MapEnd, start, state.get_position());
                }
                Some('\'') => {
                    state.advance(1);
                    state.add_token(ClojureSyntaxKind::Quote, start, state.get_position());
                }
                Some('`') => {
                    state.advance(1);
                    state.add_token(ClojureSyntaxKind::Quote, start, state.get_position());
                }
                Some('~') => {
                    if state.peek() == Some('@') {
                        state.advance(2);
                        state.add_token(ClojureSyntaxKind::UnquoteSplice, start, state.get_position());
                    }
                    else {
                        state.advance(1);
                        state.add_token(ClojureSyntaxKind::Unquote, start, state.get_position());
                    }
                }
                Some('^') => {
                    state.advance(1);
                    state.add_token(ClojureSyntaxKind::Meta, start, state.get_position());
                }
                Some(_) => {
                    self.lex_symbol(&mut state);
                }
                None => break,
            }
        }

        state.finish(Ok(()))
    }
}

impl ClojureLexer {
    fn lex_whitespace<S: Source>(&self, state: &mut LexerState<S, ClojureLanguage>) {
        let start = state.get_position();
        while let Some(c) = state.current() {
            if c.is_whitespace() {
                state.advance(1);
            }
            else {
                break;
            }
        }
        state.add_token(ClojureSyntaxKind::Whitespace, start, state.get_position());
    }

    fn lex_comment<S: Source>(&self, state: &mut LexerState<S, ClojureLanguage>) {
        let start = state.get_position();
        state.advance(1); // Skip ';'

        while let Some(c) = state.current() {
            if c == '\n' {
                break;
            }
            state.advance(1);
        }

        state.add_token(ClojureSyntaxKind::Comment, start, state.get_position());
    }

    fn lex_string<S: Source>(&self, state: &mut LexerState<S, ClojureLanguage>) {
        let start = state.get_position();
        state.advance(1); // Skip opening quote

        while let Some(c) = state.current() {
            if c == '"' {
                state.advance(1);
                break;
            }
            else if c == '\\' {
                state.advance(1); // Skip escape character
                if state.current().is_some() {
                    state.advance(1); // Skip escaped character
                }
            }
            else {
                state.advance(1);
            }
        }

        state.add_token(ClojureSyntaxKind::StringLiteral, start, state.get_position());
    }

    fn lex_character<S: Source>(&self, state: &mut LexerState<S, ClojureLanguage>) {
        let start = state.get_position();
        state.advance(1); // Skip '\'

        if let Some(_) = state.current() {
            state.advance(1);
        }

        state.add_token(ClojureSyntaxKind::CharacterLiteral, start, state.get_position());
    }

    fn lex_number<S: Source>(&self, state: &mut LexerState<S, ClojureLanguage>) {
        let start = state.get_position();

        while let Some(c) = state.current() {
            if c.is_ascii_digit() || c == '.' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        state.add_token(ClojureSyntaxKind::NumberLiteral, start, state.get_position());
    }

    fn lex_keyword<S: Source>(&self, state: &mut LexerState<S, ClojureLanguage>) {
        let start = state.get_position();
        state.advance(1); // Skip ':'

        while let Some(c) = state.current() {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == '?' || c == '!' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        state.add_token(ClojureSyntaxKind::KeywordLiteral, start, state.get_position());
    }

    fn lex_dispatch<S: Source>(&self, state: &mut LexerState<S, ClojureLanguage>) {
        let start = state.get_position();
        state.advance(1); // Skip '#'

        match state.current() {
            Some('{') => {
                state.advance(1);
                state.add_token(ClojureSyntaxKind::SetStart, start, state.get_position());
            }
            Some('(') => {
                state.advance(1);
                state.add_token(ClojureSyntaxKind::AnonFnStart, start, state.get_position());
            }
            Some('"') => {
                self.lex_regex(state, start);
            }
            _ => {
                state.add_token(ClojureSyntaxKind::Dispatch, start, state.get_position());
            }
        }
    }

    fn lex_regex<S: Source>(&self, state: &mut LexerState<S, ClojureLanguage>, start: usize) {
        state.advance(1); // Skip '"'

        while let Some(c) = state.current() {
            if c == '"' {
                state.advance(1);
                break;
            }
            else if c == '\\' {
                state.advance(1); // Skip escape character
                if state.current().is_some() {
                    state.advance(1); // Skip escaped character
                }
            }
            else {
                state.advance(1);
            }
        }

        state.add_token(ClojureSyntaxKind::RegexLiteral, start, state.get_position());
    }

    fn lex_symbol<S: Source>(&self, state: &mut LexerState<S, ClojureLanguage>) {
        let start = state.get_position();

        while let Some(c) = state.current() {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == '?' || c == '!' || c == '*' || c == '+' || c == '/' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        state.add_token(ClojureSyntaxKind::Symbol, start, state.get_position());
    }
}
