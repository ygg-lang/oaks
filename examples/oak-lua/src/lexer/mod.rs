//! Lua 词法分析
//!
//! 实现Lua 语言的词法分析，将源代码转换token 序列

use crate::{kind::LuaSyntaxKind, language::LuaLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, OakError, lexer::LexOutput, source::Source};

type State<S> = LexerState<S, LuaLanguage>;

/// Lua 词法分析
#[derive(Clone)]
pub struct LuaLexer<'config> {
    config: &'config LuaLanguage,
}

impl<'config> LuaLexer<'config> {
    /// 创建新的 Lua 词法分析
    pub fn new(config: &'config LuaLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
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
            state.add_token(LuaSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(LuaSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(LuaSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('-') = state.current() {
            if let Some('-') = state.peek() {
                state.advance(1); // 第一'-'
                state.advance(1); // 第二'-'

                // 检查是否是长注--[[
                if let Some('[') = state.current() {
                    if let Some('[') = state.peek() {
                        state.advance(1); // '['
                        state.advance(1); // '['

                        // 寻找 ]]
                        while let Some(ch) = state.current() {
                            if ch == ']' {
                                if let Some(']') = state.peek() {
                                    state.advance(1); // ']'
                                    state.advance(1); // ']'
                                    break;
                                }
                            }
                            state.advance(ch.len_utf8());
                        }
                    }
                    else {
                        // 单行注释，读到行
                        while let Some(ch) = state.current() {
                            if ch == '\n' || ch == '\r' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                    }
                }
                else {
                    // 单行注释，读到行
                    while let Some(ch) = state.current() {
                        if ch == '\n' || ch == '\r' {
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(LuaSyntaxKind::Comment, start_pos, state.get_position());
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
    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote_char) = state.current() {
            if quote_char == '"' || quote_char == '\'' {
                state.advance(1); // 跳过开始引

                let mut escaped = false;
                while let Some(ch) = state.current() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8());
                    }
                    else if ch == '\\' {
                        escaped = true;
                        state.advance(1);
                    }
                    else if ch == quote_char {
                        state.advance(1); // 跳过结束引号
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        // 字符串不能跨行（除非转义
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(LuaSyntaxKind::String, start_pos, state.get_position());
                true
            }
            else if quote_char == '[' {
                // 长字符串 [[...]]
                if let Some('[') = state.peek() {
                    state.advance(1); // '['
                    state.advance(1); // '['

                    // 寻找 ]]
                    while let Some(ch) = state.current() {
                        if ch == ']' {
                            if let Some(']') = state.peek() {
                                state.advance(1); // ']'
                                state.advance(1); // ']'
                                break;
                            }
                        }
                        state.advance(ch.len_utf8());
                    }

                    state.add_token(LuaSyntaxKind::String, start_pos, state.get_position());
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
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.current() {
            if ch.is_ascii_digit() {
                // 检查是否是十六进制
                if ch == '0' {
                    if let Some(next_ch) = state.peek() {
                        if next_ch == 'x' || next_ch == 'X' {
                            state.advance(1); // '0'
                            state.advance(1); // 'x' 'X'

                            // 读取十六进制数字
                            while let Some(hex_ch) = state.current() {
                                if hex_ch.is_ascii_hexdigit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }

                            state.add_token(LuaSyntaxKind::Number, start_pos, state.get_position());
                            return true;
                        }
                    }
                }

                // 普通数
                let mut has_dot = false;
                let mut has_exp = false;

                while let Some(num_ch) = state.current() {
                    if num_ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if num_ch == '.' && !has_dot && !has_exp {
                        has_dot = true;
                        state.advance(1);
                    }
                    else if (num_ch == 'e' || num_ch == 'E') && !has_exp {
                        has_exp = true;
                        state.advance(1);

                        // 可选的符号
                        if let Some(sign_ch) = state.current() {
                            if sign_ch == '+' || sign_ch == '-' {
                                state.advance(1);
                            }
                        }
                    }
                    else {
                        break;
                    }
                }

                state.add_token(LuaSyntaxKind::Number, start_pos, state.get_position());
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

    /// 处理标识符或关键
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.current() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                let range = state.take_while(|c| c.is_ascii_alphanumeric() || c == '_');
                // 使用 Source trait 的 get_text_in 方法
                let text = state.get_text_in(range);
                let token_kind = self.keyword_or_identifier(text);
                state.add_token(token_kind, range.start, range.end);
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

    /// 识别关键
    fn keyword_or_identifier(&self, text: &str) -> LuaSyntaxKind {
        match text {
            "and" => LuaSyntaxKind::And,
            "break" => LuaSyntaxKind::Break,
            "do" => LuaSyntaxKind::Do,
            "else" => LuaSyntaxKind::Else,
            "elseif" => LuaSyntaxKind::Elseif,
            "end" => LuaSyntaxKind::End,
            "false" => LuaSyntaxKind::False,
            "for" => LuaSyntaxKind::For,
            "function" => LuaSyntaxKind::Function,
            "goto" => LuaSyntaxKind::Goto,
            "if" => LuaSyntaxKind::If,
            "in" => LuaSyntaxKind::In,
            "local" => LuaSyntaxKind::Local,
            "nil" => LuaSyntaxKind::Nil,
            "not" => LuaSyntaxKind::Not,
            "or" => LuaSyntaxKind::Or,
            "repeat" => LuaSyntaxKind::Repeat,
            "return" => LuaSyntaxKind::Return,
            "then" => LuaSyntaxKind::Then,
            "true" => LuaSyntaxKind::True,
            "until" => LuaSyntaxKind::Until,
            "while" => LuaSyntaxKind::While,
            _ => LuaSyntaxKind::Identifier,
        }
    }

    /// 处理操作符和分隔
    fn lex_operator_or_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        LuaSyntaxKind::EqEq
                    }
                    else {
                        LuaSyntaxKind::Eq
                    }
                }
                '~' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        LuaSyntaxKind::TildeEq
                    }
                    else {
                        LuaSyntaxKind::Tilde
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        LuaSyntaxKind::LtEq
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        LuaSyntaxKind::LtLt
                    }
                    else {
                        LuaSyntaxKind::Lt
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        LuaSyntaxKind::GtEq
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        LuaSyntaxKind::GtGt
                    }
                    else {
                        LuaSyntaxKind::Gt
                    }
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        if let Some('.') = state.peek() {
                            state.advance(1);
                            LuaSyntaxKind::DotDotDot
                        }
                        else {
                            LuaSyntaxKind::DotDot
                        }
                    }
                    else {
                        LuaSyntaxKind::Dot
                    }
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        LuaSyntaxKind::ColonColon
                    }
                    else {
                        LuaSyntaxKind::Colon
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        LuaSyntaxKind::SlashSlash
                    }
                    else {
                        LuaSyntaxKind::Slash
                    }
                }
                '+' => {
                    state.advance(1);
                    LuaSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    LuaSyntaxKind::Minus
                }
                '*' => {
                    state.advance(1);
                    LuaSyntaxKind::Star
                }
                '%' => {
                    state.advance(1);
                    LuaSyntaxKind::Percent
                }
                '^' => {
                    state.advance(1);
                    LuaSyntaxKind::Caret
                }
                '#' => {
                    state.advance(1);
                    LuaSyntaxKind::Hash
                }
                '&' => {
                    state.advance(1);
                    LuaSyntaxKind::Ampersand
                }
                '|' => {
                    state.advance(1);
                    LuaSyntaxKind::Pipe
                }
                '(' => {
                    state.advance(1);
                    LuaSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    LuaSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    LuaSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    LuaSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    LuaSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    LuaSyntaxKind::RightBracket
                }
                ';' => {
                    state.advance(1);
                    LuaSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    LuaSyntaxKind::Comma
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

impl<'config> Lexer<LuaLanguage> for LuaLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        _offset: usize,
        _cache: IncrementalCache<LuaLanguage>,
    ) -> LexOutput<LuaLanguage> {
        let mut state = LexerState::new_with_cache(source, _offset, _cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> LuaLexer<'config> {
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        loop {
            // 尝试各种词法规则
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

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            // 如果所有规则都不匹配，检查是否到达文件末尾
            if let Some(ch) = state.current() {
                // 跳过当前字符并标记为错误
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(LuaSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                // 到达文件末尾，退出循环
                break;
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(LuaSyntaxKind::Eof, eof_pos, eof_pos);

        Ok(())
    }
}
