use crate::{kind::JuliaSyntaxKind, language::JuliaLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, lexer::LexOutput, source::Source};

type State<S> = LexerState<S, JuliaLanguage>;

#[derive(Clone, Debug)]
pub struct JuliaLexer<'config> {
    config: &'config JuliaLanguage,
}

impl<'config> JuliaLexer<'config> {
    pub fn new(config: &'config JuliaLanguage) -> Self {
        Self { config }
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
            state.add_token(JuliaSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(JuliaSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(JuliaSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符和关键
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '!' || ch == '?' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end_pos = state.get_position();
                let identifier_str = state.get_text_in((start_pos..end_pos).into());

                // 检查是否是关键
                if let Some(keyword_kind) = JuliaSyntaxKind::from_str(identifier_str) {
                    state.add_token(keyword_kind, start_pos, end_pos);
                }
                else {
                    state.add_token(JuliaSyntaxKind::Identifier, start_pos, end_pos);
                }
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

    /// 处理数字字面
    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 处理整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let mut is_float = false;

                // 检查小数点
                if let Some('.') = state.peek() {
                    // 检查下一个字符是否是数字，避免与范围操作符混淆
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            is_float = true;
                            state.advance(1); // 跳过小数
                            // 处理小数部分
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

                // 检查科学计数法
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        is_float = true;
                        state.advance(1);

                        // 可选的符号
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }

                        // 指数部分
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                // 检查类型后缀 (f32, f64, i32, i64
                if let Some(ch) = state.peek() {
                    if ch.is_ascii_alphabetic() {
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_alphanumeric() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                let token_kind = if is_float { JuliaSyntaxKind::FloatLiteral } else { JuliaSyntaxKind::IntegerLiteral };

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

    /// 处理字符串字面量
    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);
                let mut found_end = false;

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if ch == '\\' {
                        // 处理转义字符
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                if found_end {
                    let token_kind = if quote == '\'' { JuliaSyntaxKind::CharLiteral } else { JuliaSyntaxKind::StringLiteral };
                    state.add_token(token_kind, start_pos, state.get_position());
                    true
                }
                else {
                    // 未找到结束引号，回退到开始位                    state.set_position(start_pos);
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

    /// 处理三重引号字符
    fn lex_triple_string<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否是三重引号
        if let Some('"') = state.peek() {
            if let Some('"') = state.peek_next_n(1) {
                if let Some('"') = state.peek_next_n(2) {
                    state.advance(3);

                    // 寻找结束的三重引号
                    while let Some(ch) = state.peek() {
                        if ch == '"' {
                            if let Some('"') = state.peek_next_n(1) {
                                if let Some('"') = state.peek_next_n(2) {
                                    state.advance(3);
                                    state.add_token(JuliaSyntaxKind::StringLiteral, start_pos, state.get_position());
                                    return true;
                                }
                            }
                        }
                        state.advance(ch.len_utf8());
                    }

                    // 未找到结束的三重引号，回退
                    state.set_position(start_pos);
                }
            }
        }
        false
    }

    /// 处理注释
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            // 检查是否是多行注释 #=
            if let Some('=') = state.peek_next_n(1) {
                state.advance(2);
                let mut depth = 1;

                while let Some(ch) = state.peek()
                    && depth > 0
                {
                    if ch == '#' && state.peek_next_n(1) == Some('=') {
                        depth += 1;
                        state.advance(2);
                    }
                    else if ch == '=' && state.peek_next_n(1) == Some('#') {
                        depth -= 1;
                        state.advance(2);
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(JuliaSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else {
                // 单行注释
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(JuliaSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
        }
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operator<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::PlusAssign
                    }
                    else {
                        JuliaSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::MinusAssign
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::Arrow
                    }
                    else {
                        JuliaSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::StarAssign
                    }
                    else {
                        JuliaSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::SlashAssign
                    }
                    else {
                        JuliaSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::PercentAssign
                    }
                    else {
                        JuliaSyntaxKind::Percent
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::CaretAssign
                    }
                    else {
                        JuliaSyntaxKind::Caret
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::Equal
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::FatArrow
                    }
                    else {
                        JuliaSyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::NotEqual
                    }
                    else {
                        JuliaSyntaxKind::Not
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::LessEqual
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::LeftShift
                    }
                    else {
                        JuliaSyntaxKind::LessThan
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::RightShift
                    }
                    else {
                        JuliaSyntaxKind::GreaterThan
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::And
                    }
                    else {
                        JuliaSyntaxKind::BitAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::Or
                    }
                    else {
                        JuliaSyntaxKind::BitOr
                    }
                }
                '~' => {
                    state.advance(1);
                    JuliaSyntaxKind::BitNot
                }
                ':' => {
                    state.advance(1);
                    JuliaSyntaxKind::Colon
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        JuliaSyntaxKind::Range
                    }
                    else {
                        JuliaSyntaxKind::Dot
                    }
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

    /// 处理分隔
    fn lex_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => JuliaSyntaxKind::LeftParen,
                ')' => JuliaSyntaxKind::RightParen,
                '[' => JuliaSyntaxKind::LeftBracket,
                ']' => JuliaSyntaxKind::RightBracket,
                '{' => JuliaSyntaxKind::LeftBrace,
                '}' => JuliaSyntaxKind::RightBrace,
                ',' => JuliaSyntaxKind::Comma,
                ';' => JuliaSyntaxKind::Semicolon,
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

impl<'config> Lexer<JuliaLanguage> for JuliaLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<JuliaLanguage>,
    ) -> LexOutput<JuliaLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> JuliaLexer<'config> {
    /// 主要的词法分析循环
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), oak_core::OakError> {
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

            if self.lex_triple_string(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(JuliaSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(JuliaSyntaxKind::Eof, eof_pos, eof_pos);

        Ok(())
    }
}
