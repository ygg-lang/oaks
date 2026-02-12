use oak_core::{TokenType, UniversalTokenRole};

/// Solidity token types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SolidityTokenType {
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
    /// "pragma" keyword.
    Pragma,
    /// "contract" keyword.
    Contract,
}

impl TokenType for SolidityTokenType {
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
            Self::Pragma | Self::Contract => UniversalTokenRole::Keyword,
        }
    }
}
