#![doc = include_str!("readme.md")]
pub mod token_type;

pub use token_type::JTokenType;

use crate::language::JLanguage;
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, JLanguage>;

static J_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

#[derive(Clone, Debug)]
pub struct JLexer<'config> {
    config: &'config JLanguage,
}

impl<'config> Lexer<JLanguage> for JLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<JLanguage>) -> LexOutput<JLanguage> {
        let mut state: State<'_, S> = LexerState::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> JLexer<'config> {
    pub fn new(config: &'config JLanguage) -> Self {
        Self { config }
    }

    /// 主要词法分析逻辑
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
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

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            // 如果没有匹配任何模式，跳过当前字符并生成 Error token
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(JTokenType::Error, safe_point, state.get_position());
            }
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        J_WHITESPACE.scan(state, JTokenType::Whitespace)
    }

    /// J 语言的注释以 NB. 开头
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with("NB.") {
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(JTokenType::Comment, start, state.get_position());
            return true;
        }
        false
    }

    /// 字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with("'") {
            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(ch.len_utf8());
                    // 处理转义的单引号 ''
                    if state.consume_if_starts_with("'") {
                        continue;
                    }
                    state.add_token(JTokenType::StringLiteral, start, state.get_position());
                    return true;
                }
                state.advance(ch.len_utf8());
            }
            // 未闭合的字符串
            state.add_token(JTokenType::Error, start, state.get_position());
            return true;
        }
        false
    }

    /// 数字字面量
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '_' {
                // J 使用 _ 表示负号
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == 'e' || ch == 'E' || ch == 'j' || ch == 'r' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(JTokenType::NumberLiteral, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 标识符
    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(JTokenType::Identifier, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 操作符和特殊符号
    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 尝试匹配长的操作符
        for (op, token) in [("=:", JTokenType::IsGlobal), ("=.", JTokenType::IsLocal)] {
            if state.consume_if_starts_with(op) {
                state.add_token(token, start, state.get_position());
                return true;
            }
        }

        // 匹配单个字符操作符
        if let Some(ch) = state.peek() {
            let token = match ch {
                '=' => Some(JTokenType::Equal),
                '.' => Some(JTokenType::Dot),
                ':' => Some(JTokenType::Colon),
                '+' => Some(JTokenType::Plus),
                '-' => Some(JTokenType::Minus),
                '*' => Some(JTokenType::Star),
                '%' => Some(JTokenType::Percent),
                '$' => Some(JTokenType::Dollar),
                ',' => Some(JTokenType::Comma),
                '#' => Some(JTokenType::Hash),
                '/' => Some(JTokenType::Slash),
                '\\' => Some(JTokenType::Backslash),
                '|' => Some(JTokenType::Pipe),
                '&' => Some(JTokenType::Ampersand),
                '^' => Some(JTokenType::Caret),
                '~' => Some(JTokenType::Tilde),
                '<' => Some(JTokenType::Less),
                '>' => Some(JTokenType::Greater),
                '(' => Some(JTokenType::LeftParen),
                ')' => Some(JTokenType::RightParen),
                '[' => Some(JTokenType::LeftBracket),
                ']' => Some(JTokenType::RightBracket),
                '{' => Some(JTokenType::LeftBrace),
                '}' => Some(JTokenType::RightBrace),
                _ => None,
            };

            if let Some(token) = token {
                state.advance(ch.len_utf8());
                state.add_token(token, start, state.get_position());
                return true;
            }
        }

        false
    }
}
