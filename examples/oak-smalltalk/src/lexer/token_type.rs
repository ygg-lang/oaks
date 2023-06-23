use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type SmalltalkToken = Token<SmalltalkTokenType>;

impl TokenType for SmalltalkTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SmalltalkTokenType {
    // Special
    Root,
    SourceFile,
    Eof,
    Error,

    // Literals
    Number,
    Integer,
    Float,
    String,
    Character,
    Symbol,

    // Keywords
    True,
    False,
    Nil,
    Self_,
    Super,

    // Identifiers
    Identifier,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Dot,
    Semicolon,
    Comma,
    Colon,
    Pipe,
    Caret,

    // Comments
    Comment,

    // Whitespace
    Whitespace,
    Newline,
}
