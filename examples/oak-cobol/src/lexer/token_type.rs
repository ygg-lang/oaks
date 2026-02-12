use oak_core::{TokenType, UniversalTokenRole};

/// COBOL token types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CobolTokenType {
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
    /// "IDENTIFICATION" keyword.
    Identification,
    /// "DIVISION" keyword.
    Division,
}

impl TokenType for CobolTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Eof => UniversalTokenRole::Eof,
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::String | Self::Number => UniversalTokenRole::Literal,
            Self::Identification | Self::Division => UniversalTokenRole::Keyword,
        }
    }
}
