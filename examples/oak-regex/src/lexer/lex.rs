use crate::{
    language::RegexLanguage,
    lexer::{RegexLexer, token_type::RegexTokenType},
};
use oak_core::{LexerState, OakError, source::Source};

type State<'s, S> = LexerState<'s, S, RegexLanguage>;

impl<'config> RegexLexer<'config> {
    /// Runs the lexer on the given source state and produces tokens.
    pub(crate) fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();
            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_character_class(state) {
                continue;
            }

            if self.lex_quantifier(state) {
                continue;
            }

            if self.lex_group(state) {
                continue;
            }

            if self.lex_assertion(state) {
                continue;
            }

            if self.lex_escape(state) {
                continue;
            }

            if self.lex_special(state) {
                continue;
            }

            if self.lex_character(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// Skips whitespace characters
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if self.whitespace_rules().scan(state, RegexTokenType::Whitespace) {
            return true;
        }
        false
    }

    /// Skips comments (starting with #)
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        if rest.starts_with("#") {
            state.advance(1); // Consume '#'
            while let Some(ch) = state.peek() {
                if ch == '\n' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(RegexTokenType::Comment, start, state.get_position());
            return true;
        }
        false
    }

    /// Lexes character class [abc] or [^abc]
    fn lex_character_class<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.current() != Some('[') {
            return false;
        }

        state.advance(1); // Consume '['
        state.add_token(RegexTokenType::LBrack, start, state.get_position());

        // Check for negation
        if state.current() == Some('^') {
            let hat_start = state.get_position();
            state.advance(1); // Consume '^'
            state.add_token(RegexTokenType::Hat, hat_start, state.get_position());
        }

        // Parse until closing ']'
        while let Some(c) = state.current() {
            if c == ']' {
                let rbrack_start = state.get_position();
                state.advance(1); // Consume ']'
                state.add_token(RegexTokenType::RBrack, rbrack_start, state.get_position());
                return true;
            }

            if c == '\\' {
                let backslash_start = state.get_position();
                state.advance(1); // Consume '\'
                state.add_token(RegexTokenType::Backslash, backslash_start, state.get_position());
                if let Some(nc) = state.current() {
                    let char_start = state.get_position();
                    state.advance(nc.len_utf8());
                    state.add_token(RegexTokenType::Character, char_start, state.get_position());
                }
            }
            else {
                let char_start = state.get_position();
                state.advance(c.len_utf8());
                state.add_token(RegexTokenType::Character, char_start, state.get_position());
            }
        }

        state.add_token(RegexTokenType::Error, start, state.get_position());
        true
    }

    /// Lexes quantifier like *, +, ?, {n}, {n,}, {n,m}
    fn lex_quantifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        match state.current() {
            Some('*') => {
                state.advance(1);
                state.add_token(RegexTokenType::Star, start, state.get_position());
                true
            }
            Some('+') => {
                state.advance(1);
                state.add_token(RegexTokenType::Plus, start, state.get_position());
                true
            }
            Some('?') => {
                state.advance(1);
                state.add_token(RegexTokenType::Question, start, state.get_position());
                true
            }
            Some('{') => {
                state.advance(1); // Consume '{'
                state.add_token(RegexTokenType::LBrace, start, state.get_position());
                while let Some(c) = state.current() {
                    if c == '}' {
                        let rbrace_start = state.get_position();
                        state.advance(1); // Consume '}'
                        state.add_token(RegexTokenType::RBrace, rbrace_start, state.get_position());
                        return true;
                    }
                    if c.is_ascii_digit() {
                        let digit_start = state.get_position();
                        state.advance(1);
                        state.add_token(RegexTokenType::Digit, digit_start, state.get_position());
                    }
                    else if c == ',' {
                        let comma_start = state.get_position();
                        state.advance(1);
                        state.add_token(RegexTokenType::Comma, comma_start, state.get_position());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(RegexTokenType::Error, start, state.get_position());
                true
            }
            _ => false,
        }
    }

    /// Lexes groups (abc) or (?:abc)
    fn lex_group<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.current() == Some('(') {
            state.advance(1);
            state.add_token(RegexTokenType::LParen, start, state.get_position());
            return true;
        }
        if state.current() == Some(')') {
            state.advance(1);
            state.add_token(RegexTokenType::RParen, start, state.get_position());
            return true;
        }
        false
    }

    /// Lexes assertions like ^, $, \b, \B, (?=abc), (?!abc)
    fn lex_assertion<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        match state.current() {
            Some('^') => {
                state.advance(1);
                state.add_token(RegexTokenType::Hat, start, state.get_position());
                true
            }
            Some('$') => {
                state.advance(1);
                state.add_token(RegexTokenType::Dollar, start, state.get_position());
                true
            }
            _ => false,
        }
    }

    /// Lexes escape sequences like \d, \w, \s, etc.
    fn lex_escape<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.current() == Some('\\') {
            state.advance(1);
            state.add_token(RegexTokenType::Backslash, start, state.get_position());
            if let Some(c) = state.current() {
                let char_start = state.get_position();
                state.advance(c.len_utf8());
                state.add_token(RegexTokenType::Character, char_start, state.get_position());
            }
            return true;
        }
        false
    }

    /// Lexes special characters like .
    fn lex_special<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.current() == Some('.') {
            state.advance(1);
            state.add_token(RegexTokenType::Dot, start, state.get_position());
            return true;
        }
        false
    }

    /// Lexes character literal
    fn lex_character<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(c) = state.current() {
            if !self.is_operator(c) && !self.is_special(c) && c != '[' && c != '(' && c != ')' && c != '{' && c != '\\' {
                state.advance(c.len_utf8());
                state.add_token(RegexTokenType::Character, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// Lexes operators like |
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.current() == Some('|') {
            state.advance(1);
            state.add_token(RegexTokenType::Pipe, start, state.get_position());
            return true;
        }
        false
    }

    fn is_operator(&self, c: char) -> bool {
        c == '|' || c == '*' || c == '+' || c == '?' || c == '{' || c == '}' || c == ','
    }

    fn is_special(&self, c: char) -> bool {
        c == '.' || c == '^' || c == '$'
    }
}
