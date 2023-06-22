use crate::{kind::MatlabSyntaxKind, language::MatlabLanguage};
use oak_core::{
    Lexer, LexerState,
    lexer::{LexOutput, LexerCache},
    source::{Source, TextEdit},
};

type State<'s, S> = LexerState<'s, S, MatlabLanguage>;

#[derive(Clone)]
pub struct MatlabLexer<'config> {
    _config: &'config MatlabLanguage,
}

impl<'config> Lexer<MatlabLanguage> for MatlabLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<MatlabLanguage>) -> LexOutput<MatlabLanguage> {
        let mut state: State<'_, S> = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> MatlabLexer<'config> {
    pub fn new(config: &'config MatlabLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

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

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(MatlabSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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
            state.add_token(MatlabSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_newline<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();
        if state.consume_if_starts_with("\n") || state.consume_if_starts_with("\r\n") || state.consume_if_starts_with("\r") {
            state.add_token(MatlabSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_identifier<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                state.take_while(|c| c.is_ascii_alphanumeric() || c == '_');

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_kind = match text.as_ref() {
                    "function" => MatlabSyntaxKind::Function,
                    "end" => MatlabSyntaxKind::End,
                    "if" => MatlabSyntaxKind::If,
                    "else" => MatlabSyntaxKind::Else,
                    "elseif" => MatlabSyntaxKind::Elseif,
                    "while" => MatlabSyntaxKind::While,
                    "for" => MatlabSyntaxKind::For,
                    "break" => MatlabSyntaxKind::Break,
                    "continue" => MatlabSyntaxKind::Continue,
                    "return" => MatlabSyntaxKind::Return,
                    "switch" => MatlabSyntaxKind::Switch,
                    "case" => MatlabSyntaxKind::Case,
                    "otherwise" => MatlabSyntaxKind::Otherwise,
                    "try" => MatlabSyntaxKind::Try,
                    "catch" => MatlabSyntaxKind::Catch,
                    "global" => MatlabSyntaxKind::Global,
                    "persistent" => MatlabSyntaxKind::Persistent,
                    "classdef" => MatlabSyntaxKind::Classdef,
                    "properties" => MatlabSyntaxKind::Properties,
                    "methods" => MatlabSyntaxKind::Methods,
                    "events" => MatlabSyntaxKind::Events,
                    _ => MatlabSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_number<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || (ch == '.' && state.peek_next_n(1).map(|c| c.is_ascii_digit()).unwrap_or(false)) {
                if ch == '.' {
                    state.advance(1);
                }
                state.take_while(|c| c.is_ascii_digit());

                if ch != '.' && state.consume_if_starts_with(".") {
                    state.take_while(|c| c.is_ascii_digit());
                }

                if state.consume_if_starts_with("e") || state.consume_if_starts_with("E") {
                    if let Some(sign) = state.peek() {
                        if sign == '+' || sign == '-' {
                            state.advance(1);
                        }
                    }
                    state.take_while(|c| c.is_ascii_digit());
                }

                if state.consume_if_starts_with("i") || state.consume_if_starts_with("j") {
                    // complex
                }

                state.add_token(MatlabSyntaxKind::Number, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_string<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();
        if let Some(quote) = state.peek() {
            if quote == '\'' || quote == '"' {
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        if state.peek() == Some(quote) {
                            state.advance(1);
                            continue;
                        }
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if let Some(next) = state.peek() {
                            state.advance(next.len_utf8());
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                let kind = if quote == '\'' { MatlabSyntaxKind::Character } else { MatlabSyntaxKind::String };
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();
        if state.consume_if_starts_with("%") {
            if state.consume_if_starts_with("{") {
                let mut depth = 1;
                while depth > 0 && state.not_at_end() {
                    if state.starts_with("%{") {
                        depth += 1;
                        state.advance(2);
                    }
                    else if state.starts_with("%}") {
                        depth -= 1;
                        state.advance(2);
                    }
                    else if let Some(ch) = state.current() {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(MatlabSyntaxKind::BlockComment, start_pos, state.get_position());
            }
            else {
                state.take_while(|c| c != '\n' && c != '\r');
                state.add_token(MatlabSyntaxKind::Comment, start_pos, state.get_position());
            }
            return true;
        }
        false
    }

    fn lex_operator<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();
        let ops = [".*", "./", ".^", ".\\", "==", "~=", "<=", ">=", "&&", "||", "++", "--", ".'"];
        for op in ops {
            if state.consume_if_starts_with(op) {
                state.add_token(MatlabSyntaxKind::Operator, start_pos, state.get_position());
                return true;
            }
        }

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' | '-' | '*' | '/' | '\\' | '^' | '<' | '>' | '=' | '~' | '&' | '|' | '\'' => MatlabSyntaxKind::Operator,
                _ => return false,
            };
            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
            return true;
        }
        false
    }

    fn lex_delimiter<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' | ':' | '?' | '@' | '.' => MatlabSyntaxKind::Delimiter,
                _ => return false,
            };
            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}
