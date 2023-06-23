use oak_core::{TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TomlTokenKind {
    Whitespace,
    Newline,
    Comment,

    // Literals
    String,
    BasicString,
    LiteralString,
    Integer,
    Float,
    Boolean,
    OffsetDateTime,
    LocalDateTime,
    LocalDate,
    LocalTime,

    // Keywords/Identifiers
    Identifier,
    BareKey,

    // Symbols
    Equal,        // =
    Dot,          // .
    Comma,        // ,
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    DoubleLeftBracket,
    DoubleRightBracket,

    // Internal
    Error,
    Eof,
    Root,
}

pub type TomlTokenType = TomlTokenKind;

impl TokenType for TomlTokenKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::BasicString | Self::LiteralString | Self::String | Self::Integer | Self::Float | Self::Boolean | Self::OffsetDateTime | Self::LocalDateTime | Self::LocalDate | Self::LocalTime => UniversalTokenRole::Literal,
            Self::BareKey | Self::Identifier => UniversalTokenRole::Name,
            Self::Equal | Self::Dot | Self::Comma | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::DoubleLeftBracket | Self::DoubleRightBracket => UniversalTokenRole::Punctuation,
            Self::Root => UniversalTokenRole::None,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::None,
        }
    }
}
