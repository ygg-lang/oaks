use crate::{kind::ElixirSyntaxKind, language::ElixirLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState,
    errors::OakError,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'s, S> = LexerState<'s, S, ElixirLanguage>;

static ELIXIR_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static ELIXIR_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "#", block_start: "", block_end: "", nested_blocks: false });
static ELIXIR_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static ELIXIR_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: None });

#[derive(Clone)]
pub struct ElixirLexer<'config> {
    _config: &'config ElixirLanguage,
}

impl<'config> Lexer<ElixirLanguage> for ElixirLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<ElixirLanguage>) -> LexOutput<ElixirLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ElixirLexer<'config> {
    pub fn new(config: &'config ElixirLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
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

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        ELIXIR_WHITESPACE.scan(state, ElixirSyntaxKind::Whitespace)
    }

    fn skip_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        ELIXIR_COMMENT.scan(state, ElixirSyntaxKind::Comment, ElixirSyntaxKind::Comment)
    }

    fn lex_string_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        ELIXIR_STRING.scan(state, ElixirSyntaxKind::String)
    }

    fn lex_char_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        ELIXIR_CHAR.scan(state, ElixirSyntaxKind::Character)
    }

    fn lex_sigil<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();

        if state.consume_if_starts_with("~") {
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
                        state.take_while(|c| c.is_alphabetic());

                        state.add_token(ElixirSyntaxKind::Sigil, start, state.get_position());
                        return true;
                    }
                }
            }
        }
        false
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
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
                    state.take_while(|c| c.is_ascii_hexdigit() || c == '_');
                }
                Some('b') | Some('B') => {
                    state.advance(2);
                    state.take_while(|c| c == '0' || c == '1' || c == '_');
                }
                Some('o') | Some('O') => {
                    state.advance(2);
                    state.take_while(|c| ('0'..='7').contains(&c) || c == '_');
                }
                _ => {
                    state.advance(1);
                    state.take_while(|c| c.is_ascii_digit() || c == '_');
                }
            }
        }
        else {
            state.advance(1);
            state.take_while(|c| c.is_ascii_digit() || c == '_');
        }
        // fractional part
        if state.peek() == Some('.') {
            let n1 = state.peek_next_n(1);
            if n1.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_float = true;
                state.advance(1); // consume '.'
                state.take_while(|c| c.is_ascii_digit() || c == '_');
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
                    state.take_while(|d| d.is_ascii_digit() || d == '_');
                }
            }
        }
        // suffix letters
        state.take_while(|c| c.is_ascii_alphabetic());
        let end = state.get_position();
        state.add_token(if is_float { ElixirSyntaxKind::Float } else { ElixirSyntaxKind::Number }, start, end);
        true
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                state.take_while(|next_ch| next_ch.is_alphanumeric() || next_ch == '_' || next_ch == '?' || next_ch == '!');

                let text = state.get_text_in((start..state.get_position()).into());
                let kind = match text.as_ref() {
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
                        if text.as_ref().chars().next().unwrap().is_uppercase() {
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

    fn lex_atom<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();

        if state.consume_if_starts_with(":") {
            // 处理引用的原子 :"atom"
            if state.consume_if_starts_with("\"") {
                while let Some(ch) = state.peek() {
                    if ch == '"' {
                        state.advance(1);
                        break;
                    }
                    if state.consume_if_starts_with("\\") {
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
                    state.take_while(|next_ch| next_ch.is_alphanumeric() || next_ch == '_' || next_ch == '?' || next_ch == '!');
                }
            }

            state.add_token(ElixirSyntaxKind::Atom, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 多字符操作符
        let ops = [
            ("===", ElixirSyntaxKind::EqualEqualEqual),
            ("!==", ElixirSyntaxKind::NotEqualEqual),
            ("==", ElixirSyntaxKind::EqualEqual),
            ("!=", ElixirSyntaxKind::NotEqual),
            ("<=", ElixirSyntaxKind::LessEqual),
            (">=", ElixirSyntaxKind::GreaterEqual),
            ("++", ElixirSyntaxKind::PlusPlus),
            ("--", ElixirSyntaxKind::MinusMinus),
            ("**", ElixirSyntaxKind::StarStar),
            ("<<", ElixirSyntaxKind::LeftShift),
            (">>", ElixirSyntaxKind::RightShift),
            ("=~", ElixirSyntaxKind::MatchOp),
            ("|>", ElixirSyntaxKind::PipeRight),
            ("||", ElixirSyntaxKind::PipePipe),
            ("->", ElixirSyntaxKind::Arrow),
        ];

        for (pattern, kind) in ops {
            if state.consume_if_starts_with(pattern) {
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        // 单字符操作符
        if let Some(ch) = state.peek() {
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
