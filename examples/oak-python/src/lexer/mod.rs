use crate::{kind::PythonSyntaxKind, language::PythonLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, PythonLanguage>;

pub struct PythonLexer<'config> {
    config: &'config PythonLanguage,
}

impl<'config> PythonLexer<'config> {
    pub fn new(config: &'config PythonLanguage) -> Self {
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
            state.add_token(PythonSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(PythonSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(PythonSyntaxKind::Newline, start_pos, state.get_position());
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

            // 读取到行尾
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(PythonSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // 检查是否是字符串开始
        let quote_char = match state.peek() {
            Some('"') => '"',
            Some('\'') => '\'',
            _ => return false,
        };

        state.advance(1); // 跳过开始引号
        // 检查是否是三引号字符串
        let is_triple = state.peek() == Some(quote_char) && state.peek_next_n(2) == Some(quote_char);
        if is_triple {
            state.advance(2); // 跳过另外两个引号
        }

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
                if is_triple {
                    // 检查是否是三引号结

                    if state.peek_next_n(1) == Some(quote_char) && state.peek_next_n(2) == Some(quote_char) {
                        state.advance(3); // 跳过三个结束引号
                        break;
                    }
                    else {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(1); // 跳过结束引号
                    break;
                }
            }
            else if ch == '\n' || ch == '\r' {
                if !is_triple {
                    // 单行字符串不能包含换行符
                    break;
                }
                state.advance(ch.len_utf8());
            }
            else {
                state.advance(ch.len_utf8());
            }
        }

        state.add_token(PythonSyntaxKind::String, start_pos, state.get_position());
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
                Some('b') | Some('B') => {
                    state.advance(2); // 跳过 '0b' '0B'
                    // 读取二进制数

                    while let Some(ch) = state.peek() {
                        if ch == '0' || ch == '1' {
                            state.advance(1);
                        }
                        else if ch == '_' {
                            state.advance(1); // 数字分隔
                        }
                        else {
                            break;
                        }
                    }
                }
                Some('o') | Some('O') => {
                    state.advance(2); // 跳过 '0o' '0O'
                    // 读取八进制数

                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() && ch < '8' {
                            state.advance(1);
                        }
                        else if ch == '_' {
                            state.advance(1); // 数字分隔
                        }
                        else {
                            break;
                        }
                    }
                }
                Some('x') | Some('X') => {
                    state.advance(2); // 跳过 '0x' '0X'
                    // 读取十六进制数字
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() {
                            state.advance(1);
                        }
                        else if ch == '_' {
                            state.advance(1); // 数字分隔
                        }
                        else {
                            break;
                        }
                    }
                }
                _ => {
                    // 十进制数

                    self.lex_decimal_number(state, &mut is_float);
                }
            }
        }
        else {
            // 十进制数

            self.lex_decimal_number(state, &mut is_float);
        }

        // 检查复数后缀
        if let Some('j') | Some('J') = state.peek() {
            state.advance(1);
        }

        let kind = PythonSyntaxKind::Number;

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
            else if ch == '_' {
                state.advance(1); // 数字分隔
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
                else if ch == '_' {
                    state.advance(1); // 数字分隔
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
                else if ch == '_' {
                    state.advance(1); // 数字分隔
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

        if !state.peek().map_or(false, |c| c.is_ascii_alphabetic() || c == '_') {
            return false;
        }

        // 读取标识

        while let Some(ch) = state.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // 检查是否是关键

        let kind = PythonSyntaxKind::Identifier; // 简化处理，都标记为标识

        state.add_token(kind, start_pos, state.get_position());
        true
    }

    /// 处理操作

    fn lex_operator(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // 尝试匹配多字符操作符
        if let Some(ch) = state.peek() {
            let next_ch = state.peek_next_n(1);

            let kind = match (ch, next_ch) {
                ('*', Some('*')) => {
                    state.advance(2);
                    PythonSyntaxKind::DoubleStar
                }
                ('/', Some('/')) => {
                    state.advance(2);
                    PythonSyntaxKind::DoubleSlash
                }
                ('<', Some('<')) => {
                    state.advance(2);
                    PythonSyntaxKind::LeftShift
                }
                ('>', Some('>')) => {
                    state.advance(2);
                    PythonSyntaxKind::RightShift
                }
                ('<', Some('=')) => {
                    state.advance(2);
                    PythonSyntaxKind::LessEqual
                }
                ('>', Some('=')) => {
                    state.advance(2);
                    PythonSyntaxKind::GreaterEqual
                }
                ('=', Some('=')) => {
                    state.advance(2);
                    PythonSyntaxKind::Equal
                }
                ('!', Some('=')) => {
                    state.advance(2);
                    PythonSyntaxKind::NotEqual
                }
                ('+', Some('=')) => {
                    state.advance(2);
                    PythonSyntaxKind::PlusAssign
                }
                ('-', Some('=')) => {
                    state.advance(2);
                    PythonSyntaxKind::MinusAssign
                }
                ('*', Some('=')) => {
                    state.advance(2);
                    PythonSyntaxKind::StarAssign
                }
                ('/', Some('=')) => {
                    state.advance(2);
                    PythonSyntaxKind::SlashAssign
                }
                ('%', Some('=')) => {
                    state.advance(2);
                    PythonSyntaxKind::PercentAssign
                }
                ('&', Some('=')) => {
                    state.advance(2);
                    PythonSyntaxKind::AmpersandAssign
                }
                ('|', Some('=')) => {
                    state.advance(2);
                    PythonSyntaxKind::PipeAssign
                }
                ('^', Some('=')) => {
                    state.advance(2);
                    PythonSyntaxKind::CaretAssign
                }
                ('-', Some('>')) => {
                    state.advance(2);
                    PythonSyntaxKind::Arrow
                }
                // 单字符操作符
                ('+', _) => {
                    state.advance(1);
                    PythonSyntaxKind::Plus
                }
                ('-', _) => {
                    state.advance(1);
                    PythonSyntaxKind::Minus
                }
                ('*', _) => {
                    state.advance(1);
                    PythonSyntaxKind::Star
                }
                ('/', _) => {
                    state.advance(1);
                    PythonSyntaxKind::Slash
                }
                ('%', _) => {
                    state.advance(1);
                    PythonSyntaxKind::Percent
                }
                ('=', _) => {
                    state.advance(1);
                    PythonSyntaxKind::Assign
                }
                ('<', _) => {
                    state.advance(1);
                    PythonSyntaxKind::Less
                }
                ('>', _) => {
                    state.advance(1);
                    PythonSyntaxKind::Greater
                }
                ('&', _) => {
                    state.advance(1);
                    PythonSyntaxKind::Ampersand
                }
                ('|', _) => {
                    state.advance(1);
                    PythonSyntaxKind::Pipe
                }
                ('^', _) => {
                    state.advance(1);
                    PythonSyntaxKind::Caret
                }
                ('~', _) => {
                    state.advance(1);
                    PythonSyntaxKind::Tilde
                }
                ('@', _) => {
                    state.advance(1);
                    PythonSyntaxKind::At
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理分隔

    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => PythonSyntaxKind::LeftParen,
                ')' => PythonSyntaxKind::RightParen,
                '[' => PythonSyntaxKind::LeftBracket,
                ']' => PythonSyntaxKind::RightBracket,
                '{' => PythonSyntaxKind::LeftBrace,
                '}' => PythonSyntaxKind::RightBrace,
                ',' => PythonSyntaxKind::Comma,
                ':' => PythonSyntaxKind::Colon,
                ';' => PythonSyntaxKind::Semicolon,
                '.' => {
                    // 检查是否是省略号
                    if state.peek_next_n(1) == Some('.') && state.peek_next_n(2) == Some('.') {
                        state.advance(3);
                        state.add_token(PythonSyntaxKind::Ellipsis, start_pos, state.get_position());
                        return true;
                    }
                    else {
                        PythonSyntaxKind::Dot
                    }
                }
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理缩进
    fn lex_indent(&self, state: &mut State) -> bool {
        // 简化的缩进处理
        false
    }

    /// 处理其他字符
    fn lex_other(&self, state: &mut State) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();
            state.advance(ch.len_utf8());
            state.add_token(PythonSyntaxKind::Error, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<PythonLanguage> for PythonLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<PythonSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
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

            if self.lex_indent(&mut state) {
                continue;
            }

            if self.lex_other(&mut state) {
                continue;
            }

            // 如果没有匹配任何规则，前进一个字符避免无限循

            if let Some(ch) = state.peek() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(PythonSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(PythonSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
