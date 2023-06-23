use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum SmalltalkElementType {
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

impl TokenType for SmalltalkElementType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        UniversalTokenRole::None
    }
}

impl ElementType for SmalltalkElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::SmalltalkTokenType> for SmalltalkElementType {
    fn from(token: crate::lexer::token_type::SmalltalkTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
