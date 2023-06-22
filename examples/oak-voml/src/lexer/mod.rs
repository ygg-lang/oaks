use crate::{kind::VLangSyntaxKind, language::VLangLanguage};
use oak_core::{
    Lexer, LexerState,
    lexer::{LexOutput, LexerCache},
    source::Source,
};

type State<'a, S> = LexerState<'a, S, VLangLanguage>;

#[derive(Clone, Debug)]
pub struct VLangLexer<'config> {
    _config: &'config VLangLanguage,
}

impl<'config> VLangLexer<'config> {
    pub fn new(config: &'config VLangLanguage) -> Self {
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
            state.add_token(VLangSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(VLangSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(VLangSyntaxKind::Newline, start_pos, state.get_position());
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

                state.add_token(VLangSyntaxKind::Comment, start_pos, state.get_position());
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

                state.add_token(VLangSyntaxKind::Comment, start_pos, state.get_position());
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

                let token_kind = if quote == '"' { VLangSyntaxKind::StringLiteral } else { VLangSyntaxKind::CharLiteral };
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

                let token_kind = if is_float { VLangSyntaxKind::FloatLiteral } else { VLangSyntaxKind::IntegerLiteral };
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
                    "module" => VLangSyntaxKind::ModuleKw,
                    "import" => VLangSyntaxKind::ImportKw,
                    "pub" => VLangSyntaxKind::PubKw,
                    "fn" => VLangSyntaxKind::FnKw,
                    "struct" => VLangSyntaxKind::StructKw,
                    "interface" => VLangSyntaxKind::InterfaceKw,
                    "enum" => VLangSyntaxKind::EnumKw,
                    "type" => VLangSyntaxKind::TypeKw,
                    "const" => VLangSyntaxKind::ConstKw,
                    "mut" => VLangSyntaxKind::MutKw,
                    "shared" => VLangSyntaxKind::SharedKw,
                    "volatile" => VLangSyntaxKind::VolatileKw,
                    "unsafe" => VLangSyntaxKind::UnsafeKw,
                    "if" => VLangSyntaxKind::IfKw,
                    "else" => VLangSyntaxKind::ElseKw,
                    "for" => VLangSyntaxKind::ForKw,
                    "in" => VLangSyntaxKind::InKw,
                    "match" => VLangSyntaxKind::MatchKw,
                    "or" => VLangSyntaxKind::OrKw,
                    "return" => VLangSyntaxKind::ReturnKw,
                    "break" => VLangSyntaxKind::BreakKw,
                    "continue" => VLangSyntaxKind::ContinueKw,
                    "goto" => VLangSyntaxKind::GotoKw,
                    "defer" => VLangSyntaxKind::DeferKw,
                    "go" => VLangSyntaxKind::GoKw,
                    "select" => VLangSyntaxKind::SelectKw,
                    "lock" => VLangSyntaxKind::LockKw,
                    "rlock" => VLangSyntaxKind::RlockKw,
                    "as" => VLangSyntaxKind::AsKw,
                    "is" => VLangSyntaxKind::IsKw,
                    "sizeof" => VLangSyntaxKind::SizeofKw,
                    "typeof" => VLangSyntaxKind::TypeofKw,
                    "offsetof" => VLangSyntaxKind::OffsetofKw,
                    "assert" => VLangSyntaxKind::AssertKw,
                    "panic" => VLangSyntaxKind::PanicKw,
                    "eprintln" => VLangSyntaxKind::EprintlnKw,
                    "println" => VLangSyntaxKind::PrintlnKw,
                    "print" => VLangSyntaxKind::PrintKw,
                    "eprint" => VLangSyntaxKind::EprintKw,
                    "bool" => VLangSyntaxKind::BoolKw,
                    "i8" => VLangSyntaxKind::I8Kw,
                    "i16" => VLangSyntaxKind::I16Kw,
                    "i32" => VLangSyntaxKind::I32Kw,
                    "i64" => VLangSyntaxKind::I64Kw,
                    "u8" => VLangSyntaxKind::U8Kw,
                    "u16" => VLangSyntaxKind::U16Kw,
                    "u32" => VLangSyntaxKind::U32Kw,
                    "u64" => VLangSyntaxKind::U64Kw,
                    "int" => VLangSyntaxKind::IntKw,
                    "uint" => VLangSyntaxKind::UintKw,
                    "f32" => VLangSyntaxKind::F32Kw,
                    "f64" => VLangSyntaxKind::F64Kw,
                    "string" => VLangSyntaxKind::StringKw,
                    "rune" => VLangSyntaxKind::RuneKw,
                    "byte" => VLangSyntaxKind::ByteKw,
                    "voidptr" => VLangSyntaxKind::VoidptrKw,
                    "char" => VLangSyntaxKind::CharKw,
                    "true" | "false" => VLangSyntaxKind::BoolLiteral,
                    _ => VLangSyntaxKind::Identifier,
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
                        VLangSyntaxKind::PlusEq
                    }
                    else if let Some('+') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::PlusPlus
                    }
                    else {
                        state.advance(1);
                        VLangSyntaxKind::Plus
                    }
                }
                '-' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::MinusEq
                    }
                    else if let Some('-') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::MinusMinus
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::Arrow
                    }
                    else {
                        state.advance(1);
                        VLangSyntaxKind::Minus
                    }
                }
                '*' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::StarEq
                    }
                    else {
                        state.advance(1);
                        VLangSyntaxKind::Star
                    }
                }
                '/' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::SlashEq
                    }
                    else {
                        state.advance(1);
                        VLangSyntaxKind::Slash
                    }
                }
                '%' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::PercentEq
                    }
                    else {
                        state.advance(1);
                        VLangSyntaxKind::Percent
                    }
                }
                '&' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::AmpersandEq
                    }
                    else if let Some('&') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::AndAnd
                    }
                    else {
                        state.advance(1);
                        VLangSyntaxKind::Ampersand
                    }
                }
                '|' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::PipeEq
                    }
                    else if let Some('|') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::OrOr
                    }
                    else {
                        state.advance(1);
                        VLangSyntaxKind::Pipe
                    }
                }
                '^' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::CaretEq
                    }
                    else {
                        state.advance(1);
                        VLangSyntaxKind::Caret
                    }
                }
                '=' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::EqEq
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::FatArrow
                    }
                    else {
                        state.advance(1);
                        VLangSyntaxKind::Eq
                    }
                }
                '!' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::Ne
                    }
                    else {
                        state.advance(1);
                        VLangSyntaxKind::Bang
                    }
                }
                '<' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::Le
                    }
                    else if let Some('<') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            state.advance(3);
                            VLangSyntaxKind::LeftShiftEq
                        }
                        else {
                            state.advance(2);
                            VLangSyntaxKind::LeftShift
                        }
                    }
                    else {
                        state.advance(1);
                        VLangSyntaxKind::LessThan
                    }
                }
                '>' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangSyntaxKind::Ge
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            state.advance(3);
                            VLangSyntaxKind::RightShiftEq
                        }
                        else {
                            state.advance(2);
                            VLangSyntaxKind::RightShift
                        }
                    }
                    else {
                        state.advance(1);
                        VLangSyntaxKind::GreaterThan
                    }
                }
                '.' => {
                    if let Some('.') = state.peek_next_n(1) {
                        if let Some('.') = state.peek_next_n(2) {
                            state.advance(3);
                            VLangSyntaxKind::DotDotDot
                        }
                        else {
                            state.advance(2);
                            VLangSyntaxKind::DotDot
                        }
                    }
                    else {
                        state.advance(1);
                        VLangSyntaxKind::Dot
                    }
                }
                ',' => {
                    state.advance(1);
                    VLangSyntaxKind::Comma
                }
                ':' => {
                    state.advance(1);
                    VLangSyntaxKind::Colon
                }
                ';' => {
                    state.advance(1);
                    VLangSyntaxKind::Semicolon
                }
                '(' => {
                    state.advance(1);
                    VLangSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    VLangSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    VLangSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    VLangSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    VLangSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    VLangSyntaxKind::RightBrace
                }
                '?' => {
                    state.advance(1);
                    VLangSyntaxKind::Question
                }
                '~' => {
                    state.advance(1);
                    VLangSyntaxKind::Tilde
                }
                _ => {
                    state.advance(ch.len_utf8());
                    VLangSyntaxKind::Error
                }
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<VLangLanguage> for VLangLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<VLangLanguage>) -> LexOutput<VLangLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);

        while let Some(_ch) = state.peek() {
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

            // 如果都没有匹配，则跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(VLangSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        state.add_eof();
        state.finish_with_cache(Ok(()), cache)
    }
}
