use oak_core::{TokenType, UniversalTokenRole};

/// Smalltalk token types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SmalltalkTokenType {
    /// End of file.
    Eof,
    /// Whitespace.
    Whitespace,
    /// Comment.
    Comment,
    /// Identifier.
    Identifier,
    /// String literal.
    String,
    /// Number literal.
    Number,
    /// Character literal.
    Character,
    /// Symbol literal.
    Symbol,
}

impl TokenType for SmalltalkTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Eof => UniversalTokenRole::Eof,
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::String => UniversalTokenRole::Literal,
            Self::Number => UniversalTokenRole::Literal,
            Self::Character => UniversalTokenRole::Literal,
            Self::Symbol => UniversalTokenRole::Literal,
        }
    }
}
