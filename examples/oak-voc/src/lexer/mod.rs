use crate::{kind::VocSyntaxKind, language::VocLanguage};
use oak_core::{
    Lexer, LexerState,
    lexer::{LexOutput, LexerCache},
    source::Source,
};

type State<'a, S> = LexerState<'a, S, VocLanguage>;

#[derive(Clone, Debug)]
pub struct VocLexer<'config> {
    _config: &'config VocLanguage,
}

impl<'config> Lexer<VocLanguage> for VocLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<VocLanguage>) -> LexOutput<VocLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> VocLexer<'config> {
    pub fn new(config: &'config VocLanguage) -> Self {
        Self { _config: config }
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
            state.add_token(VocSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(VocSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(VocSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 单行注释 //
        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_next_n(1) {
                state.advance(2);

                // 读取到行尾
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(VocSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
            // 多行注释 /* */
            else if let Some('*') = state.peek_next_n(1) {
                state.advance(2);

                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        if let Some('/') = state.peek_next_n(1) {
                            state.advance(2);
                            break;
                        }
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(VocSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理字符串字面量
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);
                let mut escaped = false;

                while let Some(ch) = state.peek() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8());
                    }
                    else if ch == '\\' {
                        escaped = true;
                        state.advance(1);
                    }
                    else if ch == quote {
                        state.advance(1);
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break; // 字符串不能跨行
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                let token_kind = if quote == '"' { VocSyntaxKind::StringLiteral } else { VocSyntaxKind::CharLiteral };
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
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 整数部分
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                let mut is_float = false;
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            is_float = true;
                            state.advance(1); // 跳过小数点

                            // 小数部分
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
                }

                // 检查指数
                if let Some(e) = state.peek() {
                    if e == 'e' || e == 'E' {
                        let exp_start = state.get_position();
                        state.advance(1);

                        // 可选的符号
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }

                        // 指数数字
                        let mut has_exp_digits = false;
                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_digit() {
                                has_exp_digits = true;
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }

                        if has_exp_digits {
                            is_float = true;
                        }
                        else {
                            // 回退到指数开始位置
                            state.set_position(exp_start);
                        }
                    }
                }

                let token_kind = if is_float { VocSyntaxKind::FloatLiteral } else { VocSyntaxKind::IntegerLiteral };
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

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_kind = match text.as_ref() {
                    "module" => VocSyntaxKind::ModuleKw,
                    "import" => VocSyntaxKind::ImportKw,
                    "pub" => VocSyntaxKind::PubKw,
                    "fn" => VocSyntaxKind::FnKw,
                    "struct" => VocSyntaxKind::StructKw,
                    "interface" => VocSyntaxKind::InterfaceKw,
                    "enum" => VocSyntaxKind::EnumKw,
                    "type" => VocSyntaxKind::TypeKw,
                    "const" => VocSyntaxKind::ConstKw,
                    "mut" => VocSyntaxKind::MutKw,
                    "shared" => VocSyntaxKind::SharedKw,
                    "volatile" => VocSyntaxKind::VolatileKw,
                    "unsafe" => VocSyntaxKind::UnsafeKw,
                    "if" => VocSyntaxKind::IfKw,
                    "else" => VocSyntaxKind::ElseKw,
                    "for" => VocSyntaxKind::ForKw,
                    "in" => VocSyntaxKind::InKw,
                    "match" => VocSyntaxKind::MatchKw,
                    "or" => VocSyntaxKind::OrKw,
                    "return" => VocSyntaxKind::ReturnKw,
                    "break" => VocSyntaxKind::BreakKw,
                    "continue" => VocSyntaxKind::ContinueKw,
                    "goto" => VocSyntaxKind::GotoKw,
                    "defer" => VocSyntaxKind::DeferKw,
                    "go" => VocSyntaxKind::GoKw,
                    "select" => VocSyntaxKind::SelectKw,
                    "lock" => VocSyntaxKind::LockKw,
                    "rlock" => VocSyntaxKind::RlockKw,
                    "as" => VocSyntaxKind::AsKw,
                    "is" => VocSyntaxKind::IsKw,
                    "sizeof" => VocSyntaxKind::SizeofKw,
                    "typeof" => VocSyntaxKind::TypeofKw,
                    "offsetof" => VocSyntaxKind::OffsetofKw,
                    "assert" => VocSyntaxKind::AssertKw,
                    "panic" => VocSyntaxKind::PanicKw,
                    "eprintln" => VocSyntaxKind::EprintlnKw,
                    "println" => VocSyntaxKind::PrintlnKw,
                    "print" => VocSyntaxKind::PrintKw,
                    "eprint" => VocSyntaxKind::EprintKw,
                    "bool" => VocSyntaxKind::BoolKw,
                    "i8" => VocSyntaxKind::I8Kw,
                    "i16" => VocSyntaxKind::I16Kw,
                    "i32" => VocSyntaxKind::I32Kw,
                    "i64" => VocSyntaxKind::I64Kw,
                    "u8" => VocSyntaxKind::U8Kw,
                    "u16" => VocSyntaxKind::U16Kw,
                    "u32" => VocSyntaxKind::U32Kw,
                    "u64" => VocSyntaxKind::U64Kw,
                    "int" => VocSyntaxKind::IntKw,
                    "uint" => VocSyntaxKind::UintKw,
                    "f32" => VocSyntaxKind::F32Kw,
                    "f64" => VocSyntaxKind::F64Kw,
                    "string" => VocSyntaxKind::StringKw,
                    "rune" => VocSyntaxKind::RuneKw,
                    "byte" => VocSyntaxKind::ByteKw,
                    "voidptr" => VocSyntaxKind::VoidptrKw,
                    "char" => VocSyntaxKind::CharKw,
                    "true" | "false" => VocSyntaxKind::BoolLiteral,
                    _ => VocSyntaxKind::Identifier,
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

    /// 处理操作符和标点符号
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::PlusEq
                    }
                    else if let Some('+') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::PlusPlus
                    }
                    else {
                        state.advance(1);
                        VocSyntaxKind::Plus
                    }
                }
                '-' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::MinusEq
                    }
                    else if let Some('-') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::MinusMinus
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::Arrow
                    }
                    else {
                        state.advance(1);
                        VocSyntaxKind::Minus
                    }
                }
                '*' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::StarEq
                    }
                    else {
                        state.advance(1);
                        VocSyntaxKind::Star
                    }
                }
                '/' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::SlashEq
                    }
                    else {
                        state.advance(1);
                        VocSyntaxKind::Slash
                    }
                }
                '%' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::PercentEq
                    }
                    else {
                        state.advance(1);
                        VocSyntaxKind::Percent
                    }
                }
                '&' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::AmpersandEq
                    }
                    else if let Some('&') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::AndAnd
                    }
                    else {
                        state.advance(1);
                        VocSyntaxKind::Ampersand
                    }
                }
                '|' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::PipeEq
                    }
                    else if let Some('|') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::OrOr
                    }
                    else {
                        state.advance(1);
                        VocSyntaxKind::Pipe
                    }
                }
                '^' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::CaretEq
                    }
                    else {
                        state.advance(1);
                        VocSyntaxKind::Caret
                    }
                }
                '=' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::EqEq
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::FatArrow
                    }
                    else {
                        state.advance(1);
                        VocSyntaxKind::Eq
                    }
                }
                '!' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::Ne
                    }
                    else {
                        state.advance(1);
                        VocSyntaxKind::Bang
                    }
                }
                '<' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::Le
                    }
                    else if let Some('<') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            state.advance(3);
                            VocSyntaxKind::LeftShiftEq
                        }
                        else {
                            state.advance(2);
                            VocSyntaxKind::LeftShift
                        }
                    }
                    else {
                        state.advance(1);
                        VocSyntaxKind::LessThan
                    }
                }
                '>' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocSyntaxKind::Ge
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            state.advance(3);
                            VocSyntaxKind::RightShiftEq
                        }
                        else {
                            state.advance(2);
                            VocSyntaxKind::RightShift
                        }
                    }
                    else {
                        state.advance(1);
                        VocSyntaxKind::GreaterThan
                    }
                }
                '.' => {
                    if let Some('.') = state.peek_next_n(1) {
                        if let Some('.') = state.peek_next_n(2) {
                            state.advance(3);
                            VocSyntaxKind::DotDotDot
                        }
                        else {
                            state.advance(2);
                            VocSyntaxKind::DotDot
                        }
                    }
                    else {
                        state.advance(1);
                        VocSyntaxKind::Dot
                    }
                }
                ',' => {
                    state.advance(1);
                    VocSyntaxKind::Comma
                }
                ':' => {
                    state.advance(1);
                    VocSyntaxKind::Colon
                }
                ';' => {
                    state.advance(1);
                    VocSyntaxKind::Semicolon
                }
                '(' => {
                    state.advance(1);
                    VocSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    VocSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    VocSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    VocSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    VocSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    VocSyntaxKind::RightBrace
                }
                '?' => {
                    state.advance(1);
                    VocSyntaxKind::Question
                }
                '~' => {
                    state.advance(1);
                    VocSyntaxKind::Tilde
                }
                _ => {
                    state.advance(ch.len_utf8());
                    VocSyntaxKind::Error
                }
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 主要的词法分析循环
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
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

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            // 如果都没有匹配，则跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(VocSyntaxKind::Error, start_pos, state.get_position());
            }
        }
        Ok(())
    }
}
