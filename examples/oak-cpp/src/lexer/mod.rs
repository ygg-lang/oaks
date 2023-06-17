use crate::{kind::CppSyntaxKind, language::CppLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, CppLanguage>;

pub struct CppLexer<'config> {
    config: &'config CppLanguage,
}

/// C 词法分析器类型别名
pub type CLexer<'config> = CppLexer<'config>;

impl<'config> CppLexer<'config> {
    pub fn new(config: &'config CppLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State<'_>) -> bool {
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
            state.add_token(CppSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(CppSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(CppSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_next_n(1) {
                // 单行注释
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(CppSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else if let Some('*') = state.peek_next_n(1) {
                // 多行注释
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '*' && state.peek_next_n(1) == Some('/') {
                        state.advance(2);
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(CppSyntaxKind::Comment, start_pos, state.get_position());
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
    fn lex_string(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

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

                if ch == '"' {
                    state.advance(1);
                    break;
                }

                if ch == '\n' || ch == '\r' {
                    break; // 未闭合的字符
                }

                state.advance(ch.len_utf8());
            }

            state.add_token(CppSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符字面
    fn lex_character(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

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

                if ch == '\'' {
                    state.advance(1);
                    break;
                }

                if ch == '\n' || ch == '\r' {
                    break; // 未闭合的字符
                }

                state.advance(ch.len_utf8());
            }

            state.add_token(CppSyntaxKind::CharacterLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面
    fn lex_number(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || (ch == '.' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit())) {
                let mut is_float = false;

                // 处理十六进制、八进制、二进制
                if ch == '0' {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch == 'x' || next_ch == 'X' {
                            // 十六进制
                            state.advance(2);
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_hexdigit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                        else if next_ch == 'b' || next_ch == 'B' {
                            // 二进
                            state.advance(2);
                            while let Some(ch) = state.peek() {
                                if ch == '0' || ch == '1' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                        else if next_ch.is_ascii_digit() {
                            // 八进
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                        else {
                            state.advance(1); // 只是 '0'
                        }
                    }
                    else {
                        state.advance(1); // 只是 '0'
                    }
                }
                else {
                    // 十进制整数部
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            is_float = true;
                            state.advance(1); // 消费小数
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

                // 检查科学记数法
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        is_float = true;
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }
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
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphabetic() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let token_kind = if is_float { CppSyntaxKind::FloatLiteral } else { CppSyntaxKind::IntegerLiteral };

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

    /// 处理关键字或标识
    fn lex_keyword_or_identifier(&self, state: &mut State<'_>, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let token_kind = match text {
                    // C++ 关键
                    "alignas" | "alignof" | "and" | "and_eq" | "asm" | "atomic_cancel" | "atomic_commit"
                    | "atomic_noexcept" | "auto" | "bitand" | "bitor" | "bool" | "break" | "case" | "catch" | "char"
                    | "char8_t" | "char16_t" | "char32_t" | "class" | "compl" | "concept" | "const" | "consteval"
                    | "constexpr" | "constinit" | "const_cast" | "continue" | "co_await" | "co_return" | "co_yield"
                    | "decltype" | "default" | "delete" | "do" | "double" | "dynamic_cast" | "else" | "enum" | "explicit"
                    | "export" | "extern" | "false" | "float" | "for" | "friend" | "goto" | "if" | "inline" | "int"
                    | "long" | "mutable" | "namespace" | "new" | "noexcept" | "not" | "not_eq" | "nullptr" | "operator"
                    | "or" | "or_eq" | "private" | "protected" | "public" | "reflexpr" | "register" | "reinterpret_cast"
                    | "requires" | "return" | "short" | "signed" | "sizeof" | "static" | "static_assert" | "static_cast"
                    | "struct" | "switch" | "synchronized" | "template" | "this" | "thread_local" | "throw" | "true"
                    | "try" | "typedef" | "typeid" | "typename" | "union" | "unsigned" | "using" | "virtual" | "void"
                    | "volatile" | "wchar_t" | "while" | "xor" | "xor_eq" => CppSyntaxKind::Keyword,
                    "true" | "false" => CppSyntaxKind::BooleanLiteral,
                    _ => CppSyntaxKind::Identifier,
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

    /// 处理操作
    fn lex_operator(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let (token_kind, advance_count) = match ch {
                '+' => {
                    if let Some('+') = state.peek_next_n(1) {
                        (CppSyntaxKind::Increment, 2)
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        (CppSyntaxKind::PlusAssign, 2)
                    }
                    else {
                        (CppSyntaxKind::Plus, 1)
                    }
                }
                '-' => {
                    if let Some('-') = state.peek_next_n(1) {
                        (CppSyntaxKind::Decrement, 2)
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        (CppSyntaxKind::MinusAssign, 2)
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        (CppSyntaxKind::Arrow, 2)
                    }
                    else {
                        (CppSyntaxKind::Minus, 1)
                    }
                }
                '*' => {
                    if let Some('=') = state.peek_next_n(1) {
                        (CppSyntaxKind::StarAssign, 2)
                    }
                    else {
                        (CppSyntaxKind::Star, 1)
                    }
                }
                '/' => {
                    if let Some('=') = state.peek_next_n(1) {
                        (CppSyntaxKind::SlashAssign, 2)
                    }
                    else {
                        (CppSyntaxKind::Slash, 1)
                    }
                }
                '%' => {
                    if let Some('=') = state.peek_next_n(1) {
                        (CppSyntaxKind::PercentAssign, 2)
                    }
                    else {
                        (CppSyntaxKind::Percent, 1)
                    }
                }
                '=' => {
                    if let Some('=') = state.peek_next_n(1) {
                        (CppSyntaxKind::Equal, 2)
                    }
                    else {
                        (CppSyntaxKind::Assign, 1)
                    }
                }
                '!' => {
                    if let Some('=') = state.peek_next_n(1) {
                        (CppSyntaxKind::NotEqual, 2)
                    }
                    else {
                        (CppSyntaxKind::LogicalNot, 1)
                    }
                }
                '<' => {
                    if let Some('<') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            (CppSyntaxKind::LeftShiftAssign, 3)
                        }
                        else {
                            (CppSyntaxKind::LeftShift, 2)
                        }
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        (CppSyntaxKind::LessEqual, 2)
                    }
                    else {
                        (CppSyntaxKind::Less, 1)
                    }
                }
                '>' => {
                    if let Some('>') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            (CppSyntaxKind::RightShiftAssign, 3)
                        }
                        else {
                            (CppSyntaxKind::RightShift, 2)
                        }
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        (CppSyntaxKind::GreaterEqual, 2)
                    }
                    else {
                        (CppSyntaxKind::Greater, 1)
                    }
                }
                '&' => {
                    if let Some('&') = state.peek_next_n(1) {
                        (CppSyntaxKind::LogicalAnd, 2)
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        (CppSyntaxKind::AndAssign, 2)
                    }
                    else {
                        (CppSyntaxKind::BitAnd, 1)
                    }
                }
                '|' => {
                    if let Some('|') = state.peek_next_n(1) {
                        (CppSyntaxKind::LogicalOr, 2)
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        (CppSyntaxKind::OrAssign, 2)
                    }
                    else {
                        (CppSyntaxKind::BitOr, 1)
                    }
                }
                '^' => {
                    if let Some('=') = state.peek_next_n(1) {
                        (CppSyntaxKind::XorAssign, 2)
                    }
                    else {
                        (CppSyntaxKind::BitXor, 1)
                    }
                }
                '~' => (CppSyntaxKind::BitNot, 1),
                '?' => (CppSyntaxKind::Question, 1),
                ':' => {
                    if let Some(':') = state.peek_next_n(1) {
                        (CppSyntaxKind::Scope, 2)
                    }
                    else {
                        (CppSyntaxKind::Colon, 1)
                    }
                }
                '.' => (CppSyntaxKind::Dot, 1),
                _ => return false,
            };

            state.advance(advance_count);
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理分隔
    fn lex_delimiter(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => CppSyntaxKind::LeftParen,
                ')' => CppSyntaxKind::RightParen,
                '[' => CppSyntaxKind::LeftBracket,
                ']' => CppSyntaxKind::RightBracket,
                '{' => CppSyntaxKind::LeftBrace,
                '}' => CppSyntaxKind::RightBrace,
                ',' => CppSyntaxKind::Comma,
                ';' => CppSyntaxKind::Semicolon,
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

    /// 处理预处理器指令
    fn lex_preprocessor(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            // 读取到行
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(CppSyntaxKind::Preprocessor, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<CppLanguage> for CppLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<CppSyntaxKind> {
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

            if self.lex_character(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_keyword_or_identifier(&mut state, source) {
                continue;
            }

            if self.lex_preprocessor(&mut state) {
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
                state.add_token(CppSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(CppSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
