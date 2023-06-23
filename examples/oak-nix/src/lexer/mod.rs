#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::NixLanguage, lexer::token_type::NixTokenType};
use oak_core::{
    Source,
    lexer::{LexOutput, Lexer, LexerCache, LexerState, Token},
    source::TextEdit,
};

type State<'a, S> = LexerState<'a, S, NixLanguage>;

#[derive(Clone, Debug)]
pub struct NixLexer<'config> {
    _config: &'config NixLanguage,
}

impl<'config> NixLexer<'config> {
    pub fn new(config: &'config NixLanguage) -> Self {
        Self { _config: config }
    }
}

impl NixLexer<'_> {
    pub fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
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
            if self.lex_identifier(state) {
                continue;
            }
            if self.lex_operator(state) {
                continue;
            }

            // 如果没有匹配到任何模式，添加错误 kind
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(NixTokenType::Error, start_pos, state.get_position());
            }
        }
        Ok(())
    }
}

impl<'config> Lexer<NixLanguage> for NixLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<NixLanguage>) -> LexOutput<NixLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl NixLexer<'_> {
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
            state.add_token(NixTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(NixTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(NixTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // 读取到行
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(NixTokenType::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(NixTokenType::String, start_pos, state.get_position());
            true
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
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
                state.add_token(NixTokenType::Number, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理标识符和关键字
    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' || ch == '\'' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let kind = match &*text {
                    "let" => NixTokenType::Let,
                    "in" => NixTokenType::In,
                    "if" => NixTokenType::If,
                    "then" => NixTokenType::Then,
                    "else" => NixTokenType::Else,
                    "with" => NixTokenType::With,
                    "inherit" => NixTokenType::Inherit,
                    "rec" => NixTokenType::Rec,
                    "import" => NixTokenType::Import,
                    "true" | "false" => NixTokenType::Boolean,
                    "null" => NixTokenType::Null,
                    _ => NixTokenType::Identifier,
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

    /// 处理操作
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('+') = state.peek() {
                        state.advance(1);
                        NixTokenType::Concatenation
                    }
                    else {
                        NixTokenType::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        NixTokenType::Implication
                    }
                    else {
                        NixTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    NixTokenType::Star
                }
                '/' => {
                    state.advance(1);
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        NixTokenType::Update
                    }
                    else {
                        NixTokenType::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    NixTokenType::Percent
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        NixTokenType::Equal
                    }
                    else {
                        NixTokenType::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        NixTokenType::NotEqual
                    }
                    else {
                        return false;
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        NixTokenType::LessEqual
                    }
                    else {
                        NixTokenType::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        NixTokenType::GreaterEqual
                    }
                    else {
                        NixTokenType::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        NixTokenType::LogicalAnd
                    }
                    else {
                        return false;
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        NixTokenType::LogicalOr
                    }
                    else {
                        return false;
                    }
                }
                '?' => {
                    state.advance(1);
                    NixTokenType::Question
                }
                '(' => {
                    state.advance(1);
                    NixTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    NixTokenType::RightParen
                }
                '{' => {
                    state.advance(1);
                    NixTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    NixTokenType::RightBrace
                }
                '[' => {
                    state.advance(1);
                    NixTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    NixTokenType::RightBracket
                }
                ';' => {
                    state.advance(1);
                    NixTokenType::Semicolon
                }
                ':' => {
                    state.advance(1);
                    NixTokenType::Colon
                }
                ',' => {
                    state.advance(1);
                    NixTokenType::Comma
                }
                '.' => {
                    state.advance(1);
                    NixTokenType::Dot
                }
                '@' => {
                    state.advance(1);
                    NixTokenType::At
                }
                '$' => {
                    state.advance(1);
                    NixTokenType::Dollar
                }
                '#' => {
                    state.advance(1);
                    NixTokenType::Hash
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}
