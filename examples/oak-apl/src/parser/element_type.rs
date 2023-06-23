use crate::lexer::token_type::AplTokenType;
use oak_core::{ElementType, GreenNode, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// APL 语法树元素的类型别名
pub type AplElement<'a> = Arc<GreenNode<'a, AplElementType>>;

/// APL 语法树中所有可能的元素类型。
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AplElementType {
    /// Root node
    Root,
    /// Statement
    Statement,
    /// Expression
    Expression,
    /// Assignment (←)
    Assignment,
    /// Vector/Array literal
    ArrayLiteral,
    /// Function (primitive or dfn)
    Function,
    /// Operator (primitive or dop)
    Operator,
    /// Identifier (variable name)
    Identifier,
    /// Number literal
    NumberLiteral,
    /// String literal
    StringLiteral,
    /// Error node
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
