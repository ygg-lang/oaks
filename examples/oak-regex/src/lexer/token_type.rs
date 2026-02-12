use oak_core::{TokenType, UniversalTokenRole};

/// Regex token types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RegexTokenType {
    /// End of file.
    Eof,
    /// Whitespace.
    Whitespace,
    /// Identifier.
    Identifier,
    /// Operator.
    Operator,
    /// Punctuation.
    Punctuation,
    /// Unknown.
    Unknown,
}

impl TokenType for RegexTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Eof => UniversalTokenRole::Eof,
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Operator => UniversalTokenRole::Operator,
            Self::Punctuation => UniversalTokenRole::Punctuation,
            Self::Unknown => UniversalTokenRole::Error,
        }
    }
}
