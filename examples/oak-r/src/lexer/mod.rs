use crate::{kind::RSyntaxKind, language::RLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, RLanguage>;

pub struct RLexer<'config> {
    config: &'config RLanguage,
}

impl<'config> RLexer<'config> {
    pub fn new(config: &'config RLanguage) -> Self {
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
            state.add_token(RSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(RSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(RSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        if let Some('#') = state.peek() {
            let start_pos = state.get_position();
            state.advance(1); // 跳过 '#'

            // 读取到行
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(RSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // 检查是否是字符串开
        let quote_char = match state.peek() {
            Some('"') => '"',
            Some('\'') => '\'',
            _ => return false,
        };

        state.advance(1); // 跳过开始引
        let mut escaped = false;
        while let Some(ch) = state.peek() {
            if escaped {
                escaped = false;
                state.advance(ch.len_utf8());
                continue;
            }

            if ch == '\\' {
                escaped = true;
                state.advance(1);
                continue;
            }

            if ch == quote_char {
                state.advance(1); // 跳过结束引号
                break;
            }
            else if ch == '\n' || ch == '\r' {
                // R 字符串不能跨                break;
            }
            else {
                state.advance(ch.len_utf8());
            }
        }

        state.add_token(RSyntaxKind::StringLiteral, start_pos, state.get_position());
        true
    }

    /// 处理数字字面
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if !state.peek().map_or(false, |c| c.is_ascii_digit()) {
            return false;
        }

        let mut is_float = false;

        // 检查进制前缀
        if state.peek() == Some('0') {
            let next_char = state.peek_next_n(1);
            match next_char {
                Some('x') | Some('X') => {
                    state.advance(2); // 跳过 '0x' '0X'
                    // 读取十六进制数字
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                _ => {
                    // 十进制数                    self.lex_decimal_number(state, &mut is_float);
                }
            }
        }
        else {
            // 十进制数            self.lex_decimal_number(state, &mut is_float);
        }

        // 检R 整数后缀 'L'
        if let Some('L') = state.peek() {
            state.advance(1);
        }

        let kind = if is_float { RSyntaxKind::FloatLiteral } else { RSyntaxKind::IntegerLiteral };

        state.add_token(kind, start_pos, state.get_position());
        true
    }

    /// 处理十进制数
    fn lex_decimal_number(&self, state: &mut State, is_float: &mut bool) {
        // 读取整数部分
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // 检查小数点
        if state.peek() == Some('.') && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
            *is_float = true;
            state.advance(1); // 跳过小数
            // 读取小数部分
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(1);
                }
                else {
                    break;
                }
            }
        }

        // 检查科学计数法
        if let Some('e') | Some('E') = state.peek() {
            *is_float = true;
            state.advance(1);

            // 可选的符号
            if let Some('+') | Some('-') = state.peek() {
                state.advance(1);
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

    /// 处理标识符或关键
    fn lex_identifier_or_keyword(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // 检查第一个字
        if !state.peek().map_or(false, |c| c.is_ascii_alphabetic() || c == '_' || c == '.') {
            return false;
        }

        // 读取标识
        while let Some(ch) = state.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // 检查是否是关键
        let kind = RSyntaxKind::Identifier; // 简化处理，都标记为标识
        state.add_token(kind, start_pos, state.get_position());
        true
    }

    /// 处理操作
    fn lex_operator(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // 检查三字符操作符
        if state.peek() == Some('<') && state.peek_next_n(1) == Some('<') && state.peek_next_n(2) == Some('-') {
            state.advance(3);
            state.add_token(RSyntaxKind::DoubleLeftArrow, start_pos, state.get_position());
            return true;
        }

        // 检查双字符操作符
        if let Some(ch) = state.peek() {
            if let Some(next_ch) = state.peek_next_n(1) {
                let token_kind = match (ch, next_ch) {
                    ('<', '-') => RSyntaxKind::LeftArrow,
                    ('-', '>') => RSyntaxKind::RightArrow,
                    ('>', '>') => RSyntaxKind::DoubleRightArrow,
                    ('<', '=') => RSyntaxKind::LessEqual,
                    ('>', '=') => RSyntaxKind::GreaterEqual,
                    ('=', '=') => RSyntaxKind::EqualEqual,
                    ('!', '=') => RSyntaxKind::NotEqual,
                    ('&', '&') => RSyntaxKind::AndAnd,
                    ('|', '|') => RSyntaxKind::OrOr,
                    _ => {
                        // 检查单字符操作符
                        let token_kind = match ch {
                            '+' => RSyntaxKind::Plus,
                            '-' => RSyntaxKind::Minus,
                            '*' => RSyntaxKind::Star,
                            '/' => RSyntaxKind::Slash,
                            '%' => RSyntaxKind::Percent,
                            '^' => RSyntaxKind::Caret,
                            '=' => RSyntaxKind::Equal,
                            '<' => RSyntaxKind::Less,
                            '>' => RSyntaxKind::Greater,
                            '&' => RSyntaxKind::And,
                            '|' => RSyntaxKind::Or,
                            '!' => RSyntaxKind::Not,
                            '~' => RSyntaxKind::Tilde,
                            _ => return false,
                        };
                        state.advance(1);
                        state.add_token(token_kind, start_pos, state.get_position());
                        return true;
                    }
                };
                state.advance(2);
                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理分隔
    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // 检查双冒号和三冒号
        if state.peek() == Some(':') {
            if state.peek_next_n(1) == Some(':') {
                if state.peek_next_n(2) == Some(':') {
                    state.advance(3);
                    state.add_token(RSyntaxKind::TripleColon, start_pos, state.get_position());
                    return true;
                }
                else {
                    state.advance(2);
                    state.add_token(RSyntaxKind::DoubleColon, start_pos, state.get_position());
                    return true;
                }
            }
        }

        // 单字符分隔符
        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => RSyntaxKind::LeftParen,
                ')' => RSyntaxKind::RightParen,
                '[' => RSyntaxKind::LeftBracket,
                ']' => RSyntaxKind::RightBracket,
                '{' => RSyntaxKind::LeftBrace,
                '}' => RSyntaxKind::RightBrace,
                ',' => RSyntaxKind::Comma,
                ';' => RSyntaxKind::Semicolon,
                ':' => RSyntaxKind::Colon,
                '.' => RSyntaxKind::Dot,
                '$' => RSyntaxKind::Dollar,
                '@' => RSyntaxKind::At,
                _ => return false,
            };

            state.advance(1);
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<RLanguage> for RLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<RSyntaxKind> {
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

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(RSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(RSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
