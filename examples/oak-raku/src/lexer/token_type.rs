use oak_core::{TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Raku token types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RakuTokenType {
    /// End of file.
    EndOfFile,
    /// Unknown token.
    Unknown,
    /// Whitespace.
    Whitespace,
    /// Comment.
    Comment,
    /// Identifier.
    Identifier,
    /// Number.
    Number,
    /// String.
    String,
}

impl TokenType for RakuTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::EndOfFile;

    fn role(&self) -> Self::Role {
        match self {
            Self::EndOfFile => UniversalTokenRole::Eof,
            Self::Unknown => UniversalTokenRole::Error,
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number => UniversalTokenRole::Literal,
            Self::String => UniversalTokenRole::Literal,
        }
    }
}

impl Default for RakuTokenType {
    fn default() -> Self {
        Self::Unknown
    }
}
