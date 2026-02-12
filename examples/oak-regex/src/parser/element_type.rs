use crate::lexer::token_type::RegexTokenType;
use oak_core::{ElementType, UniversalElementRole};

/// Regex element types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RegexElementType {
    /// Root element.
    Root,
    /// Token wrapper.
    Token(RegexTokenType),
    /// Error element.
    Error,
}

impl ElementType for RegexElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Token(_) => UniversalElementRole::None,
            Self::Error => UniversalElementRole::Error,
        }
    }
}

impl From<RegexTokenType> for RegexElementType {
    fn from(token: RegexTokenType) -> Self {
        Self::Token(token)
    }
}
