#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::lexer::token_type::{VueLanguage, VueTokenType};
use oak_core::{
    Lexer, LexerState,
    lexer::{LexOutput, LexerCache},
    source::Source,
};

#[derive(Clone, Debug)]
pub struct VueLexer<'config> {
    _config: &'config VueLanguage,
}

type State<'a, S> = LexerState<'a, S, VueLanguage>;

impl<'config> VueLexer<'config> {
    pub fn new(config: &'config VueLanguage) -> Self {
        Self { _config: config }
    }

    fn lex_token<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        if self.lex_whitespace(state) {
            return;
        }
        if self.lex_comment(state) {
            return;
        }

        let start_pos = state.get_position();
        let ch = match state.peek() {
            Some(c) => c,
            None => {
                state.add_token(VueTokenType::Eof, start_pos, start_pos);
                return;
            }
        };

        match ch {
            '|' => {
                if state.peek_next_n(1) == Some('|') {
                    state.advance(2);
                    state.add_token(VueTokenType::Or, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueTokenType::Pipe, start_pos, start_pos + 1);
                }
            }
            '&' => {
                if state.peek_next_n(1) == Some('&') {
                    state.advance(2);
                    state.add_token(VueTokenType::And, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueTokenType::Amp, start_pos, start_pos + 1);
                }
            }
            '!' => {
                if state.peek_next_n(1) == Some('=') {
                    state.advance(2);
                    state.add_token(VueTokenType::NotEq, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueTokenType::Bang, start_pos, start_pos + 1);
                }
            }
            '+' => {
                state.advance(1);
                state.add_token(VueTokenType::Plus, start_pos, start_pos + 1);
            }
            '-' => {
                state.advance(1);
                state.add_token(VueTokenType::Minus, start_pos, start_pos + 1);
            }
            '*' => {
                state.advance(1);
                state.add_token(VueTokenType::Star, start_pos, start_pos + 1);
            }
            '%' => {
                state.advance(1);
                state.add_token(VueTokenType::Percent, start_pos, start_pos + 1);
            }
            '?' => {
                state.advance(1);
                state.add_token(VueTokenType::Question, start_pos, start_pos + 1);
            }
            '<' => {
                if state.rest().starts_with("</") {
                    state.advance(2);
                    state.add_token(VueTokenType::LtSlash, start_pos, start_pos + 2);
                }
                else if state.peek_next_n(1) == Some('=') {
                    state.advance(2);
                    state.add_token(VueTokenType::LtEq, start_pos, start_pos + 2);
                }
                else if state.rest().starts_with("<!--") {
                    state.advance(4);
                    while state.not_at_end() && !state.rest().starts_with("-->") {
                        state.advance(1);
                    }
                    if state.rest().starts_with("-->") {
                        state.advance(3);
                    }
                    state.add_token(VueTokenType::Comment, start_pos, state.get_position());
                }
                else if state.rest().starts_with("<!") {
                    state.advance(2);
                    state.add_token(VueTokenType::DocTypeStart, start_pos, start_pos + 2);
                }
                else if state.rest().starts_with("<script") && !is_ident_continue(state.peek_next_n(7).unwrap_or(' ')) {
                    state.advance(7);
                    state.add_token(VueTokenType::ScriptStart, start_pos, start_pos + 7);
                }
                else if state.rest().starts_with("<style") && !is_ident_continue(state.peek_next_n(6).unwrap_or(' ')) {
                    state.advance(6);
                    state.add_token(VueTokenType::StyleStart, start_pos, start_pos + 6);
                }
                else if state.rest().starts_with("<template") && !is_ident_continue(state.peek_next_n(9).unwrap_or(' ')) {
                    state.advance(9);
                    state.add_token(VueTokenType::TemplateStart, start_pos, start_pos + 9);
                }
                else {
                    state.advance(1);
                    state.add_token(VueTokenType::Lt, start_pos, start_pos + 1);
                }
            }
            '>' => {
                if state.peek_next_n(1) == Some('=') {
                    state.advance(2);
                    state.add_token(VueTokenType::GtEq, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueTokenType::Gt, start_pos, start_pos + 1);
                }
            }
            '/' => {
                if state.peek_next_n(1) == Some('>') {
                    state.advance(2);
                    state.add_token(VueTokenType::SlashGt, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueTokenType::Slash, start_pos, start_pos + 1);
                }
            }
            '=' => {
                if state.peek_next_n(1) == Some('>') {
                    state.advance(2);
                    state.add_token(VueTokenType::Arrow, start_pos, start_pos + 2);
                }
                else if state.peek_next_n(1) == Some('=') {
                    state.advance(2);
                    state.add_token(VueTokenType::EqEq, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueTokenType::Eq, start_pos, start_pos + 1);
                }
            }
            '{' => {
                if state.peek_next_n(1) == Some('{') {
                    state.advance(2);
                    state.add_token(VueTokenType::InterpolationStart, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueTokenType::LeftBrace, start_pos, start_pos + 1);
                }
            }
            '}' => {
                if state.peek_next_n(1) == Some('}') {
                    state.advance(2);
                    state.add_token(VueTokenType::InterpolationEnd, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueTokenType::RightBrace, start_pos, start_pos + 1);
                }
            }
            '(' => {
                state.advance(1);
                state.add_token(VueTokenType::LeftParen, start_pos, start_pos + 1);
            }
            ')' => {
                state.advance(1);
                state.add_token(VueTokenType::RightParen, start_pos, start_pos + 1);
            }
            '[' => {
                state.advance(1);
                state.add_token(VueTokenType::LeftBracket, start_pos, start_pos + 1);
            }
            ']' => {
                state.advance(1);
                state.add_token(VueTokenType::RightBracket, start_pos, start_pos + 1);
            }
            ',' => {
                state.advance(1);
                state.add_token(VueTokenType::Comma, start_pos, start_pos + 1);
            }
            ':' => {
                state.advance(1);
                state.add_token(VueTokenType::Colon, start_pos, start_pos + 1);
            }
            ';' => {
                state.advance(1);
                state.add_token(VueTokenType::Semicolon, start_pos, start_pos + 1);
            }
            '.' => {
                state.advance(1);
                state.add_token(VueTokenType::Dot, start_pos, start_pos + 1);
            }
            '@' => {
                state.advance(1);
                state.add_token(VueTokenType::At, start_pos, start_pos + 1);
            }
            '#' => {
                state.advance(1);
                state.add_token(VueTokenType::Hash, start_pos, start_pos + 1);
            }
            '"' | '\'' => {
                self.lex_string(state);
            }
            _ if ch.is_ascii_digit() => {
                self.lex_number(state);
            }
            _ if is_ident_start(ch) => {
                self.lex_identifier(state);
            }
            _ => {
                self.lex_text(state);
            }
        }
    }

    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        while let Some(ch) = state.peek() {
            if ch == '<' || ch == '{' || ch.is_whitespace() {
                break;
            }
            state.advance(ch.len_utf8())
        }
        if state.get_position() > start_pos {
            state.add_token(VueTokenType::Text, start_pos, state.get_position())
        }
    }

    fn lex_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        let mut found = false;
        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
                found = true
            }
            else {
                break;
            }
        }
        if found {
            state.add_token(VueTokenType::Whitespace, start_pos, state.get_position())
        }
        found
    }

    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        if state.rest().starts_with("<!--") {
            state.advance(4);
            while let Some(c) = state.peek() {
                if state.rest().starts_with("-->") {
                    state.advance(3);
                    break;
                }
                state.advance(c.len_utf8())
            }
            state.add_token(VueTokenType::Comment, start_pos, state.get_position());
            return true;
        }
        // JS style comments
        if state.rest().starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }
            state.add_token(VueTokenType::Comment, start_pos, state.get_position());
            return true;
        }
        if state.rest().starts_with("/*") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if state.rest().starts_with("*/") {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8())
            }
            state.add_token(VueTokenType::Comment, start_pos, state.get_position());
            return true;
        }
        false
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        let Some(quote) = state.peek()
        else {
            return;
        };
        state.advance(1);
        let mut escaped = false;
        while let Some(ch) = state.peek() {
            if escaped {
                escaped = false;
                state.advance(ch.len_utf8())
            }
            else if ch == '\\' {
                escaped = true;
                state.advance(1)
            }
            else if ch == quote {
                state.advance(1);
                break;
            }
            else {
                state.advance(ch.len_utf8())
            }
        }
        state.add_token(VueTokenType::StringLiteral, start_pos, state.get_position())
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '.' { state.advance(1) } else { break }
        }
        state.add_token(VueTokenType::NumberLiteral, start_pos, state.get_position())
    }

    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        while let Some(ch) = state.peek() {
            if is_ident_continue(ch) { state.advance(ch.len_utf8()) } else { break }
        }
        let text = state.get_text_in((start_pos..state.get_position()).into());
        let kind = match text.as_ref() {
            "import" => VueTokenType::Import,
            "export" => VueTokenType::Export,
            "default" => VueTokenType::Default,
            "from" => VueTokenType::From,
            "as" => VueTokenType::As,
            "const" => VueTokenType::Const,
            "let" => VueTokenType::Let,
            "var" => VueTokenType::Var,
            "function" => VueTokenType::Function,
            "if" => VueTokenType::If,
            "else" => VueTokenType::Else,
            "while" => VueTokenType::While,
            "for" => VueTokenType::For,
            "return" => VueTokenType::Return,
            "break" => VueTokenType::Break,
            "continue" => VueTokenType::Continue,
            "switch" => VueTokenType::Switch,
            "try" => VueTokenType::Try,
            "throw" => VueTokenType::Throw,
            "in" => VueTokenType::In,
            "of" => VueTokenType::Of,
            "true" => VueTokenType::True,
            "false" => VueTokenType::False,
            "null" => VueTokenType::Null,
            _ => VueTokenType::Identifier,
        };
        state.add_token(kind, start_pos, state.get_position())
    }
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch == '$'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' || ch == '-'
}

impl<'config> Lexer<VueLanguage> for VueLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<VueLanguage>) -> LexOutput<VueLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);

        while state.not_at_end() {
            self.lex_token(&mut state)
        }

        state.add_eof();

        state.finish_with_cache(Ok(()), cache)
    }
}
