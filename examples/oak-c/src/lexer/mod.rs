use crate::{kind::CSyntaxKind, language::CLanguage};
use core::range::Range;
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, CLanguage>;

pub struct CLexer<'config> {
    config: &'config CLanguage,
}

impl<'config> CLexer<'config> {
    pub fn new(config: &'config CLanguage) -> Self {
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
            state.add_token(CSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(CSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(CSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理单行注释
    fn lex_line_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                state.advance(1);

                // 读取到行
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(CSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else {
                // 回退
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理多行注释
    fn lex_block_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('*') = state.peek() {
                state.advance(1);

                let mut found_end = false;
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            found_end = true;
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                if found_end {
                    state.add_token(CSyntaxKind::Comment, start_pos, state.get_position());
                    true
                }
                else {
                    // 未闭合的注释，标记为错误
                    state.add_token(CSyntaxKind::Error, start_pos, state.get_position());
                    true
                }
            }
            else {
                // 回退
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理预处理器指令
    fn lex_preprocessor(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // 检查是否在行首或前面只有空
        let mut check_pos = start_pos;
        while check_pos > 0 {
            check_pos -= 1;
            if let Some(ch) = source.get_char_at(check_pos) {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else if ch != ' ' && ch != '\t' {
                    return false;
                }
            }
        }

        if let Some('#') = state.peek() {
            state.advance(1);

            // 读取到行尾，处理行连接符
            while let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1);
                    if let Some('\n') = state.peek() {
                        state.advance(1);
                        continue;
                    }
                    else if let Some('\r') = state.peek() {
                        state.advance(1);
                        if let Some('\n') = state.peek() {
                            state.advance(1);
                        }
                        continue;
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(CSyntaxKind::PreprocessorDirective, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符和关键
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in(Range { start: start_pos, end: state.get_position() }).unwrap();
                let token_kind = match text {
                    "auto" => CSyntaxKind::Auto,
                    "register" => CSyntaxKind::Register,
                    "static" => CSyntaxKind::Static,
                    "extern" => CSyntaxKind::Extern,
                    "typedef" => CSyntaxKind::Typedef,
                    "void" => CSyntaxKind::Void,
                    "char" => CSyntaxKind::Char,
                    "short" => CSyntaxKind::Short,
                    "int" => CSyntaxKind::Int,
                    "long" => CSyntaxKind::Long,
                    "float" => CSyntaxKind::Float,
                    "double" => CSyntaxKind::Double,
                    "signed" => CSyntaxKind::Signed,
                    "unsigned" => CSyntaxKind::Unsigned,
                    "struct" => CSyntaxKind::Struct,
                    "union" => CSyntaxKind::Union,
                    "enum" => CSyntaxKind::Enum,
                    "const" => CSyntaxKind::Const,
                    "volatile" => CSyntaxKind::Volatile,
                    "restrict" => CSyntaxKind::Restrict,
                    "if" => CSyntaxKind::If,
                    "else" => CSyntaxKind::Else,
                    "switch" => CSyntaxKind::Switch,
                    "case" => CSyntaxKind::Case,
                    "default" => CSyntaxKind::Default,
                    "for" => CSyntaxKind::For,
                    "while" => CSyntaxKind::While,
                    "do" => CSyntaxKind::Do,
                    "break" => CSyntaxKind::Break,
                    "continue" => CSyntaxKind::Continue,
                    "goto" => CSyntaxKind::Goto,
                    "return" => CSyntaxKind::Return,
                    "sizeof" => CSyntaxKind::Sizeof,
                    "inline" => CSyntaxKind::Inline,
                    "_Bool" => CSyntaxKind::Bool,
                    "_Complex" => CSyntaxKind::Complex,
                    "_Imaginary" => CSyntaxKind::Imaginary,
                    "_Alignas" => CSyntaxKind::Alignas,
                    "_Alignof" => CSyntaxKind::Alignof,
                    "_Atomic" => CSyntaxKind::Atomic,
                    "_Static_assert" => CSyntaxKind::StaticAssert,
                    "_Thread_local" => CSyntaxKind::ThreadLocal,
                    "_Generic" => CSyntaxKind::Generic,
                    "_Noreturn" => CSyntaxKind::Noreturn,
                    _ => CSyntaxKind::Identifier,
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

    /// 处理数字字面
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 处理十六进制
                if ch == '0' {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        if next_ch == 'x' || next_ch == 'X' {
                            state.advance(1);
                            let mut has_digits = false;
                            while let Some(hex_ch) = state.peek() {
                                if hex_ch.is_ascii_hexdigit() {
                                    state.advance(1);
                                    has_digits = true;
                                }
                                else {
                                    break;
                                }
                            }
                            if !has_digits {
                                state.add_token(CSyntaxKind::Error, start_pos, state.get_position());
                                return true;
                            }
                        }
                        else if next_ch.is_ascii_digit() {
                            // 八进制数
                            while let Some(oct_ch) = state.peek() {
                                if oct_ch >= '0' && oct_ch <= '7' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }
                else {
                    // 十进制数
                    while let Some(digit_ch) = state.peek() {
                        if digit_ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 检查是否是浮点
                let mut is_float = false;
                if let Some('.') = state.peek() {
                    state.advance(1);
                    is_float = true;
                    while let Some(digit_ch) = state.peek() {
                        if digit_ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 检查指数部
                if let Some(exp_ch) = state.peek() {
                    if exp_ch == 'e' || exp_ch == 'E' {
                        state.advance(1);
                        is_float = true;
                        if let Some(sign_ch) = state.peek() {
                            if sign_ch == '+' || sign_ch == '-' {
                                state.advance(1);
                            }
                        }
                        let mut has_exp_digits = false;
                        while let Some(digit_ch) = state.peek() {
                            if digit_ch.is_ascii_digit() {
                                state.advance(1);
                                has_exp_digits = true;
                            }
                            else {
                                break;
                            }
                        }
                        if !has_exp_digits {
                            state.add_token(CSyntaxKind::Error, start_pos, state.get_position());
                            return true;
                        }
                    }
                }

                // 处理后缀
                while let Some(suffix_ch) = state.peek() {
                    if suffix_ch == 'u'
                        || suffix_ch == 'U'
                        || suffix_ch == 'l'
                        || suffix_ch == 'L'
                        || suffix_ch == 'f'
                        || suffix_ch == 'F'
                    {
                        state.advance(1);
                        if suffix_ch == 'f' || suffix_ch == 'F' {
                            is_float = true;
                        }
                    }
                    else {
                        break;
                    }
                }

                let token_kind = if is_float { CSyntaxKind::FloatLiteral } else { CSyntaxKind::IntegerLiteral };
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

    /// 处理字符字面
    fn lex_char_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);
            let mut found_end = false;

            while let Some(ch) = state.peek() {
                if ch == '\'' {
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
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            if found_end {
                state.add_token(CSyntaxKind::CharLiteral, start_pos, state.get_position());
            }
            else {
                state.add_token(CSyntaxKind::Error, start_pos, state.get_position());
            }
            true
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
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            if found_end {
                state.add_token(CSyntaxKind::StringLiteral, start_pos, state.get_position());
            }
            else {
                state.add_token(CSyntaxKind::Error, start_pos, state.get_position());
            }
            true
        }
        else {
            false
        }
    }

    /// 处理运算符和标点符号
    fn lex_operator_or_punctuation(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            match ch {
                '(' => {
                    state.advance(1);
                    state.add_token(CSyntaxKind::LeftParen, start_pos, state.get_position());
                    true
                }
                ')' => {
                    state.advance(1);
                    state.add_token(CSyntaxKind::RightParen, start_pos, state.get_position());
                    true
                }
                '[' => {
                    state.advance(1);
                    state.add_token(CSyntaxKind::LeftBracket, start_pos, state.get_position());
                    true
                }
                ']' => {
                    state.advance(1);
                    state.add_token(CSyntaxKind::RightBracket, start_pos, state.get_position());
                    true
                }
                '{' => {
                    state.advance(1);
                    state.add_token(CSyntaxKind::LeftBrace, start_pos, state.get_position());
                    true
                }
                '}' => {
                    state.advance(1);
                    state.add_token(CSyntaxKind::RightBrace, start_pos, state.get_position());
                    true
                }
                ',' => {
                    state.advance(1);
                    state.add_token(CSyntaxKind::Comma, start_pos, state.get_position());
                    true
                }
                ';' => {
                    state.advance(1);
                    state.add_token(CSyntaxKind::Semicolon, start_pos, state.get_position());
                    true
                }
                ':' => {
                    state.advance(1);
                    state.add_token(CSyntaxKind::Colon, start_pos, state.get_position());
                    true
                }
                '.' => {
                    state.advance(1);
                    state.add_token(CSyntaxKind::Dot, start_pos, state.get_position());
                    true
                }
                '?' => {
                    state.advance(1);
                    state.add_token(CSyntaxKind::Question, start_pos, state.get_position());
                    true
                }
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::PlusAssign, start_pos, state.get_position());
                    }
                    else if let Some('+') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::Increment, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(CSyntaxKind::Plus, start_pos, state.get_position());
                    }
                    true
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::MinusAssign, start_pos, state.get_position());
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::Decrement, start_pos, state.get_position());
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::Arrow, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(CSyntaxKind::Minus, start_pos, state.get_position());
                    }
                    true
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::StarAssign, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(CSyntaxKind::Star, start_pos, state.get_position());
                    }
                    true
                }
                '/' => {
                    // 这里不处理除法运算符，因为可能是注释
                    false
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::PercentAssign, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(CSyntaxKind::Percent, start_pos, state.get_position());
                    }
                    true
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::Equal, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(CSyntaxKind::Assign, start_pos, state.get_position());
                    }
                    true
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::NotEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(CSyntaxKind::LogicalNot, start_pos, state.get_position());
                    }
                    true
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::LessEqual, start_pos, state.get_position());
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            state.add_token(CSyntaxKind::LeftShiftAssign, start_pos, state.get_position());
                        }
                        else {
                            state.add_token(CSyntaxKind::LeftShift, start_pos, state.get_position());
                        }
                    }
                    else {
                        state.add_token(CSyntaxKind::Less, start_pos, state.get_position());
                    }
                    true
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::GreaterEqual, start_pos, state.get_position());
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            state.add_token(CSyntaxKind::RightShiftAssign, start_pos, state.get_position());
                        }
                        else {
                            state.add_token(CSyntaxKind::RightShift, start_pos, state.get_position());
                        }
                    }
                    else {
                        state.add_token(CSyntaxKind::Greater, start_pos, state.get_position());
                    }
                    true
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::LogicalAnd, start_pos, state.get_position());
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::AndAssign, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(CSyntaxKind::BitAnd, start_pos, state.get_position());
                    }
                    true
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::LogicalOr, start_pos, state.get_position());
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::OrAssign, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(CSyntaxKind::BitOr, start_pos, state.get_position());
                    }
                    true
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(CSyntaxKind::XorAssign, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(CSyntaxKind::BitXor, start_pos, state.get_position());
                    }
                    true
                }
                '~' => {
                    state.advance(1);
                    state.add_token(CSyntaxKind::BitNot, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// 处理除法运算符（需要区分注释）
    fn lex_division(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('=') = state.peek() {
                state.advance(1);
                state.add_token(CSyntaxKind::SlashAssign, start_pos, state.get_position());
                true
            }
            else {
                state.add_token(CSyntaxKind::Slash, start_pos, state.get_position());
                true
            }
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<CLanguage> for CLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<CSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则，按优先级排
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_line_comment(&mut state) {
                continue;
            }

            if self.lex_block_comment(&mut state) {
                continue;
            }

            if self.lex_preprocessor(&mut state, source) {
                continue;
            }

            if self.lex_string_literal(&mut state) {
                continue;
            }

            if self.lex_char_literal(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator_or_punctuation(&mut state) {
                continue;
            }

            if self.lex_division(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(CSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(CSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
