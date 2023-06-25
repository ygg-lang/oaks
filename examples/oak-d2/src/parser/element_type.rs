use crate::lexer::token_type::D2TokenType;
use oak_core::{ElementType, UniversalElementRole};

/// Element types for the D2 AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum D2ElementType {
    /// Token.
    Token(D2TokenType),
    /// Root node.
    Root,
    /// Shape definition.
    Shape,
    /// Connection definition.
    Connection,
    /// Container definition.
    Container,
    /// Parsing error.
    Error,
}

impl From<D2TokenType> for D2ElementType {
    fn from(token: D2TokenType) -> Self {
        Self::Token(token)
    }
}

impl ElementType for D2ElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}
