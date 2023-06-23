#![doc = include_str!("readme.md")]
pub mod token_type;
use crate::language::CsvLanguage;
use oak_core::{Lexer, LexerState, OakError, lexer::LexOutput, source::Source};
pub use token_type::CsvTokenType;

type State<'a, S> = LexerState<'a, S, CsvLanguage>;

#[derive(Clone)]
pub struct CsvLexer<'config> {
    _config: &'config CsvLanguage,
    field_separator: char,
    quote_char: char,
}

impl<'config> Lexer<CsvLanguage> for CsvLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl oak_core::LexerCache<CsvLanguage>) -> LexOutput<CsvLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> CsvLexer<'config> {
    pub fn new(config: &'config CsvLanguage) -> Self {
        Self { _config: config, field_separator: ',', quote_char: '"' }
    }

    pub fn with_separator(mut self, separator: char) -> Self {
        self.field_separator = separator;
        self
    }

    pub fn with_quote_char(mut self, quote: char) -> Self {
        self.quote_char = quote;
        self
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        let mut found_whitespace = false;

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
                found_whitespace = true
            }
            else {
                break;
            }
        }

        if found_whitespace {
            state.add_token(CsvTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '\r' {
                state.advance(1);
                // 检查是否是 CRLF
                if state.peek() == Some('\n') {
                    state.advance(1)
                }
                state.add_token(CsvTokenType::Newline, start_pos, state.get_position());
                true
            }
            else if ch == '\n' {
                state.advance(1);
                state.add_token(CsvTokenType::Newline, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理带引号的字段
    fn lex_quoted_field<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == self.quote_char {
                state.advance(ch.len_utf8()); // 跳过开始引号
                while let Some(ch) = state.peek() {
                    if ch == self.quote_char {
                        state.advance(ch.len_utf8());
                        // 检查是否是转义引号（双引号）
                        if state.peek() == Some(self.quote_char) {
                            state.advance(self.quote_char.len_utf8()); // 跳过转义引号
                        }
                        else {
                            // 结束引号
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }
                state.add_token(CsvTokenType::Field, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理不带引号的字段
    fn lex_unquoted_field<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        let mut found_char = false;

        while let Some(ch) = state.peek() {
            if ch == self.field_separator || ch == '\n' || ch == '\r' {
                break;
            }
            else {
                state.advance(ch.len_utf8());
                found_char = true
            }
        }

        if found_char {
            state.add_token(CsvTokenType::Field, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理逗号
    fn lex_comma<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == self.field_separator {
                state.advance(ch.len_utf8());
                state.add_token(CsvTokenType::Comma, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comma(state) {
                continue;
            }

            if self.lex_quoted_field(state) {
                continue;
            }

            if self.lex_unquoted_field(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(CsvTokenType::Error, start_pos, state.get_position())
            }
        }
        Ok(())
    }
}
