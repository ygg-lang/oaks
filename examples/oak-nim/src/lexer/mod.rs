#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::NimLanguage, lexer::token_type::NimTokenType};
use oak_core::{Lexer, LexerCache, LexerState, lexer::LexOutput, source::Source};
use std::borrow::Cow;

type State<'s, S> = LexerState<'s, S, NimLanguage>;

#[derive(Clone, Debug)]
pub struct NimLexer<'config> {
    _config: &'config NimLanguage,
}

impl<'config> NimLexer<'config> {
    pub fn new(config: &'config NimLanguage) -> Self {
        Self { _config: config }
    }

    /// 跳过空白字符
    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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
            state.add_token(NimTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(NimTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(NimTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // 检查是否是文档注释 ##
            if let Some('#') = state.peek() {
                state.advance(1);
            }

            // 读取到行
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            let kind = NimTokenType::CommentToken;

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                if ch == '\\' {
                    state.advance(1);
                    if let Some(c) = state.peek() {
                        state.advance(c.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(NimTokenType::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符字面量
    fn lex_char<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            if let Some('\\') = state.peek() {
                state.advance(1);
                if let Some(c) = state.peek() {
                    state.advance(c.len_utf8());
                }
            }
            else if let Some(c) = state.peek() {
                if c != '\'' {
                    state.advance(c.len_utf8());
                }
            }

            if let Some('\'') = state.peek() {
                state.advance(1);
            }

            state.add_token(NimTokenType::CharLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 简单的浮点数处理
                let mut is_float = false;
                if let Some('.') = state.peek() {
                    state.advance(1);
                    is_float = true;
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }

                let kind = if is_float { NimTokenType::FloatLiteral } else { NimTokenType::IntLiteral };
                state.add_token(kind, start_pos, state.get_position());
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
    fn lex_identifier<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let kind = match text {
                    Cow::Borrowed("and") => NimTokenType::AndKeyword,
                    Cow::Borrowed("or") => NimTokenType::OrKeyword,
                    Cow::Borrowed("not") => NimTokenType::NotKeyword,
                    Cow::Borrowed("if") => NimTokenType::IfKeyword,
                    Cow::Borrowed("else") => NimTokenType::ElseKeyword,
                    Cow::Borrowed("elif") => NimTokenType::ElifKeyword,
                    Cow::Borrowed("while") => NimTokenType::WhileKeyword,
                    Cow::Borrowed("for") => NimTokenType::ForKeyword,
                    Cow::Borrowed("proc") => NimTokenType::ProcKeyword,
                    Cow::Borrowed("func") => NimTokenType::FuncKeyword,
                    Cow::Borrowed("var") => NimTokenType::VarKeyword,
                    Cow::Borrowed("let") => NimTokenType::LetKeyword,
                    Cow::Borrowed("const") => NimTokenType::ConstKeyword,
                    Cow::Borrowed("type") => NimTokenType::TypeKeyword,
                    Cow::Borrowed("import") => NimTokenType::ImportKeyword,
                    Cow::Borrowed("from") => NimTokenType::FromKeyword,
                    Cow::Borrowed("include") => NimTokenType::IncludeKeyword,
                    Cow::Borrowed("return") => NimTokenType::ReturnKeyword,
                    Cow::Borrowed("yield") => NimTokenType::YieldKeyword,
                    Cow::Borrowed("break") => NimTokenType::BreakKeyword,
                    Cow::Borrowed("continue") => NimTokenType::ContinueKeyword,
                    Cow::Borrowed("try") => NimTokenType::TryKeyword,
                    Cow::Borrowed("except") => NimTokenType::ExceptKeyword,
                    Cow::Borrowed("finally") => NimTokenType::FinallyKeyword,
                    Cow::Borrowed("raise") => NimTokenType::RaiseKeyword,
                    Cow::Borrowed("case") => NimTokenType::CaseKeyword,
                    Cow::Borrowed("of") => NimTokenType::OfKeyword,
                    Cow::Borrowed("when") => NimTokenType::WhenKeyword,
                    Cow::Borrowed("is") => NimTokenType::IsKeyword,
                    Cow::Borrowed("in") => NimTokenType::InKeyword,
                    Cow::Borrowed("nil") => NimTokenType::NilKeyword,
                    _ => NimTokenType::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
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

    /// 处理操作符
    fn lex_operator<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            match ch {
                '+' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Plus, start_pos, state.get_position());
                    true
                }
                '-' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Minus, start_pos, state.get_position());
                    true
                }
                '*' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Star, start_pos, state.get_position());
                    true
                }
                '/' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Slash, start_pos, state.get_position());
                    true
                }
                '=' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimTokenType::EqualEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimTokenType::Equal, start_pos, state.get_position());
                    }
                    true
                }
                '!' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimTokenType::NotEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimTokenType::Error, start_pos, state.get_position());
                    }
                    true
                }
                '<' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimTokenType::LessEqual, start_pos, state.get_position());
                    }
                    else if state.peek() == Some('<') {
                        state.advance(1);
                        state.add_token(NimTokenType::LeftShift, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimTokenType::Less, start_pos, state.get_position());
                    }
                    true
                }
                '>' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimTokenType::GreaterEqual, start_pos, state.get_position());
                    }
                    else if state.peek() == Some('>') {
                        state.advance(1);
                        state.add_token(NimTokenType::RightShift, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimTokenType::Greater, start_pos, state.get_position());
                    }
                    true
                }
                '(' => {
                    state.advance(1);
                    state.add_token(NimTokenType::LeftParen, start_pos, state.get_position());
                    true
                }
                ')' => {
                    state.advance(1);
                    state.add_token(NimTokenType::RightParen, start_pos, state.get_position());
                    true
                }
                '[' => {
                    state.advance(1);
                    state.add_token(NimTokenType::LeftBracket, start_pos, state.get_position());
                    true
                }
                ']' => {
                    state.advance(1);
                    state.add_token(NimTokenType::RightBracket, start_pos, state.get_position());
                    true
                }
                '{' => {
                    state.advance(1);
                    state.add_token(NimTokenType::LeftBrace, start_pos, state.get_position());
                    true
                }
                '}' => {
                    state.advance(1);
                    state.add_token(NimTokenType::RightBrace, start_pos, state.get_position());
                    true
                }
                ',' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Comma, start_pos, state.get_position());
                    true
                }
                ';' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Semicolon, start_pos, state.get_position());
                    true
                }
                ':' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Colon, start_pos, state.get_position());
                    true
                }
                '.' => {
                    state.advance(1);
                    state.add_token(NimTokenType::Dot, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    pub fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            if self.skip_whitespace(state) || self.lex_newline(state) || self.lex_comment(state) || self.lex_string(state) || self.lex_char(state) || self.lex_number(state) || self.lex_identifier(state) || self.lex_operator(state) {
                continue;
            }

            // 如果没有匹配到任何模式，添加错误 kind
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(NimTokenType::Error, start_pos, state.get_position());
            }
        }
        Ok(())
    }
}

impl<'config> Lexer<NimLanguage> for NimLexer<'config> {
    fn lex<'s, S: Source + ?Sized>(&self, source: &'s S, _edits: &[oak_core::source::TextEdit], cache: &'s mut impl LexerCache<NimLanguage>) -> LexOutput<NimLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}
