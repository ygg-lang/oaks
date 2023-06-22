use crate::kind::{VueLanguage, VueSyntaxKind};
use oak_core::{
    Lexer, LexerState,
    lexer::{LexOutput, LexerCache},
    source::Source,
};

pub struct VueLexer;

type State<'a, S> = LexerState<'a, S, VueLanguage>;

impl VueLexer {
    pub fn new() -> Self {
        Self
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
                state.add_token(VueSyntaxKind::Eof, start_pos, start_pos);
                return;
            }
        };

        match ch {
            '|' => {
                if state.peek_next_n(1) == Some('|') {
                    state.advance(2);
                    state.add_token(VueSyntaxKind::Or, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueSyntaxKind::Pipe, start_pos, start_pos + 1);
                }
            }
            '&' => {
                if state.peek_next_n(1) == Some('&') {
                    state.advance(2);
                    state.add_token(VueSyntaxKind::And, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueSyntaxKind::Amp, start_pos, start_pos + 1);
                }
            }
            '!' => {
                if state.peek_next_n(1) == Some('=') {
                    state.advance(2);
                    state.add_token(VueSyntaxKind::NotEq, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueSyntaxKind::Bang, start_pos, start_pos + 1);
                }
            }
            '+' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::Plus, start_pos, start_pos + 1);
            }
            '-' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::Minus, start_pos, start_pos + 1);
            }
            '*' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::Star, start_pos, start_pos + 1);
            }
            '%' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::Percent, start_pos, start_pos + 1);
            }
            '?' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::Question, start_pos, start_pos + 1);
            }
            '<' => {
                if state.rest().starts_with("</") {
                    state.advance(2);
                    state.add_token(VueSyntaxKind::LtSlash, start_pos, start_pos + 2);
                }
                else if state.peek_next_n(1) == Some('=') {
                    state.advance(2);
                    state.add_token(VueSyntaxKind::LtEq, start_pos, start_pos + 2);
                }
                else if state.rest().starts_with("<!--") {
                    state.advance(4);
                    while state.not_at_end() && !state.rest().starts_with("-->") {
                        state.advance(1);
                    }
                    if state.rest().starts_with("-->") {
                        state.advance(3);
                    }
                    state.add_token(VueSyntaxKind::Comment, start_pos, state.get_position());
                }
                else if state.rest().starts_with("<!") {
                    state.advance(2);
                    state.add_token(VueSyntaxKind::DocTypeStart, start_pos, start_pos + 2);
                }
                else if state.rest().starts_with("<script") && !is_ident_continue(state.peek_next_n(7).unwrap_or(' ')) {
                    state.advance(7);
                    state.add_token(VueSyntaxKind::ScriptStart, start_pos, start_pos + 7);
                }
                else if state.rest().starts_with("<style") && !is_ident_continue(state.peek_next_n(6).unwrap_or(' ')) {
                    state.advance(6);
                    state.add_token(VueSyntaxKind::StyleStart, start_pos, start_pos + 6);
                }
                else if state.rest().starts_with("<template") && !is_ident_continue(state.peek_next_n(9).unwrap_or(' ')) {
                    state.advance(9);
                    state.add_token(VueSyntaxKind::TemplateStart, start_pos, start_pos + 9);
                }
                else {
                    state.advance(1);
                    state.add_token(VueSyntaxKind::Lt, start_pos, start_pos + 1);
                }
            }
            '>' => {
                if state.peek_next_n(1) == Some('=') {
                    state.advance(2);
                    state.add_token(VueSyntaxKind::GtEq, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueSyntaxKind::Gt, start_pos, start_pos + 1);
                }
            }
            '/' => {
                if state.peek_next_n(1) == Some('>') {
                    state.advance(2);
                    state.add_token(VueSyntaxKind::SelfClosingEnd, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueSyntaxKind::Slash, start_pos, start_pos + 1);
                }
            }
            '=' => {
                if state.peek_next_n(1) == Some('>') {
                    state.advance(2);
                    state.add_token(VueSyntaxKind::Arrow, start_pos, start_pos + 2);
                }
                else if state.peek_next_n(1) == Some('=') {
                    state.advance(2);
                    state.add_token(VueSyntaxKind::EqEq, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueSyntaxKind::Eq, start_pos, start_pos + 1);
                }
            }
            '{' => {
                if state.peek_next_n(1) == Some('{') {
                    state.advance(2);
                    state.add_token(VueSyntaxKind::InterpolationStart, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueSyntaxKind::LeftBrace, start_pos, start_pos + 1);
                }
            }
            '}' => {
                if state.peek_next_n(1) == Some('}') {
                    state.advance(2);
                    state.add_token(VueSyntaxKind::InterpolationEnd, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(VueSyntaxKind::RightBrace, start_pos, start_pos + 1);
                }
            }
            '(' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::LeftParen, start_pos, start_pos + 1);
            }
            ')' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::RightParen, start_pos, start_pos + 1);
            }
            '[' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::LeftBracket, start_pos, start_pos + 1);
            }
            ']' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::RightBracket, start_pos, start_pos + 1);
            }
            ',' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::Comma, start_pos, start_pos + 1);
            }
            ':' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::Colon, start_pos, start_pos + 1);
            }
            ';' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::Semicolon, start_pos, start_pos + 1);
            }
            '.' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::Dot, start_pos, start_pos + 1);
            }
            '@' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::At, start_pos, start_pos + 1);
            }
            '#' => {
                state.advance(1);
                state.add_token(VueSyntaxKind::Hash, start_pos, start_pos + 1);
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
            state.advance(ch.len_utf8());
        }
        if state.get_position() > start_pos {
            state.add_token(VueSyntaxKind::Text, start_pos, state.get_position());
        }
    }

    fn lex_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        let mut found = false;
        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
                found = true;
            }
            else {
                break;
            }
        }
        if found {
            state.add_token(VueSyntaxKind::Whitespace, start_pos, state.get_position());
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
                state.advance(c.len_utf8());
            }
            state.add_token(VueSyntaxKind::Comment, start_pos, state.get_position());
            return true;
        }
        // JS style comments
        if state.rest().starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(VueSyntaxKind::Comment, start_pos, state.get_position());
            return true;
        }
        if state.rest().starts_with("/*") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if state.rest().starts_with("*/") {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(VueSyntaxKind::Comment, start_pos, state.get_position());
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
                state.advance(ch.len_utf8());
            }
            else if ch == '\\' {
                escaped = true;
                state.advance(1);
            }
            else if ch == quote {
                state.advance(1);
                break;
            }
            else {
                state.advance(ch.len_utf8());
            }
        }
        state.add_token(VueSyntaxKind::StringLiteral, start_pos, state.get_position());
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                state.advance(1);
            }
            else {
                break;
            }
        }
        state.add_token(VueSyntaxKind::NumberLiteral, start_pos, state.get_position());
    }

    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        while let Some(ch) = state.peek() {
            if is_ident_continue(ch) {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }
        let text = state.get_text_in((start_pos..state.get_position()).into());
        let kind = match text.as_ref() {
            "import" => VueSyntaxKind::Import,
            "export" => VueSyntaxKind::Export,
            "default" => VueSyntaxKind::Default,
            "from" => VueSyntaxKind::From,
            "as" => VueSyntaxKind::As,
            "const" => VueSyntaxKind::Const,
            "let" => VueSyntaxKind::Let,
            "var" => VueSyntaxKind::Var,
            "function" => VueSyntaxKind::Function,
            "if" => VueSyntaxKind::If,
            "else" => VueSyntaxKind::Else,
            "while" => VueSyntaxKind::While,
            "for" => VueSyntaxKind::For,
            "return" => VueSyntaxKind::Return,
            "break" => VueSyntaxKind::Break,
            "continue" => VueSyntaxKind::Continue,
            "switch" => VueSyntaxKind::Switch,
            "try" => VueSyntaxKind::Try,
            "throw" => VueSyntaxKind::Throw,
            "in" => VueSyntaxKind::In,
            "of" => VueSyntaxKind::Of,
            "true" => VueSyntaxKind::True,
            "false" => VueSyntaxKind::False,
            "null" => VueSyntaxKind::Null,
            _ => VueSyntaxKind::Identifier,
        };
        state.add_token(kind, start_pos, state.get_position());
    }
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch == '$'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' || ch == '-'
}

impl Lexer<VueLanguage> for VueLexer {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<VueLanguage>) -> LexOutput<VueLanguage> {
        let mut state = State::new(source);

        while state.not_at_end() {
            self.lex_token(&mut state);
        }

        let pos = state.get_position();
        state.add_token(VueSyntaxKind::Eof, pos, pos);

        state.finish_with_cache(Ok(()), cache)
    }
}
