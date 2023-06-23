use oak_core::{Source, Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type ElmToken = Token<ElmTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ElmTokenType {
    Root,
    Whitespace,
    Newline,
    Comment,
    Identifier,
    Number,
    Float,
    String,
    Char,

    // Keywords
    If,
    Then,
    Else,
    Case,
    Of,
    Let,
    In,
    Type,
    Alias,
    Module,
    Where,
    Import,
    Exposing,
    As,
    Port,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    DoubleSlash,
    Caret,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    DoubleAmpersand,
    DoublePipe,
    DoublePlus,
    DoubleLess,
    DoubleGreater,
    Arrow,
    Pipe,
    PipeGreater,
    Dot,
    DoubleDot,
    TripleDot,
    Comma,
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Backslash,
    Bar,

    Error,
    Eof,
}

impl ElmTokenType {
    pub fn is_keyword(self) -> bool {
        matches!(self, Self::If | Self::Then | Self::Else | Self::Case | Self::Of | Self::Let | Self::In | Self::Type | Self::Alias | Self::Module | Self::Where | Self::Import | Self::Exposing | Self::As | Self::Port)
    }
}

impl TokenType for ElmTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}
