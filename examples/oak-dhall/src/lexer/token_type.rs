use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type DHallToken = Token<DHallTokenType>;

impl DHallTokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::If
                | Self::Then
                | Self::Else
                | Self::Let
                | Self::In
                | Self::Using
                | Self::As
                | Self::Merge
                | Self::Some
                | Self::None
                | Self::NaN
                | Self::Infinity
                | Self::Type
                | Self::Kind
                | Self::Sort
                | Self::Bool
                | Self::Natural
                | Self::Integer
                | Self::Double
                | Self::Text
                | Self::List
                | Self::Optional
                | Self::True
                | Self::False
                | Self::With
                | Self::Forall
                | Self::Assert
        )
    }
}

impl TokenType for DHallTokenType {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum DHallTokenType {
    Whitespace,

    Newline,

    Comment,

    Identifier,

    Number,

    String,

    // Keywords
    If,

    Then,

    Else,

    Let,

    In,

    Using,

    As,

    Merge,

    Some,

    None,

    NaN,

    Infinity,

    Type,

    Kind,

    Sort,

    Bool,

    Natural,

    Integer,

    Double,

    Text,

    List,

    Optional,

    True,

    False,

    With,

    Forall,

    Assert,

    // Operators
    Arrow,

    FatArrow,

    EqualEqual,

    NotEqual,

    And,

    Or,

    Append,

    Combine,

    CombineTypes,

    Prefer,

    Lambda,

    // Punctuation
    LeftParen,

    RightParen,

    LeftBrace,

    RightBrace,

    LeftBracket,

    RightBracket,

    Comma,

    Semicolon,

    Dot,

    Colon,

    Equal,

    Less,

    Greater,

    Plus,

    Minus,

    Star,

    Slash,

    Pipe,

    At,

    Hash,

    Question,

    Error,

    Eof,

    // Special
    Root,
    SourceFile,
}
