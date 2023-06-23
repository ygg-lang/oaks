use crate::lexer::token_type::RakuTokenType;
use oak_core::{ElementType, UniversalElementRole};

/// Raku element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RakuElementType {
    /// A token element.
    Token(RakuTokenType),
    /// The root element.
    Root,
    /// An expression element.
    Expression,
    /// A statement element.
    Statement,
}

impl From<RakuTokenType> for RakuElementType {
    fn from(token: RakuTokenType) -> Self {
        Self::Token(token)
    }
}

impl ElementType for RakuElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Token(_) => UniversalElementRole::None,
            Self::Root => UniversalElementRole::Root,
            Self::Expression => UniversalElementRole::Expression,
            Self::Statement => UniversalElementRole::Statement,
        }
    }
}

impl Default for RakuElementType {
    fn default() -> Self {
        Self::Root
    }
}
