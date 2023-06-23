#![doc = include_str!("readme.md")]
pub mod token_type;

pub use token_type::CTokenType;

use crate::language::CLanguage;
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};
#[cfg(feature = "serde")]
use serde::Serialize;
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, CLanguage>;

/// Lexer for the C language.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Copy, Debug)]
pub struct CLexer<'config> {
    /// Language configuration.
    config: &'config CLanguage,
}

impl<'config> Lexer<CLanguage> for CLexer<'config> {
    /// Tokenizes the source code into a stream of C tokens.
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<CLanguage>) -> LexOutput<CLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> CLexer<'config> {
    /// Creates a new `CLexer` with the given language configuration.
    pub fn new(config: &'config CLanguage) -> Self {
        Self { config }
    }

    /// Runs the lexer on the current state until the end of the source.
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();
            if self.skip_whitespace(state) {
                continue;
            }
            if self.skip_comment(state) {
                continue;
            }
            if self.lex_newline(state) {
                continue;
            }
            if self.lex_string(state) {
                continue;
            }
            if self.lex_char(state) {
                continue;
            }
            if self.lex_number(state) {
                continue;
            }
            if self.lex_keyword_or_identifier(state) {
                continue;
            }
            if self.lex_operator_or_delimiter(state) {
                continue;
            }
            if self.lex_preprocessor(state) {
                continue;
            }
            if self.lex_text(state) {
                continue;
            }
            else {
                let start = state.get_position();
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                    state.add_token(CTokenType::Error, start, state.get_position())
                }
            }
            state.advance_if_dead_lock(safe_point)
        }
        Ok(())
    }

    /// Skips whitespace characters (except newlines).
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let mut count = 0;

        while let Some(ch) = state.peek() {
            if ch.is_whitespace() && ch != '\n' && ch != '\r' {
                state.advance(ch.len_utf8());
                count += 1
            }
            else {
                break;
            }
        }

        if count > 0 {
            state.add_token(CTokenType::Whitespace, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.consume_if_starts_with("//") {
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }
            state.add_token(CTokenType::Comment, start, state.get_position());
            return true;
        }
        else if state.consume_if_starts_with("/*") {
            while state.not_at_end() {
                if state.consume_if_starts_with("*/") {
                    break;
                }
                if let Some(ch) = state.peek() { state.advance(ch.len_utf8()) } else { break }
            }
            state.add_token(CTokenType::Comment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '\n' {
                state.advance(1);
                state.add_token(CTokenType::Whitespace, start, state.get_position());
                return true;
            }
            else if ch == '\r' {
                state.advance(1);
                if state.peek() == Some('\n') {
                    state.advance(1)
                }
                state.add_token(CTokenType::Whitespace, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8())
                    }
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(CTokenType::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_char<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8())
                    }
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(CTokenType::CharLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '.' || ch == 'e' || ch == 'E' || ch == '+' || ch == '-' { state.advance(ch.len_utf8()) } else { break }
                }

                let text = state.get_text_in((start..state.get_position()).into());
                let kind = if text.contains('.') || text.contains('e') || text.contains('E') { CTokenType::FloatLiteral } else { CTokenType::IntegerLiteral };
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_keyword_or_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                }

                let text = state.get_text_in((start..state.get_position()).into());
                let kind = if C_KEYWORDS.contains(&&*text) {
                    match &*text {
                        "auto" => CTokenType::Auto,
                        "register" => CTokenType::Register,
                        "static" => CTokenType::Static,
                        "extern" => CTokenType::Extern,
                        "typedef" => CTokenType::Typedef,
                        "void" => CTokenType::Void,
                        "char" => CTokenType::Char,
                        "short" => CTokenType::Short,
                        "int" => CTokenType::Int,
                        "long" => CTokenType::Long,
                        "float" => CTokenType::Float,
                        "double" => CTokenType::Double,
                        "signed" => CTokenType::Signed,
                        "unsigned" => CTokenType::Unsigned,
                        "struct" => CTokenType::Struct,
                        "union" => CTokenType::Union,
                        "enum" => CTokenType::Enum,
                        "const" => CTokenType::Const,
                        "volatile" => CTokenType::Volatile,
                        "restrict" => CTokenType::Restrict,
                        "if" => CTokenType::If,
                        "else" => CTokenType::Else,
                        "switch" => CTokenType::Switch,
                        "case" => CTokenType::Case,
                        "default" => CTokenType::Default,
                        "for" => CTokenType::For,
                        "while" => CTokenType::While,
                        "do" => CTokenType::Do,
                        "break" => CTokenType::Break,
                        "continue" => CTokenType::Continue,
                        "goto" => CTokenType::Goto,
                        "return" => CTokenType::Return,
                        "sizeof" => CTokenType::Sizeof,
                        "inline" => CTokenType::Inline,
                        "_Bool" => CTokenType::Bool,
                        "_Complex" => CTokenType::Complex,
                        "_Imaginary" => CTokenType::Imaginary,
                        "_Alignas" => CTokenType::Alignas,
                        "_Alignof" => CTokenType::Alignof,
                        "_Atomic" => CTokenType::Atomic,
                        "_Static_assert" => CTokenType::StaticAssert,
                        "_Thread_local" => CTokenType::ThreadLocal,
                        "_Generic" => CTokenType::Generic,
                        "_Noreturn" => CTokenType::Noreturn,
                        _ => CTokenType::Identifier,
                    }
                }
                else {
                    CTokenType::Identifier
                };
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_operator_or_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            let three_char = if let Some(next_ch) = state.peek_next_n(1) { if let Some(third_ch) = state.peek_next_n(2) { Some(format!("{}{}{}", ch, next_ch, third_ch)) } else { None } } else { None };

            let two_char = if let Some(next_ch) = state.peek_next_n(1) { format!("{}{}", ch, next_ch) } else { String::new() };

            // 检查三字符操作符
            if let Some(ref three) = three_char {
                if let Some(&kind) = C_THREE_CHAR_OPERATORS.get(three.as_str()) {
                    state.advance(3);
                    state.add_token(kind, start, state.get_position());
                    return true;
                }
            }

            // 检查双字符操作符
            if let Some(&kind) = C_TWO_CHAR_OPERATORS.get(two_char.as_str()) {
                state.advance(2);
                state.add_token(kind, start, state.get_position());
                return true;
            }

            // 检查单字符操作符 and 分隔符
            let kind = match ch {
                '(' => CTokenType::LeftParen,
                ')' => CTokenType::RightParen,
                '[' => CTokenType::LeftBracket,
                ']' => CTokenType::RightBracket,
                '{' => CTokenType::LeftBrace,
                '}' => CTokenType::RightBrace,
                ',' => CTokenType::Comma,
                ';' => CTokenType::Semicolon,
                ':' => CTokenType::Colon,
                '.' => CTokenType::Dot,
                '?' => CTokenType::Question,
                '+' => CTokenType::Plus,
                '-' => CTokenType::Minus,
                '*' => CTokenType::Star,
                '/' => CTokenType::Slash,
                '%' => CTokenType::Percent,
                '=' => CTokenType::Assign,
                '<' => CTokenType::Less,
                '>' => CTokenType::Greater,
                '!' => CTokenType::LogicalNot,
                '&' => CTokenType::BitAnd,
                '|' => CTokenType::BitOr,
                '^' => CTokenType::BitXor,
                '~' => CTokenType::BitNot,
                _ => return false,
            };
            state.advance(1);
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_preprocessor<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.consume_if_starts_with("#") {
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }
            state.add_token(CTokenType::PreprocessorDirective, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_whitespace() && !ch.is_ascii_alphanumeric() && !"()[]{},.;:?+-*/%=<>!&|^~#\"'_".contains(ch) {
                state.advance(ch.len_utf8());
                state.add_token(CTokenType::Text, start, state.get_position());
                return true;
            }
        }
        false
    }
}

static C_KEYWORDS: LazyLock<&[&str]> = LazyLock::new(|| {
    &[
        "auto",
        "register",
        "static",
        "extern",
        "typedef",
        "void",
        "char",
        "short",
        "int",
        "long",
        "float",
        "double",
        "signed",
        "unsigned",
        "struct",
        "union",
        "enum",
        "const",
        "volatile",
        "restrict",
        "if",
        "else",
        "switch",
        "case",
        "default",
        "for",
        "while",
        "do",
        "break",
        "continue",
        "goto",
        "return",
        "sizeof",
        "inline",
        "_Bool",
        "_Complex",
        "_Imaginary",
        "_Alignas",
        "_Alignof",
        "_Atomic",
        "_Static_assert",
        "_Thread_local",
        "_Generic",
        "_Noreturn",
    ]
});

static C_TWO_CHAR_OPERATORS: LazyLock<std::collections::HashMap<&str, CTokenType>> = LazyLock::new(|| {
    let mut map = std::collections::HashMap::new();
    map.insert("+=", CTokenType::PlusAssign);
    map.insert("-=", CTokenType::MinusAssign);
    map.insert("*=", CTokenType::StarAssign);
    map.insert("/=", CTokenType::SlashAssign);
    map.insert("%=", CTokenType::PercentAssign);
    map.insert("==", CTokenType::Equal);
    map.insert("!=", CTokenType::NotEqual);
    map.insert("<=", CTokenType::LessEqual);
    map.insert(">=", CTokenType::GreaterEqual);
    map.insert("&&", CTokenType::LogicalAnd);
    map.insert("||", CTokenType::LogicalOr);
    map.insert("<<", CTokenType::LeftShift);
    map.insert(">>", CTokenType::RightShift);
    map.insert("&=", CTokenType::AndAssign);
    map.insert("|=", CTokenType::OrAssign);
    map.insert("^=", CTokenType::XorAssign);
    map.insert("++", CTokenType::Increment);
    map.insert("--", CTokenType::Decrement);
    map.insert("->", CTokenType::Arrow);
    map
});

static C_THREE_CHAR_OPERATORS: LazyLock<std::collections::HashMap<&str, CTokenType>> = LazyLock::new(|| {
    let mut map = std::collections::HashMap::new();
    map.insert("<<=", CTokenType::LeftShiftAssign);
    map.insert(">>=", CTokenType::RightShiftAssign);
    map
});
