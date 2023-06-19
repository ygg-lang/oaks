use oak_core::SyntaxKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmalltalkKind {
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

    // Special
    Eof,
    Error,
}

impl SyntaxKind for SmalltalkKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error | Self::Eof)
    }

    fn is_element_type(&self) -> bool {
        false
    }
}
