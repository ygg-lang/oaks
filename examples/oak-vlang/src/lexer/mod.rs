#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::VLangLanguage, lexer::token_type::VLangTokenType};
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
            state.add_token(VLangTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(VLangTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(VLangTokenType::Newline, start_pos, state.get_position());
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

                state.add_token(VLangTokenType::Comment, start_pos, state.get_position());
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

                state.add_token(VLangTokenType::Comment, start_pos, state.get_position());
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

                let token_kind = if quote == '"' { VLangTokenType::StringLiteral } else { VLangTokenType::CharLiteral };
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

                let token_kind = if is_float { VLangTokenType::FloatLiteral } else { VLangTokenType::IntegerLiteral };
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
                    "module" => VLangTokenType::ModuleKw,
                    "import" => VLangTokenType::ImportKw,
                    "pub" => VLangTokenType::PubKw,
                    "fn" => VLangTokenType::FnKw,
                    "struct" => VLangTokenType::StructKw,
                    "interface" => VLangTokenType::InterfaceKw,
                    "enum" => VLangTokenType::EnumKw,
                    "type" => VLangTokenType::TypeKw,
                    "const" => VLangTokenType::ConstKw,
                    "mut" => VLangTokenType::MutKw,
                    "shared" => VLangTokenType::SharedKw,
                    "volatile" => VLangTokenType::VolatileKw,
                    "unsafe" => VLangTokenType::UnsafeKw,
                    "if" => VLangTokenType::IfKw,
                    "else" => VLangTokenType::ElseKw,
                    "for" => VLangTokenType::ForKw,
                    "in" => VLangTokenType::InKw,
                    "match" => VLangTokenType::MatchKw,
                    "or" => VLangTokenType::OrKw,
                    "return" => VLangTokenType::ReturnKw,
                    "break" => VLangTokenType::BreakKw,
                    "continue" => VLangTokenType::ContinueKw,
                    "goto" => VLangTokenType::GotoKw,
                    "defer" => VLangTokenType::DeferKw,
                    "go" => VLangTokenType::GoKw,
                    "select" => VLangTokenType::SelectKw,
                    "lock" => VLangTokenType::LockKw,
                    "rlock" => VLangTokenType::RlockKw,
                    "as" => VLangTokenType::AsKw,
                    "is" => VLangTokenType::IsKw,
                    "sizeof" => VLangTokenType::SizeofKw,
                    "typeof" => VLangTokenType::TypeofKw,
                    "offsetof" => VLangTokenType::OffsetofKw,
                    "assert" => VLangTokenType::AssertKw,
                    "panic" => VLangTokenType::PanicKw,
                    "eprintln" => VLangTokenType::EprintlnKw,
                    "println" => VLangTokenType::PrintlnKw,
                    "print" => VLangTokenType::PrintKw,
                    "eprint" => VLangTokenType::EprintKw,
                    "bool" => VLangTokenType::BoolKw,
                    "i8" => VLangTokenType::I8Kw,
                    "i16" => VLangTokenType::I16Kw,
                    "i32" => VLangTokenType::I32Kw,
                    "i64" => VLangTokenType::I64Kw,
                    "u8" => VLangTokenType::U8Kw,
                    "u16" => VLangTokenType::U16Kw,
                    "u32" => VLangTokenType::U32Kw,
                    "u64" => VLangTokenType::U64Kw,
                    "int" => VLangTokenType::IntKw,
                    "uint" => VLangTokenType::UintKw,
                    "f32" => VLangTokenType::F32Kw,
                    "f64" => VLangTokenType::F64Kw,
                    "string" => VLangTokenType::StringKw,
                    "rune" => VLangTokenType::RuneKw,
                    "byte" => VLangTokenType::ByteKw,
                    "voidptr" => VLangTokenType::VoidptrKw,
                    "char" => VLangTokenType::CharKw,
                    "true" | "false" => VLangTokenType::BoolLiteral,
                    _ => VLangTokenType::Identifier,
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
                        VLangTokenType::PlusEq
                    }
                    else if let Some('+') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::PlusPlus
                    }
                    else {
                        state.advance(1);
                        VLangTokenType::Plus
                    }
                }
                '-' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::MinusEq
                    }
                    else if let Some('-') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::MinusMinus
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::Arrow
                    }
                    else {
                        state.advance(1);
                        VLangTokenType::Minus
                    }
                }
                '*' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::StarEq
                    }
                    else {
                        state.advance(1);
                        VLangTokenType::Star
                    }
                }
                '/' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::SlashEq
                    }
                    else {
                        state.advance(1);
                        VLangTokenType::Slash
                    }
                }
                '%' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::PercentEq
                    }
                    else {
                        state.advance(1);
                        VLangTokenType::Percent
                    }
                }
                '&' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::AmpersandEq
                    }
                    else if let Some('&') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::AndAnd
                    }
                    else {
                        state.advance(1);
                        VLangTokenType::Ampersand
                    }
                }
                '|' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::PipeEq
                    }
                    else if let Some('|') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::OrOr
                    }
                    else {
                        state.advance(1);
                        VLangTokenType::Pipe
                    }
                }
                '^' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::CaretEq
                    }
                    else {
                        state.advance(1);
                        VLangTokenType::Caret
                    }
                }
                '=' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::EqEq
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::FatArrow
                    }
                    else {
                        state.advance(1);
                        VLangTokenType::Eq
                    }
                }
                '!' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::Ne
                    }
                    else {
                        state.advance(1);
                        VLangTokenType::Bang
                    }
                }
                '<' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::Le
                    }
                    else if let Some('<') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            state.advance(3);
                            VLangTokenType::LeftShiftEq
                        }
                        else {
                            state.advance(2);
                            VLangTokenType::LeftShift
                        }
                    }
                    else {
                        state.advance(1);
                        VLangTokenType::LessThan
                    }
                }
                '>' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VLangTokenType::Ge
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            state.advance(3);
                            VLangTokenType::RightShiftEq
                        }
                        else {
                            state.advance(2);
                            VLangTokenType::RightShift
                        }
                    }
                    else {
                        state.advance(1);
                        VLangTokenType::GreaterThan
                    }
                }
                '.' => {
                    if let Some('.') = state.peek_next_n(1) {
                        if let Some('.') = state.peek_next_n(2) {
                            state.advance(3);
                            VLangTokenType::DotDotDot
                        }
                        else {
                            state.advance(2);
                            VLangTokenType::DotDot
                        }
                    }
                    else {
                        state.advance(1);
                        VLangTokenType::Dot
                    }
                }
                ',' => {
                    state.advance(1);
                    VLangTokenType::Comma
                }
                ':' => {
                    state.advance(1);
                    VLangTokenType::Colon
                }
                ';' => {
                    state.advance(1);
                    VLangTokenType::Semicolon
                }
                '(' => {
                    state.advance(1);
                    VLangTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    VLangTokenType::RightParen
                }
                '[' => {
                    state.advance(1);
                    VLangTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    VLangTokenType::RightBracket
                }
                '{' => {
                    state.advance(1);
                    VLangTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    VLangTokenType::RightBrace
                }
                '?' => {
                    state.advance(1);
                    VLangTokenType::Question
                }
                '~' => {
                    state.advance(1);
                    VLangTokenType::Tilde
                }
                _ => {
                    state.advance(ch.len_utf8());
                    VLangTokenType::Error
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
                state.add_token(VLangTokenType::Error, start_pos, state.get_position());
            }
        }

        state.add_eof();
        state.finish_with_cache(Ok(()), cache)
    }
}
