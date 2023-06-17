use crate::{kind::JsonSyntaxKind, language::JsonLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, JsonLanguage>;

/// JSON 词法分析
pub struct JsonLexer<'config> {
    config: &'config JsonLanguage,
}

impl<'config> JsonLexer<'config> {
    pub fn new(config: &'config JsonLanguage) -> Self {
        Self { config }
    }

    /// 为了向后兼容，提tokenize_source 方法
    pub fn tokenize_source(&self, source: &SourceText) -> LexOutput<JsonSyntaxKind> {
        self.lex(source)
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
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
    fn lex_newline(&self, state: &mut State) -> bool {
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
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        if !self.config.comments {
            return false;
        }

        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            // 检查下一个字
            if let Some(next_ch) = source.get_char_at(state.get_position() + 1) {
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
                        true
                    }
                    '*' => {
                        // 多行注释
                        state.advance(2); // 跳过 '/*'
                        let mut closed = false;

                        while let Some(ch) = state.peek() {
                            if ch == '*' {
                                if let Some('/') = source.get_char_at(state.get_position() + 1) {
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
                        true
                    }
                    _ => false,
                }
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string_literal(&self, state: &mut State) -> bool {
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
    fn lex_number(&self, state: &mut State) -> bool {
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
    fn lex_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();
        let remaining_text = source.get_text_at(start_pos).unwrap_or("");

        if remaining_text.starts_with("true") {
            // 确保后面不是标识符字
            if let Some(next_ch) = remaining_text.chars().nth(4) {
                if next_ch.is_alphanumeric() || next_ch == '_' {
                    return false;
                }
            }
            state.advance(4);
            state.add_token(JsonSyntaxKind::BooleanLiteral, start_pos, state.get_position());
            true
        }
        else if remaining_text.starts_with("false") {
            // 确保后面不是标识符字
            if let Some(next_ch) = remaining_text.chars().nth(5) {
                if next_ch.is_alphanumeric() || next_ch == '_' {
                    return false;
                }
            }
            state.advance(5);
            state.add_token(JsonSyntaxKind::BooleanLiteral, start_pos, state.get_position());
            true
        }
        else if remaining_text.starts_with("null") {
            // 确保后面不是标识符字
            if let Some(next_ch) = remaining_text.chars().nth(4) {
                if next_ch.is_alphanumeric() || next_ch == '_' {
                    return false;
                }
            }
            state.advance(4);
            state.add_token(JsonSyntaxKind::NullLiteral, start_pos, state.get_position());
            true
        }
        else if self.config.infinity_and_nan {
            if remaining_text.starts_with("Infinity") {
                if let Some(next_ch) = remaining_text.chars().nth(8) {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        return false;
                    }
                }
                state.advance(8);
                state.add_token(JsonSyntaxKind::NumberLiteral, start_pos, state.get_position());
                true
            }
            else if remaining_text.starts_with("NaN") {
                if let Some(next_ch) = remaining_text.chars().nth(3) {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        return false;
                    }
                }
                state.advance(3);
                state.add_token(JsonSyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    /// 处理裸键（JSON5 特性）
    fn lex_bare_key(&self, state: &mut State) -> bool {
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
    fn lex_operator_or_delimiter(&self, state: &mut State) -> bool {
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
    fn lex(&self, source: &SourceText) -> LexOutput<JsonSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state, source) {
                continue;
            }

            if self.lex_string_literal(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_keyword(&mut state, source) {
                continue;
            }

            if self.lex_bare_key(&mut state) {
                continue;
            }

            if self.lex_operator_or_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(JsonSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(JsonSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
