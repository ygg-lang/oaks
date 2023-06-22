use crate::{kind::StylusSyntaxKind, language::StylusLanguage};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, StylusLanguage>;

#[derive(Clone, Debug)]
pub struct StylusLexer<'config> {
    config: &'config StylusLanguage,
}

impl<'config> StylusLexer<'config> {
    pub fn new(config: &'config StylusLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符（不包括换行符）
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
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
            state.add_token(StylusSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(StylusSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(StylusSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
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

            state.add_token(StylusSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                // 检查是否为多行字符串（三个引号）
                let mut quote_count = 0;

                // 计算连续的引号数
                while let Some(ch) = state.peek_next_n(quote_count) {
                    if ch == quote {
                        quote_count += 1;
                    }
                    else {
                        break;
                    }
                }

                if quote_count >= 3 {
                    // 多行字符串
                    state.advance(3); // 跳过开始的三个引号

                    while let Some(ch) = state.peek() {
                        if ch == quote {
                            // 检查是否为结束的三个引号
                            let mut end_quote_count = 0;

                            while let Some(check_ch) = state.peek_next_n(end_quote_count) {
                                if check_ch == quote {
                                    end_quote_count += 1;
                                }
                                else {
                                    break;
                                }
                            }

                            if end_quote_count >= 3 {
                                state.advance(3); // 跳过结束的三个引号
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

                    state.add_token(StylusSyntaxKind::String, start_pos, state.get_position());
                    true
                }
                else {
                    // 单行字符串
                    state.advance(1); // 跳过开始引号

                    while let Some(ch) = state.peek() {
                        if ch == quote {
                            state.advance(1); // 跳过结束引号
                            break;
                        }
                        else if ch == '\n' || ch == '\r' {
                            break; // 字符串不能跨行
                        }
                        else if ch == '\\' && quote == '"' {
                            // 处理转义字符（仅在双引号字符串中）
                            state.advance(1);
                            if let Some(_) = state.peek() {
                                state.advance(1);
                            }
                        }
                        else {
                            state.advance(ch.len_utf8());
                        }
                    }

                    state.add_token(StylusSyntaxKind::String, start_pos, state.get_position());
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

    /// 处理数字字面量
    fn lex_number<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
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
            if state.peek() == Some('0') {
                if let Some('x') | Some('X') = state.peek_next_n(1) {
                    state.advance(2); // 跳过 "0x"

                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() || ch == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }

                    state.add_token(StylusSyntaxKind::Number, start_pos, state.get_position());
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
            if let Some(next_ch) = state.peek_next_n(1) {
                if next_ch.is_ascii_digit() {
                    is_float = true;
                    state.advance(1); // 小数点

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

        let token_kind = if is_float { StylusSyntaxKind::Number } else { StylusSyntaxKind::Number };

        state.add_token(token_kind, start_pos, state.get_position());
        true
    }

    /// 处理标识符或关键字
    fn lex_identifier_or_keyword<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
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

                let end_pos = state.get_position();
                let text = state.get_text_in(oak_core::Range { start: start_pos, end: end_pos });
                let token_kind = self.keyword_or_identifier(&text);
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

    /// 判断是关键字还是标识符
    fn keyword_or_identifier(&self, text: &str) -> StylusSyntaxKind {
        match text {
            // CSS 颜色关键字
            "red" | "blue" | "green" | "white" | "black" | "transparent" => StylusSyntaxKind::Color,
            // 其他都是标识符
            _ => StylusSyntaxKind::Identifier,
        }
    }

    /// 处理分隔符和操作符
    fn lex_delimiter<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '{' => StylusSyntaxKind::LeftBrace,
                '}' => StylusSyntaxKind::RightBrace,
                '(' => StylusSyntaxKind::LeftParen,
                ')' => StylusSyntaxKind::RightParen,
                ':' => StylusSyntaxKind::Colon,
                ';' => StylusSyntaxKind::Semicolon,
                ',' => StylusSyntaxKind::Comma,
                '.' => StylusSyntaxKind::Dot,
                '#' => StylusSyntaxKind::Hash,
                '&' => StylusSyntaxKind::Ampersand,
                '+' => StylusSyntaxKind::Plus,
                '-' => StylusSyntaxKind::Minus,
                '*' => StylusSyntaxKind::Star,
                '/' => StylusSyntaxKind::Slash,
                '%' => StylusSyntaxKind::Percent,
                '=' => StylusSyntaxKind::Equal,
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

impl<'config> StylusLexer<'config> {
    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

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

            if self.lex_string(state) {
                continue;
            }

            if let Some(ch) = state.peek() {
                if ch.is_ascii_digit() || ch == '+' || ch == '-' {
                    if self.lex_number(state) {
                        continue;
                    }
                }
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(StylusSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        state.add_eof();
        Ok(())
    }
}

impl<'config> Lexer<StylusLanguage> for StylusLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<StylusLanguage>) -> LexOutput<StylusLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}
