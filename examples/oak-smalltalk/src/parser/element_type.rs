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
                match token {
            crate::lexer::token_type::SmalltalkTokenType::Root => Self::Root,
            crate::lexer::token_type::SmalltalkTokenType::SourceFile => Self::SourceFile,
            crate::lexer::token_type::SmalltalkTokenType::Eof => Self::Eof,
            crate::lexer::token_type::SmalltalkTokenType::Error => Self::Error,
            crate::lexer::token_type::SmalltalkTokenType::Number => Self::Number,
            crate::lexer::token_type::SmalltalkTokenType::Integer => Self::Integer,
            crate::lexer::token_type::SmalltalkTokenType::Float => Self::Float,
            crate::lexer::token_type::SmalltalkTokenType::String => Self::String,
            crate::lexer::token_type::SmalltalkTokenType::Character => Self::Character,
            crate::lexer::token_type::SmalltalkTokenType::Symbol => Self::Symbol,
            crate::lexer::token_type::SmalltalkTokenType::True => Self::True,
            crate::lexer::token_type::SmalltalkTokenType::False => Self::False,
            crate::lexer::token_type::SmalltalkTokenType::Nil => Self::Nil,
            crate::lexer::token_type::SmalltalkTokenType::Self_ => Self::Self_,
            crate::lexer::token_type::SmalltalkTokenType::Super => Self::Super,
            crate::lexer::token_type::SmalltalkTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::SmalltalkTokenType::Plus => Self::Plus,
            crate::lexer::token_type::SmalltalkTokenType::Minus => Self::Minus,
            crate::lexer::token_type::SmalltalkTokenType::Star => Self::Star,
            crate::lexer::token_type::SmalltalkTokenType::Slash => Self::Slash,
            crate::lexer::token_type::SmalltalkTokenType::Percent => Self::Percent,
            crate::lexer::token_type::SmalltalkTokenType::Equal => Self::Equal,
            crate::lexer::token_type::SmalltalkTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::SmalltalkTokenType::Less => Self::Less,
            crate::lexer::token_type::SmalltalkTokenType::Greater => Self::Greater,
            crate::lexer::token_type::SmalltalkTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::SmalltalkTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::SmalltalkTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::SmalltalkTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::SmalltalkTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::SmalltalkTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::SmalltalkTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::SmalltalkTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::SmalltalkTokenType::Dot => Self::Dot,
            crate::lexer::token_type::SmalltalkTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::SmalltalkTokenType::Comma => Self::Comma,
            crate::lexer::token_type::SmalltalkTokenType::Colon => Self::Colon,
            crate::lexer::token_type::SmalltalkTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::SmalltalkTokenType::Caret => Self::Caret,
            crate::lexer::token_type::SmalltalkTokenType::Comment => Self::Comment,
            crate::lexer::token_type::SmalltalkTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::SmalltalkTokenType::Newline => Self::Newline,
            _ => Self::Error,
        }
    }
}
