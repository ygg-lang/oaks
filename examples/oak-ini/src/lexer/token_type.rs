use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type IniToken = Token<IniTokenType>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IniTokenType {
    // Basic kinds
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,

    // Tokens
    LeftBrace,          // {
    RightBrace,         // }
    LeftBracket,        // [
    RightBracket,       // ]
    DoubleLeftBracket,  // [[
    DoubleRightBracket, // ]]
    Comma,              // ,
    Dot,                // .
    Equal,              // =

    // Values
    Identifier,
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
}

impl TokenType for IniTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::DoubleLeftBracket | Self::DoubleRightBracket | Self::Comma | Self::Dot => UniversalTokenRole::Punctuation,
            Self::Equal => UniversalTokenRole::Operator,
            Self::Identifier => UniversalTokenRole::Name,
            Self::String | Self::Integer | Self::Float | Self::Boolean | Self::DateTime => UniversalTokenRole::Literal,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::Eof,
        }
    }
}
