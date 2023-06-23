#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::FSharpLanguage, lexer::token_type::FSharpTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, Range, Source, TextEdit,
    lexer::{LexOutput, WhitespaceConfig},
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, FSharpLanguage>;

static FS_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

/// F# 词法分析器
#[derive(Clone)]
pub struct FSharpLexer<'config> {
    _config: &'config FSharpLanguage,
}

impl<'config> Lexer<FSharpLanguage> for FSharpLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<FSharpLanguage>) -> LexOutput<FSharpLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> FSharpLexer<'config> {
    pub fn new(config: &'config FSharpLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            // 跳过空白字符
            if self.skip_whitespace(state) {
                continue;
            }

            // 处理注释
            if self.skip_comment(state) {
                continue;
            }

            // 处理字符串字面量
            if self.lex_string_literal(state) {
                continue;
            }

            // 处理字符字面量
            if self.lex_char_literal(state) {
                continue;
            }

            // 处理数字字面量
            if self.lex_number(state) {
                continue;
            }

            // 处理标识符和关键字
            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            // 处理操作符和标点符号
            if self.lex_operator_or_punctuation(state) {
                continue;
            }

            // 如果没有匹配任何模式，跳过当前字符
            let start = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(FSharpTokenType::Error, start, state.get_position())
            }
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch == '\n' || ch == '\r' {
                state.advance(ch.len_utf8());
                state.add_token(FSharpTokenType::Newline, start, state.get_position());
                return true;
            }
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
                while let Some(next) = state.peek() {
                    if next == '\n' || next == '\r' || !next.is_whitespace() {
                        break;
                    }
                    state.advance(next.len_utf8());
                }
                state.add_token(FSharpTokenType::Whitespace, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 行注释: // ... 直到换行
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(FSharpTokenType::LineComment, start, state.get_position());
            return true;
        }

        // 块注释: (* ... *) 支持嵌套
        if rest.starts_with("(*") {
            state.advance(2);
            let mut depth = 1usize;
            while let Some(ch) = state.peek() {
                if ch == '(' && state.peek_next_n(1) == Some('*') {
                    state.advance(2);
                    depth += 1;
                    continue;
                }
                if ch == '*' && state.peek_next_n(1) == Some(')') {
                    state.advance(2);
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    continue;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(FSharpTokenType::BlockComment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 原始字符串: @"..."
        if state.peek() == Some('@') && state.peek_next_n(1) == Some('"') {
            state.advance(2); // 跳过 @"
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(FSharpTokenType::StringLiteral, start, state.get_position());
            return true;
        }

        // 普通字符串: "..."
        if state.peek() == Some('"') {
            state.advance(1); // 跳过 "
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(FSharpTokenType::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_char_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.peek() == Some('\'') {
            state.advance(1); // 跳过 '
            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            if state.peek() == Some('\'') {
                state.advance(1); // 跳过结束的 '
            }
            state.add_token(FSharpTokenType::CharLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if !state.current().map_or(false, |c| c.is_ascii_digit()) {
            return false;
        }

        let start = state.get_position();

        // 处理整数部分
        while state.current().map_or(false, |c| c.is_ascii_digit()) {
            state.advance(1);
        }

        // 处理小数点
        if state.current() == Some('.') && state.peek().map_or(false, |c| c.is_ascii_digit()) {
            state.advance(1); // 跳过 '.'
            while state.current().map_or(false, |c| c.is_ascii_digit()) {
                state.advance(1);
            }
            state.add_token(FSharpTokenType::FloatLiteral, start, state.get_position());
        }
        else {
            // 处理科学计数法
            if matches!(state.current(), Some('e') | Some('E')) {
                state.advance(1);
                if matches!(state.current(), Some('+') | Some('-')) {
                    state.advance(1);
                }
                while state.current().map_or(false, |c| c.is_ascii_digit()) {
                    state.advance(1);
                }
                state.add_token(FSharpTokenType::FloatLiteral, start, state.get_position());
            }
            else {
                // 处理数字后缀
                if state.current().map_or(false, |c| c.is_ascii_alphabetic()) {
                    while state.current().map_or(false, |c| c.is_ascii_alphanumeric()) {
                        state.advance(1);
                    }
                }
                state.add_token(FSharpTokenType::IntegerLiteral, start, state.get_position());
            }
        }

        true
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if !state.current().map_or(false, |c| c.is_ascii_alphabetic() || c == '_') {
            return false;
        }

        let start = state.get_position();
        while state.current().map_or(false, |c| c.is_ascii_alphanumeric() || c == '_') {
            state.advance(1);
        }

        let text = state.get_text_in((start..state.get_position()).into());
        let kind = match text.as_ref() {
            "abstract" => FSharpTokenType::Abstract,
            "and" => FSharpTokenType::And,
            "as" => FSharpTokenType::As,
            "assert" => FSharpTokenType::Assert,
            "base" => FSharpTokenType::Base,
            "begin" => FSharpTokenType::Begin,
            "class" => FSharpTokenType::Class,
            "default" => FSharpTokenType::Default,
            "delegate" => FSharpTokenType::Delegate,
            "do" => FSharpTokenType::Do,
            "done" => FSharpTokenType::Done,
            "downcast" => FSharpTokenType::Downcast,
            "downto" => FSharpTokenType::Downto,
            "elif" => FSharpTokenType::Elif,
            "else" => FSharpTokenType::Else,
            "end" => FSharpTokenType::End,
            "exception" => FSharpTokenType::Exception,
            "extern" => FSharpTokenType::Extern,
            "false" => FSharpTokenType::False,
            "finally" => FSharpTokenType::Finally,
            "for" => FSharpTokenType::For,
            "fun" => FSharpTokenType::Fun,
            "function" => FSharpTokenType::Function,
            "if" => FSharpTokenType::If,
            "in" => FSharpTokenType::In,
            "inherit" => FSharpTokenType::Inherit,
            "inline" => FSharpTokenType::Inline,
            "interface" => FSharpTokenType::Interface,
            "internal" => FSharpTokenType::Internal,
            "lazy" => FSharpTokenType::Lazy,
            "let" => FSharpTokenType::Let,
            "match" => FSharpTokenType::Match,
            "member" => FSharpTokenType::Member,
            "module" => FSharpTokenType::Module,
            "mutable" => FSharpTokenType::Mutable,
            "namespace" => FSharpTokenType::Namespace,
            "new" => FSharpTokenType::New,
            "not" => FSharpTokenType::Not,
            "null" => FSharpTokenType::Null,
            "of" => FSharpTokenType::Of,
            "open" => FSharpTokenType::Open,
            "or" => FSharpTokenType::Or,
            "override" => FSharpTokenType::Override,
            "private" => FSharpTokenType::Private,
            "public" => FSharpTokenType::Public,
            "rec" => FSharpTokenType::Rec,
            "return" => FSharpTokenType::Return,
            "select" => FSharpTokenType::Select,
            "static" => FSharpTokenType::Static,
            "struct" => FSharpTokenType::Struct,
            "then" => FSharpTokenType::Then,
            "to" => FSharpTokenType::To,
            "true" => FSharpTokenType::True,
            "try" => FSharpTokenType::Try,
            "type" => FSharpTokenType::Type,
            "upcast" => FSharpTokenType::Upcast,
            "use" => FSharpTokenType::Use,
            "val" => FSharpTokenType::Val,
            "void" => FSharpTokenType::Void,
            "when" => FSharpTokenType::When,
            "while" => FSharpTokenType::While,
            "with" => FSharpTokenType::With,
            "yield" => FSharpTokenType::Yield,
            _ => FSharpTokenType::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_operator_or_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let current = state.current();
        if current.is_none() {
            return false;
        }

        let start = state.get_position();
        let c = current.unwrap();
        let next = state.peek();

        // 双字符操作符
        match (c, next) {
            ('-', Some('>')) => {
                state.advance(2);
                state.add_token(FSharpTokenType::Arrow, start, state.get_position());
                return true;
            }
            (':', Some(':')) => {
                state.advance(2);
                state.add_token(FSharpTokenType::Cons, start, state.get_position());
                return true;
            }
            ('=', Some('=')) => {
                state.advance(2);
                state.add_token(FSharpTokenType::Equal, start, state.get_position());
                return true;
            }
            ('<', Some('=')) => {
                state.advance(2);
                state.add_token(FSharpTokenType::LessEqual, start, state.get_position());
                return true;
            }
            ('>', Some('=')) => {
                state.advance(2);
                state.add_token(FSharpTokenType::GreaterEqual, start, state.get_position());
                return true;
            }
            ('<', Some('>')) => {
                state.advance(2);
                state.add_token(FSharpTokenType::NotEqual, start, state.get_position());
                return true;
            }
            ('|', Some('>')) => {
                state.advance(2);
                state.add_token(FSharpTokenType::Pipe, start, state.get_position());
                return true;
            }
            _ => {}
        }

        // 单字符操作符和标点符号
        let kind = match c {
            '+' => FSharpTokenType::Plus,
            '-' => FSharpTokenType::Minus,
            '*' => FSharpTokenType::Star,
            '/' => FSharpTokenType::Slash,
            '%' => FSharpTokenType::Percent,
            '=' => FSharpTokenType::Equal,
            '<' => FSharpTokenType::LessThan,
            '>' => FSharpTokenType::GreaterThan,
            '&' => FSharpTokenType::Ampersand,
            '|' => FSharpTokenType::Pipe,
            '^' => FSharpTokenType::Caret,
            '!' => FSharpTokenType::Not,
            '?' => FSharpTokenType::Question,
            ':' => FSharpTokenType::Colon,
            ';' => FSharpTokenType::Semicolon,
            ',' => FSharpTokenType::Comma,
            '.' => FSharpTokenType::Dot,
            '(' => FSharpTokenType::LeftParen,
            ')' => FSharpTokenType::RightParen,
            '[' => FSharpTokenType::LeftBracket,
            ']' => FSharpTokenType::RightBracket,
            '{' => FSharpTokenType::LeftBrace,
            '}' => FSharpTokenType::RightBrace,
            _ => return false,
        };

        state.advance(1);
        state.add_token(kind, start, state.get_position());
        true
    }
}
