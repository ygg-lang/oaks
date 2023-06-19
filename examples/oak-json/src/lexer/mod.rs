use crate::{kind::JsonSyntaxKind, language::JsonLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, OakError, SourceText, lexer::LexOutput, source::Source};

type State<S> = LexerState<S, JsonLanguage>;

/// JSON 词法分析
#[derive(Clone)]
pub struct JsonLexer<'config> {
    config: &'config JsonLanguage,
}

impl<'config> JsonLexer<'config> {
    pub fn new(config: &'config JsonLanguage) -> Self {
        Self { config }
    }

    /// 为了向后兼容，提供tokenize_source 方法
    pub fn tokenize_source(&self, source: &SourceText) -> LexOutput<JsonLanguage> {
        self.lex(source)
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
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
            state.add_token(JsonSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(JsonSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(JsonSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        if !self.config.comments {
            return false;
        }

        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            // 检查下一个字符
            let remaining_text = state.get_text_in((start_pos..state.length()).into());
            if remaining_text.len() > 1 {
                let next_ch = remaining_text.chars().nth(1).unwrap();
                match next_ch {
                    '/' => {
                        // 单行注释
                        state.advance(2); // 跳过 '//'

                        // 读取到行
                        while let Some(ch) = state.peek() {
                            if ch == '\n' || ch == '\r' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }

                        state.add_token(JsonSyntaxKind::Comment, start_pos, state.get_position());
                        return true;
                    }
                    '*' => {
                        // 多行注释
                        state.advance(2); // 跳过 '/*'
                        let mut closed = false;

                        while let Some(ch) = state.peek() {
                            if ch == '*' {
                                let current_pos = state.get_position();
                                let remaining = state.get_text_in((current_pos..state.length()).into());
                                if remaining.len() > 1 && remaining.chars().nth(1) == Some('/') {
                                    state.advance(2); // 跳过 '*/'
                                    closed = true;
                                    break;
                                }
                            }
                            state.advance(ch.len_utf8());
                        }

                        if !closed {
                            // 未闭合的注释，添加错误但仍然创建 kind
                        }

                        state.add_token(JsonSyntaxKind::Comment, start_pos, state.get_position());
                        return true;
                    }
                    _ => {}
                }
            }
        }
        false
    }

    /// 处理字符串字面量
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        let quote_char = if let Some('"') = state.peek() {
            '"'
        }
        else if self.config.single_quotes && matches!(state.peek(), Some('\'')) {
            '\''
        }
        else {
            return false;
        };

        state.advance(quote_char.len_utf8()); // 跳过开始的引号
        let mut escaped = false;

        while let Some(ch) = state.peek() {
            if escaped {
                escaped = false;
                state.advance(ch.len_utf8());
            }
            else if ch == '\\' {
                escaped = true;
                state.advance(ch.len_utf8());
            }
            else if ch == quote_char {
                state.advance(ch.len_utf8()); // 跳过结束的引
                break;
            }
            else if ch == '\n' || ch == '\r' {
                // 字符串不能跨
                break;
            }
            else {
                state.advance(ch.len_utf8());
            }
        }

        state.add_token(JsonSyntaxKind::StringLiteral, start_pos, state.get_position());
        true
    }

    /// 处理数字字面
    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 处理负号
        if let Some('-') = state.peek() {
            state.advance(1);
        }

        let mut has_digits = false;

        // 处理十六进制数字（如果配置允许）
        if self.config.hex_numbers
            && let Some('0') = state.peek()
        {
            if let Some(next_ch) = state.peek_next_n(1) {
                if next_ch == 'x' || next_ch == 'X' {
                    state.advance(2); // 跳过 '0x'
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() {
                            has_digits = true;
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }

                    if has_digits {
                        state.add_token(JsonSyntaxKind::NumberLiteral, start_pos, state.get_position());
                        return true;
                    }
                    else {
                        // 回退到开始位
                        state.set_position(start_pos);
                        return false;
                    }
                }
            }
        }

        // 处理整数部分
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                has_digits = true;
                state.advance(1);
            }
            else {
                break;
            }
        }

        // 处理小数点和小数部分
        if let Some('.') = state.peek() {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    has_digits = true;
                    state.advance(1);
                }
                else {
                    break;
                }
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
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
            }
        }

        if has_digits && state.get_position() > start_pos {
            state.add_token(JsonSyntaxKind::NumberLiteral, start_pos, state.get_position());
            true
        }
        else {
            // 回退到开始位
            state.set_position(start_pos);
            false
        }
    }

    /// 处理布尔值和 null
    fn lex_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // Check for "true"
        if start_pos + 4 <= state.length() {
            let text = state.get_text_in((start_pos..start_pos + 4).into());
            if text == "true" {
                state.advance(4);
                state.add_token(JsonSyntaxKind::BooleanLiteral, start_pos, state.get_position());
                return true;
            }
        }

        // Check for "false"
        if start_pos + 5 <= state.length() {
            let text = state.get_text_in((start_pos..start_pos + 5).into());
            if text == "false" {
                state.advance(5);
                state.add_token(JsonSyntaxKind::BooleanLiteral, start_pos, state.get_position());
                return true;
            }
        }

        // Check for "null"
        if start_pos + 4 <= state.length() {
            let text = state.get_text_in((start_pos..start_pos + 4).into());
            if text == "null" {
                state.advance(4);
                state.add_token(JsonSyntaxKind::NullLiteral, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理裸键（JSON5 特性）
    fn lex_bare_key<S: Source>(&self, state: &mut State<S>) -> bool {
        if !self.config.bare_keys {
            return false;
        }

        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                // 继续读取标识符字
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(JsonSyntaxKind::BareKey, start_pos, state.get_position());
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

    /// 处理操作符和分隔
    fn lex_operator_or_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '{' => JsonSyntaxKind::LeftBrace,
                '}' => JsonSyntaxKind::RightBrace,
                '[' => JsonSyntaxKind::LeftBracket,
                ']' => JsonSyntaxKind::RightBracket,
                ',' => JsonSyntaxKind::Comma,
                ':' => JsonSyntaxKind::Colon,
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
}

impl<'config> Lexer<JsonLanguage> for JsonLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        _start_offset: usize,
        _cache: IncrementalCache<'_, JsonLanguage>,
    ) -> LexOutput<JsonLanguage> {
        let mut state = LexerState::new_with_cache(source, _start_offset, _cache);
        let result = self.run(&mut state);
        state.finish(result)
    }

    fn lex(&self, source: impl Source) -> LexOutput<JsonLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> JsonLexer<'config> {
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_keyword(state) {
                continue;
            }

            if self.lex_bare_key(state) {
                continue;
            }

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(JsonSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(JsonSyntaxKind::Eof, eof_pos, eof_pos);

        Ok(())
    }
}
