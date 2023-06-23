#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::JsonLanguage, lexer::token_type::JsonTokenType};
use oak_core::{
    errors::OakError,
    lexer::{CommentConfig, LexOutput, Lexer, LexerCache, LexerState, StringConfig},
    source::{Source, TextEdit},
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, JsonLanguage>;

static JSON_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: false });
static JSON_SINGLE_QUOTE_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

/// JSON 词法分析
#[derive(Clone)]
pub struct JsonLexer<'config> {
    config: &'config JsonLanguage,
}

impl<'config> Lexer<JsonLanguage> for JsonLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<JsonLanguage>) -> LexOutput<JsonLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> JsonLexer<'config> {
    pub fn new(config: &'config JsonLanguage) -> Self {
        Self { config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();
            let Some(ch) = state.peek()
            else {
                break;
            };

            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    self.skip_whitespace_fast(state);
                }
                '"' => {
                    self.lex_string_fast(state);
                }
                '/' if self.config.comments => {
                    JSON_COMMENT.scan(state, JsonTokenType::Comment, JsonTokenType::Comment);
                }
                '-' | '0'..='9' => {
                    self.lex_number(state);
                }
                '{' | '}' | '[' | ']' | ',' | ':' => {
                    self.lex_operator_or_delimiter(state);
                }
                't' | 'f' | 'n' => {
                    if !self.lex_keyword(state) {
                        if self.config.bare_keys {
                            self.lex_bare_key(state);
                        }
                    }
                }
                '\'' if self.config.single_quotes => {
                    JSON_SINGLE_QUOTE_STRING.scan(state, JsonTokenType::StringLiteral);
                }
                _ => {
                    let mut handled = false;
                    if self.config.bare_keys && (ch.is_alphabetic() || ch == '_' || ch == '$') {
                        handled = self.lex_bare_key(state);
                    }

                    if !handled {
                        // 如果所有规则都不匹配，跳过当前字符并标记为错误
                        state.advance(ch.len_utf8());
                        state.add_token(JsonTokenType::Error, safe_point, state.get_position());
                    }
                }
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 处理数字字面
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 处理负号
        state.consume_if_starts_with("-");

        let mut has_digits = false;

        // 处理十六进制数字（如果配置允许）
        if self.config.hex_numbers && state.starts_with("0") {
            let n1 = state.peek_next_n(1);
            if n1 == Some('x') || n1 == Some('X') {
                state.advance(2); // 跳过 '0x'
                let range = state.take_while(|c| c.is_ascii_hexdigit() || c == '_');
                if range.end > range.start {
                    state.add_token(JsonTokenType::NumberLiteral, start_pos, state.get_position());
                    return true;
                }
                // Fallback to decimal handling if no hex digits
            }
        }

        // 处理整数部分
        let r1 = state.take_while(|c| c.is_ascii_digit());
        if r1.end > r1.start {
            has_digits = true;
        }

        // 处理小数点和小数部分
        if state.consume_if_starts_with(".") {
            let r2 = state.take_while(|c| c.is_ascii_digit());
            if r2.end > r2.start {
                has_digits = true;
            }
        }

        // 处理科学计数
        if let Some(ch) = state.peek() {
            if ch == 'e' || ch == 'E' {
                state.advance(1);
                if let Some(sign) = state.peek() {
                    if sign == '+' || sign == '-' {
                        state.advance(1);
                    }
                }
                state.take_while(|c| c.is_ascii_digit());
            }
        }

        if has_digits {
            state.add_token(JsonTokenType::NumberLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理布尔值和 null
    fn lex_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        if state.consume_if_starts_with("true") || state.consume_if_starts_with("false") {
            state.add_token(JsonTokenType::BooleanLiteral, start_pos, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("null") {
            state.add_token(JsonTokenType::NullLiteral, start_pos, state.get_position());
            return true;
        }
        false
    }

    /// 处理裸键（JSON5 特性）
    fn lex_bare_key<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());
                state.take_while(|c| c.is_alphanumeric() || c == '_' || c == '$');
                state.add_token(JsonTokenType::BareKey, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理操作符和分隔
    fn lex_operator_or_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '{' => JsonTokenType::LeftBrace,
                '}' => JsonTokenType::RightBrace,
                '[' => JsonTokenType::LeftBracket,
                ']' => JsonTokenType::RightBracket,
                ',' => JsonTokenType::Comma,
                ':' => JsonTokenType::Colon,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_whitespace_fast<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        let mut count = 0;
        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
                count += 1;
            }
            else {
                break;
            }
        }
        if count > 0 {
            state.add_token(JsonTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_string_fast<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        if !state.consume_if_starts_with("\"") {
            return false;
        }

        let mut escaped = false;
        while let Some(ch) = state.peek() {
            state.advance(ch.len_utf8());
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == '"' {
                state.add_token(JsonTokenType::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }
        // 未闭合的字符串
        state.add_token(JsonTokenType::Error, start_pos, state.get_position());
        false
    }
}
