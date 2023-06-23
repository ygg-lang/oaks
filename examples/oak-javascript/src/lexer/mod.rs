//! JavaScript lexer implementation.

pub mod token_type;

use crate::{language::JavaScriptLanguage, lexer::token_type::JavaScriptTokenType};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};
use std::simd::prelude::*;

type State<'a, S> = LexerState<'a, S, JavaScriptLanguage>;

/// JavaScript lexer.
#[derive(Clone, Debug)]
pub struct JavaScriptLexer<'config> {
    _config: &'config JavaScriptLanguage,
}

impl<'config> JavaScriptLexer<'config> {
    /// Creates a new JavaScript lexer.
    pub fn new(config: &'config JavaScriptLanguage) -> Self {
        Self { _config: config }
    }

    fn safe_check<'a, S: Source + ?Sized>(&self, state: &State<'a, S>) -> Result<(), OakError> {
        if state.get_position() <= state.get_length() { Ok(()) } else { Err(OakError::custom_error(format!("Lexer out-of-bounds: pos={}, len={}", state.get_position(), state.get_length()))) }
    }

    /// Main lexer run method.
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();
            self.safe_check(state)?;

            if let Some(ch) = state.peek() {
                match ch {
                    ' ' | '\t' => {
                        self.skip_whitespace(state);
                    }
                    '\n' | '\r' => {
                        self.lex_newline(state);
                    }
                    '/' => {
                        // Comment or Slash or SlashEqual
                        if let Some(next) = state.peek_next_n(1) {
                            if next == '/' || next == '*' {
                                self.lex_comment(state);
                            }
                            else {
                                self.lex_operator_or_punctuation(state);
                            }
                        }
                        else {
                            self.lex_operator_or_punctuation(state);
                        }
                    }
                    '"' | '\'' => {
                        self.lex_string_literal(state);
                    }
                    '`' => {
                        self.lex_template_literal(state);
                    }
                    '0'..='9' => {
                        self.lex_numeric_literal(state);
                    }
                    '.' => {
                        // Dot, DotDotDot, or Number (.5)
                        if self.is_next_digit(state) {
                            self.lex_numeric_literal(state);
                        }
                        else {
                            self.lex_operator_or_punctuation(state);
                        }
                    }
                    'a'..='z' | 'A'..='Z' | '_' | '$' => {
                        self.lex_identifier_or_keyword(state);
                    }
                    '+' | '-' | '*' | '%' | '<' | '>' | '=' | '!' | '&' | '|' | '^' | '~' | '?' | '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | ':' => {
                        self.lex_operator_or_punctuation(state);
                    }
                    _ => {
                        let start = state.get_position();
                        state.advance(ch.len_utf8());
                        state.add_token(JavaScriptTokenType::Error, start, state.get_position());
                    }
                }
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let bytes = state.rest_bytes();
        let mut i = 0;
        let len = bytes.len();
        const LANES: usize = 32;

        while i + LANES <= len {
            let chunk = Simd::<u8, LANES>::from_slice(unsafe { bytes.get_unchecked(i..i + LANES) });
            let is_space = chunk.simd_eq(Simd::splat(b' '));
            let is_tab = chunk.simd_eq(Simd::splat(b'\t'));
            let is_ws = is_space | is_tab;

            if !is_ws.all() {
                let not_ws = !is_ws;
                let idx = not_ws.first_set().unwrap();
                i += idx;
                state.advance(i);
                state.add_token(JavaScriptTokenType::Whitespace, start, state.get_position());
                return true;
            }
            i += LANES
        }

        while i < len {
            let ch = unsafe { *bytes.get_unchecked(i) };
            if ch != b' ' && ch != b'\t' {
                break;
            }
            i += 1
        }

        if i > 0 {
            state.advance(i);
            state.add_token(JavaScriptTokenType::Whitespace, start, state.get_position());
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
            state.add_token(JavaScriptTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(JavaScriptTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释（行注释和块注释）
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 行注释: // ... 直到换行
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }
            state.add_token(JavaScriptTokenType::LineComment, start, state.get_position());
            return true;
        }

        // 块注释: /* ... */
        if rest.starts_with("/*") {
            state.advance(2);
            let mut found_end = false;
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    found_end = true;
                    break;
                }
                state.advance(ch.len_utf8())
            }

            if !found_end {
                let error = OakError::syntax_error("Unterminated comment".to_string(), start, None);
                state.add_error(error)
            }

            state.add_token(JavaScriptTokenType::BlockComment, start, state.get_position());
            return true;
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(first_char) = state.peek() {
            if first_char == '"' || first_char == '\'' {
                let quote = first_char;
                state.advance(1);
                let mut found_end = false;

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if ch == '\\' {
                        // Skip escaped character
                        state.advance(1);
                        if let Some(escaped) = state.peek() {
                            state.advance(escaped.len_utf8())
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }

                if !found_end {
                    let error = OakError::syntax_error("Unterminated string literal".to_string(), start_pos, None);
                    state.add_error(error)
                }

                state.add_token(JavaScriptTokenType::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理模板字符串
    fn lex_template_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('`') = state.peek() {
            state.advance(1);
            let mut found_end = false;

            while let Some(ch) = state.peek() {
                if ch == '`' {
                    state.advance(1);
                    found_end = true;
                    break;
                }
                else if ch == '\\' {
                    // 处理转义字符
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8())
                    }
                }
                else if ch == '$' {
                    if let Some('{') = state.peek_next_n(1) {
                        // 模板表达式，暂时跳过
                        state.advance(2);
                        let mut brace_count = 1;
                        while let Some(inner_ch) = state.peek() {
                            if inner_ch == '{' {
                                brace_count += 1
                            }
                            else if inner_ch == '}' {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    state.advance(1);
                                    break;
                                }
                            }
                            state.advance(inner_ch.len_utf8())
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }

            if !found_end {
                let error = OakError::syntax_error("Unterminated template literal".to_string(), start_pos, None);
                state.add_error(error)
            }

            state.add_token(JavaScriptTokenType::TemplateString, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面量
    fn lex_numeric_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            // 十六进制数字 (0x 或 0X)
            if ch == '0' {
                if let Some(next) = state.peek_next_n(1) {
                    if next == 'x' || next == 'X' {
                        state.advance(2); // 跳过 '0x'
                        let mut has_digits = false;
                        while let Some(hex_ch) = state.peek() {
                            if hex_ch.is_ascii_hexdigit() {
                                state.advance(1);
                                has_digits = true
                            }
                            else {
                                break;
                            }
                        }

                        if !has_digits {
                            let error = OakError::syntax_error("Invalid hexadecimal number".to_string(), start_pos, None);
                            state.add_error(error)
                        }

                        // 检查 BigInt 后缀
                        if let Some('n') = state.peek() {
                            state.advance(1);
                            state.add_token(JavaScriptTokenType::BigIntLiteral, start_pos, state.get_position())
                        }
                        else {
                            state.add_token(JavaScriptTokenType::NumericLiteral, start_pos, state.get_position())
                        }
                        return true;
                    }
                }
            }

            // 普通数字或小数
            if ch.is_ascii_digit() || (ch == '.' && self.is_next_digit(state)) {
                // 处理整数部分
                if ch != '.' {
                    while let Some(digit) = state.peek() {
                        if digit.is_ascii_digit() { state.advance(1) } else { break }
                    }
                }

                // 处理小数部分
                if let Some('.') = state.peek() {
                    state.advance(1);
                    while let Some(digit) = state.peek() {
                        if digit.is_ascii_digit() { state.advance(1) } else { break }
                    }
                }

                // 处理指数部分
                if let Some(exp) = state.peek() {
                    if exp == 'e' || exp == 'E' {
                        state.advance(1);

                        // 可选的符号
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1)
                            }
                        }

                        // 必须有数字
                        let mut has_exp_digits = false;
                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_digit() {
                                state.advance(1);
                                has_exp_digits = true
                            }
                            else {
                                break;
                            }
                        }

                        if !has_exp_digits {
                            let error = OakError::syntax_error("Invalid number exponent".to_string(), start_pos, None);
                            state.add_error(error)
                        }
                    }
                }

                // 检查 BigInt 后缀
                if let Some('n') = state.peek() {
                    state.advance(1);
                    state.add_token(JavaScriptTokenType::BigIntLiteral, start_pos, state.get_position())
                }
                else {
                    state.add_token(JavaScriptTokenType::NumericLiteral, start_pos, state.get_position())
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

    /// 检查下一个字符是否是数字
    fn is_next_digit<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(next_ch) = state.peek_next_n(1) { next_ch.is_ascii_digit() } else { false }
    }

    /// 处理标识符或关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(next_ch) = state.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' || next_ch == '$' { state.advance(next_ch.len_utf8()) } else { break }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_kind = self.keyword_or_identifier(&text);
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

    /// 判断是关键字还是标识
    fn keyword_or_identifier(&self, text: &str) -> JavaScriptTokenType {
        JavaScriptTokenType::from_keyword(text).unwrap_or(JavaScriptTokenType::IdentifierName)
    }

    /// 处理操作符和标点符号
    fn lex_operator_or_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    match state.peek() {
                        Some('+') => {
                            state.advance(1);
                            JavaScriptTokenType::PlusPlus
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptTokenType::PlusEqual
                        }
                        _ => JavaScriptTokenType::Plus,
                    }
                }
                '-' => {
                    state.advance(1);
                    match state.peek() {
                        Some('-') => {
                            state.advance(1);
                            JavaScriptTokenType::MinusMinus
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptTokenType::MinusEqual
                        }
                        _ => JavaScriptTokenType::Minus,
                    }
                }
                '*' => {
                    state.advance(1);
                    match state.peek() {
                        Some('*') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptTokenType::StarStarEqual
                            }
                            else {
                                JavaScriptTokenType::StarStar
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptTokenType::StarEqual
                        }
                        _ => JavaScriptTokenType::Star,
                    }
                }
                '/' => {
                    // 检查是否是注释
                    if let Some(next) = state.peek_next_n(1) {
                        if next == '/' || next == '*' {
                            return false; // 让注释处理函数处理                        
                        }
                    }
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaScriptTokenType::SlashEqual
                    }
                    else {
                        JavaScriptTokenType::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaScriptTokenType::PercentEqual
                    }
                    else {
                        JavaScriptTokenType::Percent
                    }
                }
                '<' => {
                    state.advance(1);
                    match state.peek() {
                        Some('<') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptTokenType::LeftShiftEqual
                            }
                            else {
                                JavaScriptTokenType::LeftShift
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptTokenType::LessEqual
                        }
                        _ => JavaScriptTokenType::Less,
                    }
                }
                '>' => {
                    state.advance(1);
                    match state.peek() {
                        Some('>') => {
                            state.advance(1);
                            match state.peek() {
                                Some('>') => {
                                    state.advance(1);
                                    if let Some('=') = state.peek() {
                                        state.advance(1);
                                        JavaScriptTokenType::UnsignedRightShiftEqual
                                    }
                                    else {
                                        JavaScriptTokenType::UnsignedRightShift
                                    }
                                }
                                Some('=') => {
                                    state.advance(1);
                                    JavaScriptTokenType::RightShiftEqual
                                }
                                _ => JavaScriptTokenType::RightShift,
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptTokenType::GreaterEqual
                        }
                        _ => JavaScriptTokenType::Greater,
                    }
                }
                '=' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptTokenType::EqualEqualEqual
                            }
                            else {
                                JavaScriptTokenType::EqualEqual
                            }
                        }
                        Some('>') => {
                            state.advance(1);
                            JavaScriptTokenType::Arrow
                        }
                        _ => JavaScriptTokenType::Equal,
                    }
                }
                '!' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptTokenType::NotEqualEqual
                            }
                            else {
                                JavaScriptTokenType::NotEqual
                            }
                        }
                        _ => JavaScriptTokenType::Exclamation,
                    }
                }
                '&' => {
                    state.advance(1);
                    match state.peek() {
                        Some('&') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptTokenType::AmpersandAmpersandEqual
                            }
                            else {
                                JavaScriptTokenType::AmpersandAmpersand
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptTokenType::AmpersandEqual
                        }
                        _ => JavaScriptTokenType::Ampersand,
                    }
                }
                '|' => {
                    state.advance(1);
                    match state.peek() {
                        Some('|') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptTokenType::PipePipeEqual
                            }
                            else {
                                JavaScriptTokenType::PipePipe
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptTokenType::PipeEqual
                        }
                        _ => JavaScriptTokenType::Pipe,
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaScriptTokenType::CaretEqual
                    }
                    else {
                        JavaScriptTokenType::Caret
                    }
                }
                '~' => {
                    state.advance(1);
                    JavaScriptTokenType::Tilde
                }
                '?' => {
                    state.advance(1);
                    match state.peek() {
                        Some('?') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptTokenType::QuestionQuestionEqual
                            }
                            else {
                                JavaScriptTokenType::QuestionQuestion
                            }
                        }
                        Some('.') => {
                            state.advance(1);
                            JavaScriptTokenType::QuestionDot
                        }
                        _ => JavaScriptTokenType::Question,
                    }
                }
                '(' => {
                    state.advance(1);
                    JavaScriptTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    JavaScriptTokenType::RightParen
                }
                '{' => {
                    state.advance(1);
                    JavaScriptTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    JavaScriptTokenType::RightBrace
                }
                '[' => {
                    state.advance(1);
                    JavaScriptTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    JavaScriptTokenType::RightBracket
                }
                ';' => {
                    state.advance(1);
                    JavaScriptTokenType::Semicolon
                }
                ',' => {
                    state.advance(1);
                    JavaScriptTokenType::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        if let Some('.') = state.peek_next_n(1) {
                            state.advance(2);
                            JavaScriptTokenType::DotDotDot
                        }
                        else {
                            JavaScriptTokenType::Dot
                        }
                    }
                    else {
                        JavaScriptTokenType::Dot
                    }
                }
                ':' => {
                    state.advance(1);
                    JavaScriptTokenType::Colon
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

impl<'config> Lexer<JavaScriptLanguage> for JavaScriptLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<JavaScriptLanguage>) -> LexOutput<JavaScriptLanguage> {
        let mut state = LexerState::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}
