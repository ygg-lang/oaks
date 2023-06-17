use crate::{kind::JavaSyntaxKind, language::JavaLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, JavaLanguage>;

pub struct JavaLexer<'config> {
    #[allow(dead_code)]
    config: &'config JavaLanguage,
}

impl<'config> JavaLexer<'config> {
    pub fn new(config: &'config JavaLanguage) -> Self {
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
            state.add_token(JavaSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(JavaSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(JavaSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符或关键字
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let length = state.get_position() - start_pos;
                let text = source.get_text_at(start_pos).and_then(|s| s.get(..length)).unwrap_or("");
                let token_kind = JavaSyntaxKind::from_keyword_str(text).unwrap_or(JavaSyntaxKind::Identifier);

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

    /// 处理数字字面量
    fn lex_number_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 处理整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let mut is_float = false;

                // 检查小数点
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            is_float = true;
                            state.advance(1); // 跳过小数

                            // 处理小数部分
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
                }

                // 检查指数部
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

                        // 指数数字
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

                // 检查后缀
                if let Some(suffix) = state.peek() {
                    match suffix {
                        'f' | 'F' => {
                            is_float = true;
                            state.advance(1);
                        }
                        'd' | 'D' => {
                            is_float = true;
                            state.advance(1);
                        }
                        'l' | 'L' => {
                            state.advance(1);
                        }
                        _ => {}
                    }
                }

                let token_kind = if is_float { JavaSyntaxKind::FloatingPointLiteral } else { JavaSyntaxKind::IntegerLiteral };

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
    fn lex_string_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);
            let mut found_end = false;

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    found_end = true;
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 字符串不能跨行（除非转义
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            if found_end {
                state.add_token(JavaSyntaxKind::StringLiteral, start_pos, state.get_position());
                true
            }
            else {
                // 回退到开始位
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理字符字面
    fn lex_char_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);
            let mut found_end = false;

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch != '\'' && ch != '\n' && ch != '\r' {
                    state.advance(ch.len_utf8());
                }

                if let Some('\'') = state.peek() {
                    state.advance(1);
                    found_end = true;
                }
            }

            if found_end {
                state.add_token(JavaSyntaxKind::CharacterLiteral, start_pos, state.get_position());
                true
            }
            else {
                // 回退到开始位
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some(next_ch) = state.peek_next_n(1) {
                if next_ch == '/' {
                    // 行注
                    state.advance(2);

                    while let Some(ch) = state.peek() {
                        if ch == '\n' || ch == '\r' {
                            break;
                        }
                        else {
                            state.advance(ch.len_utf8());
                        }
                    }

                    state.add_token(JavaSyntaxKind::LineComment, start_pos, state.get_position());
                    return true;
                }
                else if next_ch == '*' {
                    // 块注
                    state.advance(2);
                    let mut found_end = false;

                    while let Some(ch) = state.peek() {
                        if ch == '*' {
                            if let Some('/') = state.peek_next_n(1) {
                                state.advance(2);
                                found_end = true;
                                break;
                            }
                            else {
                                state.advance(1);
                            }
                        }
                        else {
                            state.advance(ch.len_utf8());
                        }
                    }

                    if found_end {
                        state.add_token(JavaSyntaxKind::BlockComment, start_pos, state.get_position());
                        return true;
                    }
                    else {
                        // 未闭合的块注释，回退
                        state.set_position(start_pos);
                        return false;
                    }
                }
            }
        }
        false
    }

    /// 处理运算符和分隔
    fn lex_operator_or_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::Equals
                    }
                    else {
                        JavaSyntaxKind::Assign
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::GreaterThanEquals
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaSyntaxKind::UnsignedRightShiftEquals
                            }
                            else {
                                JavaSyntaxKind::UnsignedRightShift
                            }
                        }
                        else if let Some('=') = state.peek() {
                            state.advance(1);
                            JavaSyntaxKind::RightShiftEquals
                        }
                        else {
                            JavaSyntaxKind::RightShift
                        }
                    }
                    else {
                        JavaSyntaxKind::GreaterThan
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::LessThanEquals
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            JavaSyntaxKind::LeftShiftEquals
                        }
                        else {
                            JavaSyntaxKind::LeftShift
                        }
                    }
                    else {
                        JavaSyntaxKind::LessThan
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::BangEquals
                    }
                    else {
                        JavaSyntaxKind::Bang
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::AmpersandAmpersand
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::AmpersandEquals
                    }
                    else {
                        JavaSyntaxKind::Ampersand
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::PipePipe
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::PipeEquals
                    }
                    else {
                        JavaSyntaxKind::Pipe
                    }
                }
                '+' => {
                    state.advance(1);
                    if let Some('+') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::PlusPlus
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::PlusEquals
                    }
                    else {
                        JavaSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::MinusMinus
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::MinusEquals
                    }
                    else {
                        JavaSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::AsteriskEquals
                    }
                    else {
                        JavaSyntaxKind::Asterisk
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::SlashEquals
                    }
                    else {
                        JavaSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::PercentEquals
                    }
                    else {
                        JavaSyntaxKind::Percent
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::CaretEquals
                    }
                    else {
                        JavaSyntaxKind::Caret
                    }
                }
                '~' => {
                    state.advance(1);
                    JavaSyntaxKind::Tilde
                }
                '?' => {
                    state.advance(1);
                    JavaSyntaxKind::Question
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        JavaSyntaxKind::DoubleColon
                    }
                    else {
                        JavaSyntaxKind::Colon
                    }
                }
                '(' => {
                    state.advance(1);
                    JavaSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    JavaSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    JavaSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    JavaSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    JavaSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    JavaSyntaxKind::RightBracket
                }
                ';' => {
                    state.advance(1);
                    JavaSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    JavaSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        if let Some('.') = state.peek_next_n(1) {
                            state.advance(2);
                            JavaSyntaxKind::Ellipsis
                        }
                        else {
                            JavaSyntaxKind::Dot
                        }
                    }
                    else {
                        JavaSyntaxKind::Dot
                    }
                }
                '@' => {
                    state.advance(1);
                    JavaSyntaxKind::At
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
}

impl<'config> Lexer<JavaLanguage> for JavaLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<JavaSyntaxKind> {
        let mut state = State::new(source);

        loop {
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

            if self.lex_string_literal(&mut state) {
                continue;
            }

            if self.lex_char_literal(&mut state) {
                continue;
            }

            if self.lex_number_literal(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator_or_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，检查是否到达文件末尾
            if let Some(ch) = state.peek() {
                // 跳过当前字符并标记为错误
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(JavaSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                // 到达文件末尾，退出循环
                break;
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(JavaSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
