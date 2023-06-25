use crate::lexer::token_type::AplTokenType;
use oak_core::{ElementType, GreenNode, UniversalElementRole};
use std::sync::Arc;

/// Type alias for APL syntax tree elements.
pub type AplElement<'a> = Arc<GreenNode<'a, AplElementType>>;

/// Element types for the APL language.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AplElementType {
    /// Root node.
    Root,
    /// A statement.
    Statement,
    /// An expression.
    Expression,
    /// An assignment (←).
    Assignment,
    /// A vector or array literal.
    ArrayLiteral,
    /// A function (primitive or dfn).
    Function,
    /// An operator (primitive or dop).
    Operator,
    /// An identifier (variable name).
    Identifier,
    /// A number literal.
    NumberLiteral,
    /// A string literal.
    StringLiteral,
    /// An error node.
    Error,
}

impl ElementType for AplElementType {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        use UniversalElementRole::*;
        match self {
            Self::Root => Root,
            Self::Statement => Statement,
            Self::Expression | Self::ArrayLiteral | Self::Assignment => Expression,
            Self::Function | Self::Operator => Definition,
            Self::Identifier => Reference,
            Self::NumberLiteral | Self::StringLiteral => Expression,
            Self::Error => Error,
        }
    }
}

impl From<AplTokenType> for AplElementType {
    fn from(token_type: AplTokenType) -> Self {
        match token_type {
            AplTokenType::Identifier => Self::Identifier,
            AplTokenType::StringLiteral => Self::StringLiteral,
            AplTokenType::NumberLiteral => Self::NumberLiteral,
            _ => Self::Error,
        }
    }
}
