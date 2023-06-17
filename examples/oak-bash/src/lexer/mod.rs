use crate::{kind::BashSyntaxKind, language::BashLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, BashLanguage>;

pub struct BashLexer<'config> {
    config: &'config BashLanguage,
}

impl<'config> BashLexer<'config> {
    pub fn new(config: &'config BashLanguage) -> Self {
        Self { config }
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
            state.add_token(BashSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(BashSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(BashSyntaxKind::Newline, start_pos, state.get_position());
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

            // 读取到行
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(BashSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                let mut escaped = false;
                while let Some(ch) = state.peek() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8());
                        continue;
                    }

                    if ch == '\\' && quote == '"' {
                        escaped = true;
                        state.advance(1);
                        continue;
                    }

                    if ch == quote {
                        state.advance(1);
                        break;
                    }

                    if ch == '\n' || ch == '\r' {
                        break; // 未闭合的字符
                    }

                    state.advance(ch.len_utf8());
                }

                state.add_token(BashSyntaxKind::StringLiteral, start_pos, state.get_position());
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

    /// 处理变量
    fn lex_variable(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('$') = state.peek() {
            state.advance(1);

            // 处理 ${var} 形式
            if let Some('{') = state.peek() {
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '}' {
                        state.advance(1);
                        break;
                    }
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
            }
            // 处理 $var 形式
            else {
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
            }

            state.add_token(BashSyntaxKind::Variable, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                state.add_token(BashSyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    /// 处理关键字和标识
    fn lex_keyword_or_identifier(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let token_kind = match text {
                    "if" | "then" | "else" | "elif" | "fi" | "for" | "while" | "until" | "do" | "done" | "case" | "esac"
                    | "in" | "function" | "return" | "exit" | "break" | "continue" | "local" | "export" | "readonly"
                    | "declare" | "echo" | "printf" | "read" | "test" | "true" | "false" => BashSyntaxKind::Keyword,
                    _ => BashSyntaxKind::Identifier,
                };

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

    /// 处理运算符和分隔
    fn lex_operator_or_delimiter(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' | ')' | '[' | ']' | '{' | '}' => {
                    state.advance(1);
                    BashSyntaxKind::Delimiter
                }
                ';' | '&' | '|' => {
                    state.advance(1);
                    // 检查双字符操作
                    if let Some(next_ch) = state.peek() {
                        if (ch == '&' && next_ch == '&') || (ch == '|' && next_ch == '|') {
                            state.advance(1);
                        }
                    }
                    BashSyntaxKind::Operator
                }
                '>' | '<' => {
                    state.advance(1);
                    // 检查重定向操作
                    if let Some(next_ch) = state.peek() {
                        if next_ch == ch || next_ch == '&' {
                            state.advance(1);
                        }
                    }
                    BashSyntaxKind::Operator
                }
                '=' | '!' | '+' | '-' | '*' | '/' | '%' => {
                    state.advance(1);
                    // 检查复合操作符
                    if let Some('=') = state.peek() {
                        state.advance(1);
                    }
                    BashSyntaxKind::Operator
                }
                ',' | '.' | ':' => {
                    state.advance(1);
                    BashSyntaxKind::Delimiter
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理 Here Document
    fn lex_heredoc(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('<') = source.get_char_at(start_pos + 1) {
                state.advance(2);

                // 读取标识
                let delimiter_start = state.get_position();
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                if state.get_position() > delimiter_start {
                    state.add_token(BashSyntaxKind::Heredoc, start_pos, state.get_position());
                    true
                }
                else {
                    state.set_position(start_pos);
                    false
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

    /// 处理 Glob 模式
    fn lex_glob_pattern(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '*' || ch == '?' {
                state.advance(1);
                state.add_token(BashSyntaxKind::GlobPattern, start_pos, state.get_position());
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

    /// 处理特殊字符
    fn lex_special_char(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '~' || ch == '^' || ch == '@' {
                state.advance(1);
                state.add_token(BashSyntaxKind::SpecialChar, start_pos, state.get_position());
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

    /// 处理普通文
    fn lex_text(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_whitespace()
                && ![
                    '#', '$', '"', '\'', '(', ')', '[', ']', '{', '}', ';', '&', '|', '>', '<', '=', '!', '+', '-', '*', '/',
                    '%', ',', '.', ':', '~', '^', '?', '@',
                ]
                .contains(&ch)
            {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if !ch.is_ascii_whitespace()
                        && ![
                            '#', '$', '"', '\'', '(', ')', '[', ']', '{', '}', ';', '&', '|', '>', '<', '=', '!', '+', '-',
                            '*', '/', '%', ',', '.', ':', '~', '^', '?', '@',
                        ]
                        .contains(&ch)
                    {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(BashSyntaxKind::Text, start_pos, state.get_position());
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
}

impl<'config> Lexer<BashLanguage> for BashLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<BashSyntaxKind> {
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

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_variable(&mut state) {
                continue;
            }

            if self.lex_heredoc(&mut state, source) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_keyword_or_identifier(&mut state, source) {
                continue;
            }

            if self.lex_operator_or_delimiter(&mut state, source) {
                continue;
            }

            if self.lex_glob_pattern(&mut state) {
                continue;
            }

            if self.lex_special_char(&mut state) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(BashSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(BashSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
