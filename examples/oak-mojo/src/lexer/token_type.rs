use oak_core::UniversalTokenRole;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MojoTokenType {
    // Keywords
    Fn,
    Struct,
    Var,
    Let,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Break,
    Continue,
    Import,
    From,
    True,
    False,
    None,

    // Identifiers and Literals
    Identifier,
    Integer,
    Float,
    String,

    // Operators and Delimiters
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,

    // Trivia
    Whitespace,
    Newline,
    Comment,
    Indent,
    Dedent,

    // Special
    EndOfStream,
    Error,
}

impl oak_core::TokenType for MojoTokenType {
    const END_OF_STREAM: Self = MojoTokenType::EndOfStream;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            MojoTokenType::Fn
            | MojoTokenType::Struct
            | MojoTokenType::Var
            | MojoTokenType::Let
            | MojoTokenType::If
            | MojoTokenType::Else
            | MojoTokenType::While
            | MojoTokenType::For
            | MojoTokenType::In
            | MojoTokenType::Return
            | MojoTokenType::Break
            | MojoTokenType::Continue
            | MojoTokenType::Import
            | MojoTokenType::From
            | MojoTokenType::True
            | MojoTokenType::False
            | MojoTokenType::None => UniversalTokenRole::Keyword,

            MojoTokenType::Identifier => UniversalTokenRole::Name,

            MojoTokenType::Integer | MojoTokenType::Float | MojoTokenType::String => UniversalTokenRole::Literal,

            MojoTokenType::Plus
            | MojoTokenType::Minus
            | MojoTokenType::Star
            | MojoTokenType::Slash
            | MojoTokenType::Percent
            | MojoTokenType::Equal
            | MojoTokenType::EqualEqual
            | MojoTokenType::NotEqual
            | MojoTokenType::Less
            | MojoTokenType::LessEqual
            | MojoTokenType::Greater
            | MojoTokenType::GreaterEqual
            | MojoTokenType::And
            | MojoTokenType::Or
            | MojoTokenType::Not => UniversalTokenRole::Operator,

            MojoTokenType::LeftParen
            | MojoTokenType::RightParen
            | MojoTokenType::LeftBracket
            | MojoTokenType::RightBracket
            | MojoTokenType::LeftBrace
            | MojoTokenType::RightBrace
            | MojoTokenType::Comma
            | MojoTokenType::Dot
            | MojoTokenType::Colon
            | MojoTokenType::Semicolon
            | MojoTokenType::Arrow => UniversalTokenRole::Punctuation,

            MojoTokenType::Whitespace | MojoTokenType::Newline | MojoTokenType::Comment | MojoTokenType::Indent | MojoTokenType::Dedent => UniversalTokenRole::Whitespace,

            MojoTokenType::EndOfStream => UniversalTokenRole::Eof,
            MojoTokenType::Error => UniversalTokenRole::Error,
        }
    }
}
