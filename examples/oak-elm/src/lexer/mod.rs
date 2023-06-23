#![doc = include_str!("readme.md")]
use oak_core::Source;
pub mod token_type;

use crate::{language::ElmLanguage, lexer::token_type::ElmTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState,
    errors::OakError,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
};
use std::sync::LazyLock;

type State<'s, S> = LexerState<'s, S, ElmLanguage>;

static ELM_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static ELM_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "--", block_start: "{-", block_end: "-}", nested_blocks: true });
static ELM_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static ELM_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone, Debug)]
pub struct ElmLexer<'config> {
    config: &'config ElmLanguage,
}

impl<'config> Lexer<ElmLanguage> for ElmLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<ElmLanguage>) -> LexOutput<ElmLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ElmLexer<'config> {
    pub fn new(config: &'config ElmLanguage) -> Self {
        Self { config }
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

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
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
        ELM_WHITESPACE.scan(state, ElmTokenType::Whitespace)
    }

    fn skip_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        ELM_COMMENT.scan(state, ElmTokenType::Comment, ElmTokenType::Comment)
    }

    fn lex_string_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        ELM_STRING.scan(state, ElmTokenType::String)
    }

    fn lex_char_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        ELM_CHAR.scan(state, ElmTokenType::Char)
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

        state.advance(1);
        state.take_while(|c| c.is_ascii_digit() || c == '_');

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

        let end = state.get_position();
        state.add_token(if is_float { ElmTokenType::Float } else { ElmTokenType::Number }, start, end);
        true
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                state.take_while(|next_ch| next_ch.is_alphanumeric() || next_ch == '_');

                let text = state.get_text_in((start..state.get_position()).into());
                let kind = match text.as_ref() {
                    "if" => ElmTokenType::If,
                    "then" => ElmTokenType::Then,
                    "else" => ElmTokenType::Else,
                    "case" => ElmTokenType::Case,
                    "of" => ElmTokenType::Of,
                    "let" => ElmTokenType::Let,
                    "in" => ElmTokenType::In,
                    "type" => ElmTokenType::Type,
                    "alias" => ElmTokenType::Alias,
                    "module" => ElmTokenType::Module,
                    "where" => ElmTokenType::Where,
                    "import" => ElmTokenType::Import,
                    "exposing" => ElmTokenType::Exposing,
                    "as" => ElmTokenType::As,
                    "port" => ElmTokenType::Port,
                    _ => ElmTokenType::Identifier,
                };

                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 多字符操作符
        let ops = [
            ("==", ElmTokenType::EqualEqual),
            ("/=", ElmTokenType::NotEqual),
            ("<=", ElmTokenType::LessEqual),
            (">=", ElmTokenType::GreaterEqual),
            ("&&", ElmTokenType::DoubleAmpersand),
            ("||", ElmTokenType::DoublePipe),
            ("++", ElmTokenType::DoublePlus),
            ("<<", ElmTokenType::DoubleLess),
            (">>", ElmTokenType::DoubleGreater),
            ("|>", ElmTokenType::PipeGreater),
            ("->", ElmTokenType::Arrow),
            ("...", ElmTokenType::TripleDot),
            ("..", ElmTokenType::DoubleDot),
            ("//", ElmTokenType::DoubleSlash),
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
                '+' => ElmTokenType::Plus,
                '-' => ElmTokenType::Minus,
                '*' => ElmTokenType::Star,
                '/' => ElmTokenType::Slash,
                '=' => ElmTokenType::Equal,
                '<' => ElmTokenType::Less,
                '>' => ElmTokenType::Greater,
                '^' => ElmTokenType::Caret,
                '|' => ElmTokenType::Pipe,
                '(' => ElmTokenType::LeftParen,
                ')' => ElmTokenType::RightParen,
                '{' => ElmTokenType::LeftBrace,
                '}' => ElmTokenType::RightBrace,
                '[' => ElmTokenType::LeftBracket,
                ']' => ElmTokenType::RightBracket,
                ',' => ElmTokenType::Comma,
                ';' => ElmTokenType::Semicolon,
                '.' => ElmTokenType::Dot,
                ':' => ElmTokenType::Colon,
                '\\' => ElmTokenType::Backslash,
                '%' => ElmTokenType::Percent,
                '\n' => ElmTokenType::Newline,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }
}
