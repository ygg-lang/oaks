use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmalltalkSyntaxKind {
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

impl TokenType for SmalltalkSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for SmalltalkSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
