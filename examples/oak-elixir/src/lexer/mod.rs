use crate::{kind::ElixirSyntaxKind, language::ElixirLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, ElixirLanguage>;

static ELIXIR_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static ELIXIR_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["#"] });
static ELIXIR_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static ELIXIR_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: None });

#[derive(Clone)]
pub struct ElixirLexer<'config> {
    config: &'config ElixirLanguage,
}

impl<'config> Lexer<ElixirLanguage> for ElixirLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<ElixirLanguage>,
    ) -> LexOutput<ElixirLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> ElixirLexer<'config> {
    pub fn new(config: &'config ElixirLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_char_literal(state) {
                continue;
            }

            if self.lex_sigil(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_atom(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(ElixirSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match ELIXIR_WHITESPACE.scan(state.rest(), state.get_position(), ElixirSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        match ELIXIR_COMMENT.scan(state.rest(), state.get_position(), ElixirSyntaxKind::Comment) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let _start = state.get_position();
        match ELIXIR_STRING.scan(state.rest(), state.get_position(), ElixirSyntaxKind::String) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn lex_char_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let _start = state.get_position();
        match ELIXIR_CHAR.scan(state.rest(), state.get_position(), ElixirSyntaxKind::Character) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn lex_sigil<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        if rest.starts_with("~") {
            state.advance(1);
            if let Some(sigil_type) = state.peek() {
                if sigil_type.is_alphabetic() {
                    state.advance(sigil_type.len_utf8());

                    // 查找分隔符
                    if let Some(delimiter) = state.peek() {
                        let closing_delimiter = match delimiter {
                            '(' => ')',
                            '[' => ']',
                            '{' => '}',
                            '<' => '>',
                            '/' => '/',
                            '|' => '|',
                            '"' => '"',
                            '\'' => '\'',
                            _ => delimiter,
                        };

                        state.advance(delimiter.len_utf8());

                        while let Some(ch) = state.peek() {
                            if ch == closing_delimiter {
                                state.advance(ch.len_utf8());
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }

                        // 可选的修饰符
                        while let Some(ch) = state.peek() {
                            if ch.is_alphabetic() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }

                        state.add_token(ElixirSyntaxKind::Sigil, start, state.get_position());
                        return true;
                    }
                }
            }
        }
        false
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };
        if !first.is_ascii_digit() {
            return false;
        }
        let mut is_float = false;
        if first == '0' {
            match state.peek_next_n(1) {
                Some('x') | Some('X') => {
                    state.advance(2);
                    while let Some(c) = state.peek() {
                        if c.is_ascii_hexdigit() || c == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                Some('b') | Some('B') => {
                    state.advance(2);
                    while let Some(c) = state.peek() {
                        if c == '0' || c == '1' || c == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                Some('o') | Some('O') => {
                    state.advance(2);
                    while let Some(c) = state.peek() {
                        if ('0'..='7').contains(&c) || c == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                _ => {
                    state.advance(1);
                    while let Some(c) = state.peek() {
                        if c.is_ascii_digit() || c == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }
        else {
            state.advance(1);
            while let Some(c) = state.peek() {
                if c.is_ascii_digit() || c == '_' {
                    state.advance(1);
                }
                else {
                    break;
                }
            }
        }
        // fractional part
        if state.peek() == Some('.') {
            let n1 = state.peek_next_n(1);
            if n1.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_float = true;
                state.advance(1); // consume '.'
                while let Some(c) = state.peek() {
                    if c.is_ascii_digit() || c == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
            }
        }
        // exponent
        if let Some(c) = state.peek() {
            if c == 'e' || c == 'E' {
                let n1 = state.peek_next_n(1);
                if n1 == Some('+') || n1 == Some('-') || n1.map(|d| d.is_ascii_digit()).unwrap_or(false) {
                    is_float = true;
                    state.advance(1);
                    if let Some(sign) = state.peek() {
                        if sign == '+' || sign == '-' {
                            state.advance(1);
                        }
                    }
                    while let Some(d) = state.peek() {
                        if d.is_ascii_digit() || d == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }
        // suffix letters (Elixir does not have explicit number suffixes like Rust, but we keep the structure for consistency if needed later)
        while let Some(c) = state.peek() {
            if c.is_ascii_alphabetic() {
                state.advance(1);
            }
            else {
                break;
            }
        }
        let end = state.get_position();
        state.add_token(if is_float { ElixirSyntaxKind::Float } else { ElixirSyntaxKind::Number }, start, end);
        true
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(next_ch) = state.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' || next_ch == '?' || next_ch == '!' {
                        state.advance(next_ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start..state.get_position()).into());
                let kind = match text {
                    "after" => ElixirSyntaxKind::After,
                    "and" => ElixirSyntaxKind::And,
                    "case" => ElixirSyntaxKind::Case,
                    "catch" => ElixirSyntaxKind::Catch,
                    "cond" => ElixirSyntaxKind::Cond,
                    "def" => ElixirSyntaxKind::Def,
                    "defp" => ElixirSyntaxKind::Defp,
                    "defmodule" => ElixirSyntaxKind::Defmodule,
                    "defstruct" => ElixirSyntaxKind::Defstruct,
                    "defprotocol" => ElixirSyntaxKind::Defprotocol,
                    "defimpl" => ElixirSyntaxKind::Defimpl,
                    "defmacro" => ElixirSyntaxKind::Defmacro,
                    "defmacrop" => ElixirSyntaxKind::Defmacrop,
                    "do" => ElixirSyntaxKind::Do,
                    "else" => ElixirSyntaxKind::Else,
                    "elsif" => ElixirSyntaxKind::Elsif,
                    "end" => ElixirSyntaxKind::End,
                    "false" => ElixirSyntaxKind::False,
                    "fn" => ElixirSyntaxKind::Fn,
                    "if" => ElixirSyntaxKind::If,
                    "in" => ElixirSyntaxKind::In,
                    "not" => ElixirSyntaxKind::Not,
                    "or" => ElixirSyntaxKind::Or,
                    "receive" => ElixirSyntaxKind::Receive,
                    "rescue" => ElixirSyntaxKind::Rescue,
                    "true" => ElixirSyntaxKind::True,
                    "try" => ElixirSyntaxKind::Try,
                    "unless" => ElixirSyntaxKind::Unless,
                    "when" => ElixirSyntaxKind::When,
                    "with" => ElixirSyntaxKind::With,
                    _ => {
                        if text.chars().next().unwrap().is_uppercase() {
                            ElixirSyntaxKind::Variable
                        }
                        else {
                            ElixirSyntaxKind::Identifier
                        }
                    }
                };

                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_atom<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if state.current() == Some(':') {
            state.advance(1);

            // 处理引用的原子 :"atom"
            if state.peek() == Some('"') {
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '"' {
                        state.advance(1);
                        break;
                    }
                    if ch == '\\' {
                        state.advance(1);
                        if let Some(escaped) = state.peek() {
                            state.advance(escaped.len_utf8());
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
            }
            else if let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch == '_' {
                    state.advance(ch.len_utf8());
                    while let Some(next_ch) = state.peek() {
                        if next_ch.is_alphanumeric() || next_ch == '_' || next_ch == '?' || next_ch == '!' {
                            state.advance(next_ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }
            }

            state.add_token(ElixirSyntaxKind::Atom, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 多字符操作符
        if rest.starts_with("===") {
            state.advance(3);
            state.add_token(ElixirSyntaxKind::EqualEqualEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with("!==") {
            state.advance(3);
            state.add_token(ElixirSyntaxKind::NotEqualEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with("==") {
            state.advance(2);
            state.add_token(ElixirSyntaxKind::EqualEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with("!=") {
            state.advance(2);
            state.add_token(ElixirSyntaxKind::NotEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with("<=") {
            state.advance(2);
            state.add_token(ElixirSyntaxKind::LessEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with(">=") {
            state.advance(2);
            state.add_token(ElixirSyntaxKind::GreaterEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with("++") {
            state.advance(2);
            state.add_token(ElixirSyntaxKind::PlusPlus, start, state.get_position());
            return true;
        }
        if rest.starts_with("--") {
            state.advance(2);
            state.add_token(ElixirSyntaxKind::MinusMinus, start, state.get_position());
            return true;
        }
        if rest.starts_with("**") {
            state.advance(2);
            state.add_token(ElixirSyntaxKind::StarStar, start, state.get_position());
            return true;
        }
        if rest.starts_with("<<") {
            state.advance(2);
            state.add_token(ElixirSyntaxKind::LeftShift, start, state.get_position());
            return true;
        }
        if rest.starts_with(">>") {
            state.advance(2);
            state.add_token(ElixirSyntaxKind::RightShift, start, state.get_position());
            return true;
        }
        if rest.starts_with("=~") {
            state.advance(2);
            state.add_token(ElixirSyntaxKind::MatchOp, start, state.get_position());
            return true;
        }
        if rest.starts_with("|>") {
            state.advance(2);
            state.add_token(ElixirSyntaxKind::PipeRight, start, state.get_position());
            return true;
        }
        if rest.starts_with("||") {
            state.advance(2);
            state.add_token(ElixirSyntaxKind::PipePipe, start, state.get_position());
            return true;
        }
        if rest.starts_with("->") {
            state.advance(2);
            state.add_token(ElixirSyntaxKind::Arrow, start, state.get_position());
            return true;
        }

        // 单字符操作符
        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => ElixirSyntaxKind::Plus,
                '-' => ElixirSyntaxKind::Minus,
                '*' => ElixirSyntaxKind::Star,
                '/' => ElixirSyntaxKind::Slash,
                '=' => ElixirSyntaxKind::Equal,
                '<' => ElixirSyntaxKind::Less,
                '>' => ElixirSyntaxKind::Greater,
                '!' => ElixirSyntaxKind::Exclamation,
                '?' => ElixirSyntaxKind::Question,
                '&' => ElixirSyntaxKind::Ampersand,
                '@' => ElixirSyntaxKind::At,
                '^' => ElixirSyntaxKind::Caret,
                '~' => ElixirSyntaxKind::Tilde,
                '|' => ElixirSyntaxKind::Pipe,
                '#' => ElixirSyntaxKind::Hash,
                '(' => ElixirSyntaxKind::LeftParen,
                ')' => ElixirSyntaxKind::RightParen,
                '{' => ElixirSyntaxKind::LeftBrace,
                '}' => ElixirSyntaxKind::RightBrace,
                '[' => ElixirSyntaxKind::LeftBracket,
                ']' => ElixirSyntaxKind::RightBracket,
                ',' => ElixirSyntaxKind::Comma,
                ';' => ElixirSyntaxKind::Semicolon,
                '.' => ElixirSyntaxKind::Dot,
                ':' => ElixirSyntaxKind::Colon,
                '\n' => ElixirSyntaxKind::Newline,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }
}
