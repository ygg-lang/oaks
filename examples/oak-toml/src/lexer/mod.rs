use crate::{language::TomlLanguage, syntax::TomlSyntaxKind};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, TomlLanguage>;

pub struct TomlLexer<'config> {
    config: &'config TomlLanguage,
}

impl<'config> TomlLexer<'config> {
    pub fn new(config: &'config TomlLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符（不包括换行符）
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
            state.add_token(TomlSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(TomlSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(TomlSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(TomlSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                // 检查是否为多行字符串（三个引号
                let mut quote_count = 0;
                let mut temp_pos = state.get_position();

                // 计算连续的引号数
                while let Some(ch) = source.get_char_at(temp_pos) {
                    if ch == quote {
                        quote_count += 1;
                        temp_pos += 1;
                    }
                    else {
                        break;
                    }
                }

                if quote_count >= 3 {
                    // 多行字符
                    state.advance(3); // 跳过开始的三个引号

                    let mut found_end = false;
                    while let Some(ch) = state.peek() {
                        if ch == quote {
                            // 检查是否为结束的三个引
                            let mut end_quote_count = 0;
                            let mut check_pos = state.get_position();

                            while let Some(check_ch) = source.get_char_at(check_pos) {
                                if check_ch == quote {
                                    end_quote_count += 1;
                                    check_pos += 1;
                                }
                                else {
                                    break;
                                }
                            }

                            if end_quote_count >= 3 {
                                state.advance(3); // 跳过结束的三个引
                                found_end = true;
                                break;
                            }
                            else {
                                state.advance(1);
                            }
                        }
                        else if ch == '\\' && quote == '"' {
                            // 处理转义字符（仅在基本字符串中）
                            state.advance(1);
                            if let Some(_) = state.peek() {
                                state.advance(1);
                            }
                        }
                        else {
                            state.advance(ch.len_utf8());
                        }
                    }

                    let token_kind = if quote == '"' {
                        TomlSyntaxKind::MultilineBasicString
                    }
                    else {
                        TomlSyntaxKind::MultilineLiteralString
                    };

                    state.add_token(token_kind, start_pos, state.get_position());
                    true
                }
                else {
                    // 单行字符
                    state.advance(1); // 跳过开始引

                    while let Some(ch) = state.peek() {
                        if ch == quote {
                            state.advance(1); // 跳过结束引号
                            break;
                        }
                        else if ch == '\n' || ch == '\r' {
                            break; // 字符串不能跨
                        }
                        else if ch == '\\' && quote == '"' {
                            // 处理转义字符（仅在基本字符串中）
                            state.advance(1);
                            if let Some(_) = state.peek() {
                                state.advance(1);
                            }
                        }
                        else {
                            state.advance(ch.len_utf8());
                        }
                    }

                    let token_kind = if quote == '"' { TomlSyntaxKind::BasicString } else { TomlSyntaxKind::LiteralString };

                    state.add_token(token_kind, start_pos, state.get_position());
                    true
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

    /// 处理数字字面
    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();
        let mut is_float = false;

        // 处理符号
        if let Some(ch) = state.peek() {
            if ch == '+' || ch == '-' {
                state.advance(1);
            }
        }

        // 处理十六进制数字（如果允许）
        if self.config.allow_hex_numbers {
            if let Some('0') = state.peek() {
                let next_pos = state.get_position() + 1;
                if let Some('x') | Some('X') = source.get_char_at(next_pos) {
                    state.advance(2); // 跳过 "0x"

                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() || ch == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }

                    state.add_token(TomlSyntaxKind::Integer, start_pos, state.get_position());
                    return true;
                }
            }
        }

        // 处理十进制数
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // 处理小数
        if let Some('.') = state.peek() {
            let next_pos = state.get_position() + 1;
            if let Some(next_ch) = source.get_char_at(next_pos) {
                if next_ch.is_ascii_digit() {
                    is_float = true;
                    state.advance(1); // 小数

                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }

        // 处理科学计数
        if let Some('e') | Some('E') = state.peek() {
            is_float = true;
            state.advance(1);

            if let Some('+') | Some('-') = state.peek() {
                state.advance(1);
            }

            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() || ch == '_' {
                    state.advance(1);
                }
                else {
                    break;
                }
            }
        }

        let token_kind = if is_float { TomlSyntaxKind::Float } else { TomlSyntaxKind::Integer };

        state.add_token(token_kind, start_pos, state.get_position());
        true
    }

    /// 处理标识符或关键
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap();
                let token_kind = self.keyword_or_identifier(text);
                state.add_token(token_kind, start_pos, state.get_position());
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

    /// 判断是关键字还是标识
    fn keyword_or_identifier(&self, text: &str) -> TomlSyntaxKind {
        match text {
            "true" | "false" => TomlSyntaxKind::Boolean,
            _ if self.is_datetime_like(text) => TomlSyntaxKind::OffsetDateTime,
            _ => TomlSyntaxKind::BareKey,
        }
    }

    /// 检查文本是否像日期时间
    fn is_datetime_like(&self, text: &str) -> bool {
        // 简单的启发式：包含数字、破折号、冒号和 T
        text.chars().any(|c| c.is_ascii_digit()) && (text.contains('-') || text.contains(':') || text.contains('T'))
    }

    /// 处理分隔
    fn lex_delimiter(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '{' => TomlSyntaxKind::LeftBrace,
                '}' => TomlSyntaxKind::RightBrace,
                '[' => {
                    // 检查是否为双左括号
                    let next_pos = state.get_position() + 1;
                    if let Some('[') = source.get_char_at(next_pos) {
                        state.advance(2);
                        state.add_token(TomlSyntaxKind::DoubleLeftBracket, start_pos, state.get_position());
                        return true;
                    }
                    else {
                        TomlSyntaxKind::LeftBracket
                    }
                }
                ']' => {
                    // 检查是否为双右括号
                    let next_pos = state.get_position() + 1;
                    if let Some(']') = source.get_char_at(next_pos) {
                        state.advance(2);
                        state.add_token(TomlSyntaxKind::DoubleRightBracket, start_pos, state.get_position());
                        return true;
                    }
                    else {
                        TomlSyntaxKind::RightBracket
                    }
                }
                ',' => TomlSyntaxKind::Comma,
                '.' => TomlSyntaxKind::Dot,
                '=' => TomlSyntaxKind::Equal,
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

impl<'config> Lexer<TomlLanguage> for TomlLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<TomlSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_string(&mut state, source) {
                continue;
            }

            if let Some(ch) = state.peek() {
                if ch.is_ascii_digit() || ch == '+' || ch == '-' {
                    if self.lex_number(&mut state, source) {
                        continue;
                    }
                }
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_delimiter(&mut state, source) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(TomlSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(TomlSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
