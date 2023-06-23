#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::VocLanguage, lexer::token_type::VocTokenType};
use oak_core::{
    Lexer, LexerState, Source,
    lexer::{LexOutput, LexerCache},
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
            state.add_token(VocTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(VocTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(VocTokenType::Newline, start_pos, state.get_position());
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

                state.add_token(VocTokenType::Comment, start_pos, state.get_position());
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

                state.add_token(VocTokenType::Comment, start_pos, state.get_position());
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

                let token_kind = if quote == '"' { VocTokenType::StringLiteral } else { VocTokenType::CharLiteral };
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

                let token_kind = if is_float { VocTokenType::FloatLiteral } else { VocTokenType::IntegerLiteral };
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
                use oak_valkyrie::ValkyrieKeywords as VK;
                let token_kind = match text.as_ref() {
                    "module" => VocTokenType::Keyword(VK::Namespace), // Map module to namespace
                    "import" => VocTokenType::Keyword(VK::Using),     // Map import to using
                    "pub" => VocTokenType::Identifier,                // pub is not in VK keywords yet
                    "fn" => VocTokenType::Keyword(VK::Micro),         // Map fn to micro
                    "struct" => VocTokenType::Keyword(VK::Class),     // Map struct to class
                    "interface" => VocTokenType::Keyword(VK::Trait),  // Map interface to trait
                    "enum" => VocTokenType::Keyword(VK::Enums),
                    "type" => VocTokenType::Keyword(VK::Type),
                    "const" => VocTokenType::Keyword(VK::Let),
                    "mut" => VocTokenType::Keyword(VK::Mut),
                    "if" => VocTokenType::Keyword(VK::If),
                    "else" => VocTokenType::Keyword(VK::Else),
                    "for" => VocTokenType::Keyword(VK::For),
                    "in" => VocTokenType::Keyword(VK::In),
                    "match" => VocTokenType::Keyword(VK::Match),
                    "return" => VocTokenType::Keyword(VK::Return),
                    "break" => VocTokenType::Keyword(VK::Break),
                    "continue" => VocTokenType::Keyword(VK::Continue),
                    "as" => VocTokenType::Keyword(VK::Is),
                    "true" | "false" => VocTokenType::BoolLiteral,
                    "bool" => VocTokenType::BoolKw,
                    "i8" => VocTokenType::I8Kw,
                    "i16" => VocTokenType::I16Kw,
                    "i32" => VocTokenType::I32Kw,
                    "i64" => VocTokenType::I64Kw,
                    "u8" => VocTokenType::U8Kw,
                    "u16" => VocTokenType::U16Kw,
                    "u32" => VocTokenType::U32Kw,
                    "u64" => VocTokenType::U64Kw,
                    "int" => VocTokenType::IntKw,
                    "uint" => VocTokenType::UintKw,
                    "f32" => VocTokenType::F32Kw,
                    "f64" => VocTokenType::F64Kw,
                    "string" => VocTokenType::StringKw,
                    "char" => VocTokenType::CharKw,
                    _ => VocTokenType::Identifier,
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
                        VocTokenType::PlusEq
                    }
                    else if let Some('+') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::PlusPlus
                    }
                    else {
                        state.advance(1);
                        VocTokenType::Plus
                    }
                }
                '-' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::MinusEq
                    }
                    else if let Some('-') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::MinusMinus
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::Arrow
                    }
                    else {
                        state.advance(1);
                        VocTokenType::Minus
                    }
                }
                '*' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::StarEq
                    }
                    else {
                        state.advance(1);
                        VocTokenType::Star
                    }
                }
                '/' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::SlashEq
                    }
                    else {
                        state.advance(1);
                        VocTokenType::Slash
                    }
                }
                '%' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::PercentEq
                    }
                    else {
                        state.advance(1);
                        VocTokenType::Percent
                    }
                }
                '&' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::AmpersandEq
                    }
                    else if let Some('&') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::AndAnd
                    }
                    else {
                        state.advance(1);
                        VocTokenType::Ampersand
                    }
                }
                '|' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::PipeEq
                    }
                    else if let Some('|') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::OrOr
                    }
                    else {
                        state.advance(1);
                        VocTokenType::Pipe
                    }
                }
                '^' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::CaretEq
                    }
                    else {
                        state.advance(1);
                        VocTokenType::Caret
                    }
                }
                '=' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::EqEq
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::FatArrow
                    }
                    else {
                        state.advance(1);
                        VocTokenType::Eq
                    }
                }
                '!' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::Ne
                    }
                    else {
                        state.advance(1);
                        VocTokenType::Bang
                    }
                }
                '<' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::Le
                    }
                    else if let Some('<') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            state.advance(3);
                            VocTokenType::LeftShiftEq
                        }
                        else {
                            state.advance(2);
                            VocTokenType::LeftShift
                        }
                    }
                    else {
                        state.advance(1);
                        VocTokenType::LessThan
                    }
                }
                '>' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VocTokenType::Ge
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            state.advance(3);
                            VocTokenType::RightShiftEq
                        }
                        else {
                            state.advance(2);
                            VocTokenType::RightShift
                        }
                    }
                    else {
                        state.advance(1);
                        VocTokenType::GreaterThan
                    }
                }
                '.' => {
                    if let Some('.') = state.peek_next_n(1) {
                        if let Some('.') = state.peek_next_n(2) {
                            state.advance(3);
                            VocTokenType::DotDotDot
                        }
                        else {
                            state.advance(2);
                            VocTokenType::DotDot
                        }
                    }
                    else {
                        state.advance(1);
                        VocTokenType::Dot
                    }
                }
                ',' => {
                    state.advance(1);
                    VocTokenType::Comma
                }
                ':' => {
                    state.advance(1);
                    VocTokenType::Colon
                }
                ';' => {
                    state.advance(1);
                    VocTokenType::Semicolon
                }
                '(' => {
                    state.advance(1);
                    VocTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    VocTokenType::RightParen
                }
                '[' => {
                    state.advance(1);
                    VocTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    VocTokenType::RightBracket
                }
                '{' => {
                    state.advance(1);
                    VocTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    VocTokenType::RightBrace
                }
                '?' => {
                    state.advance(1);
                    VocTokenType::Question
                }
                '~' => {
                    state.advance(1);
                    VocTokenType::Tilde
                }
                _ => {
                    state.advance(ch.len_utf8());
                    VocTokenType::Error
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
        match self._config.mode {
            crate::language::VocMode::Programming => self.run_programming(state),
            crate::language::VocMode::Component => self.run_component(state),
        }
    }

    fn run_programming<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
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
                state.add_token(VocTokenType::Error, start_pos, state.get_position());
            }
        }

        Ok(())
    }

    fn run_component<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let start = state.get_position();
        // Find the end of the source
        while let Some(ch) = state.peek() {
            state.advance(ch.len_utf8());
        }
        let end = state.get_position();

        // Reset to start and lex with interpolation/template support
        state.set_position(start);
        self.lex_template_content(state, start, end);

        Ok(())
    }

    fn lex_template_content<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, start: usize, end: usize) {
        let mut current = start;
        state.set_position(start);

        while state.get_position() < end {
            // Handle escaped characters
            if state.peek() == Some('\\') {
                state.advance(1);
                if let Some(c) = state.peek() {
                    state.advance(c.len_utf8());
                }
                continue;
            }

            // Handle <% template control %>
            if state.starts_with("<%") {
                let text_end = state.get_position();
                if current < text_end {
                    state.add_token(VocTokenType::TextPart, current, text_end);
                }

                let control_start = state.get_position();
                state.advance(2); // skip <%
                state.add_token(VocTokenType::TemplateControlStart, control_start, state.get_position());

                // Find matching %>
                while state.get_position() < end {
                    if state.starts_with("%>") {
                        let control_end = state.get_position();
                        state.advance(2);
                        state.add_token(VocTokenType::TemplateControlEnd, control_end, state.get_position());
                        break;
                    }
                    if let Some(c) = state.peek() {
                        state.advance(c.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                current = state.get_position();
                continue;
            }

            // Handle { interpolation }
            if state.peek() == Some('{') {
                let text_end = state.get_position();
                if current < text_end {
                    state.add_token(VocTokenType::TextPart, current, text_end);
                }

                let interp_start = state.get_position();
                state.advance(1); // skip {
                state.add_token(VocTokenType::InterpolationStart, interp_start, state.get_position());

                // Find matching }
                let mut depth = 1;
                while depth > 0 && state.get_position() < end {
                    if let Some(c) = state.peek() {
                        if c == '{' {
                            depth += 1;
                        }
                        else if c == '}' {
                            depth -= 1;
                            if depth == 0 {
                                let interp_end = state.get_position();
                                state.advance(1);
                                state.add_token(VocTokenType::InterpolationEnd, interp_end, state.get_position());
                                break;
                            }
                        }
                        state.advance(c.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                current = state.get_position();
                continue;
            }

            // Handle XML-like tags
            if state.peek() == Some('<') {
                let text_end = state.get_position();
                if current < text_end {
                    state.add_token(VocTokenType::TextPart, current, text_end);
                }

                let tag_start = state.get_position();
                state.advance(1); // skip <

                if state.peek() == Some('/') {
                    state.advance(1);
                    state.add_token(VocTokenType::TagOpen, tag_start, state.get_position());
                    state.add_token(VocTokenType::TagSlash, tag_start + 1, state.get_position());
                }
                else {
                    state.add_token(VocTokenType::TagOpen, tag_start, state.get_position());
                }

                // Lex tag name
                let name_start = state.get_position();
                if self.lex_identifier_or_keyword(state) {
                    // Re-tag the identifier as TagName if needed, but for now just use Identifier
                }

                // Lex attributes until > or />
                while state.get_position() < end {
                    self.skip_whitespace(state);
                    if state.starts_with("/>") {
                        let self_close_start = state.get_position();
                        state.advance(2);
                        state.add_token(VocTokenType::TagSelfClose, self_close_start, state.get_position());
                        break;
                    }
                    if state.peek() == Some('>') {
                        let close_start = state.get_position();
                        state.advance(1);
                        state.add_token(VocTokenType::TagClose, close_start, state.get_position());
                        break;
                    }

                    // Lex attribute
                    if self.lex_identifier_or_keyword(state) {
                        if state.peek() == Some('=') {
                            let eq_start = state.get_position();
                            state.advance(1);
                            state.add_token(VocTokenType::AttrEq, eq_start, state.get_position());
                            self.lex_string(state);
                        }
                    }
                    else if let Some(c) = state.peek() {
                        state.advance(c.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                current = state.get_position();
                continue;
            }

            // Regular character
            if let Some(c) = state.peek() {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        if current < end {
            state.add_token(VocTokenType::TextPart, current, end);
        }
    }
}
