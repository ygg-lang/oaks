use oak_core::{Token, TokenType, UniversalTokenRole};

/// Alias for `Token<DejavuTokenType>`.
pub type DejavuToken = Token<DejavuTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DejavuTokenType {
    Eof,
    Whitespace,
    Error,
}

impl TokenType for DejavuTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
        }
    }
}
