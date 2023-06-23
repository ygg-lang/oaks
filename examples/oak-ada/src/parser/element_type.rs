use crate::lexer::AdaTokenType;
use oak_core::{ElementType, GreenNode, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Ada 语法树元素的类型别名
pub type AdaElement<'a> = Arc<GreenNode<'a, AdaElementType>>;

/// Ada 语法树中所有可能的元素类型。
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AdaElementType {
    /// Root node
    Root,
    /// Compilation unit
    CompilationUnit,
    /// Context clause (with, use)
    ContextClause,
    /// Pragma
    Pragma,
    /// Subprogram declaration
    SubprogramDeclaration,
    /// Package declaration
    PackageDeclaration,
    /// Type declaration
    TypeDeclaration,
    /// Object declaration
    ObjectDeclaration,
    /// Statement
    Statement,
    /// Expression
    Expression,
    /// Error node
    Error,

    /// Identifier
    Identifier,
    /// Literal
    LiteralExpression,
    /// Identifier expression
    IdentifierExpression,
    /// Parenthesized expression
    ParenthesizedExpression,
    /// Source file
    SourceFile,
    /// Parameter list
    ParameterList,
    /// Block expression
    BlockExpression,
    /// Use item
    UseItem,
    /// Module item
    ModuleItem,
    /// Struct item
    StructItem,
    /// Enum item
    EnumItem,
    /// Let statement
    LetStatement,
    /// If expression
    IfExpression,
    /// While expression
    WhileExpression,
    /// Loop expression
    LoopExpression,
    /// For expression
    ForExpression,
    /// Call expression
    CallExpression,
    /// Index expression
    IndexExpression,
    /// Field expression
    FieldExpression,
    /// Binary expression
    BinaryExpression,
    /// Unary expression
    UnaryExpression,
}

impl ElementType for AdaElementType {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::Root | Self::SourceFile)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        use UniversalElementRole::*;
        match self {
            Self::Root | Self::SourceFile => Root,
            Self::CompilationUnit | Self::SubprogramDeclaration | Self::PackageDeclaration | Self::TypeDeclaration | Self::ObjectDeclaration | Self::ModuleItem | Self::StructItem | Self::EnumItem => Definition,
            Self::ContextClause | Self::BlockExpression | Self::ParameterList | Self::ParenthesizedExpression => Container,
            Self::Statement | Self::Pragma | Self::UseItem | Self::LetStatement => Statement,
            Self::Expression
            | Self::BinaryExpression
            | Self::UnaryExpression
            | Self::IfExpression
            | Self::WhileExpression
            | Self::LoopExpression
            | Self::ForExpression
            | Self::IdentifierExpression
            | Self::LiteralExpression
            | Self::IndexExpression
            | Self::FieldExpression => Expression,
            Self::CallExpression => Call,
            Self::Identifier => Reference,
            Self::Error => Error,
        }
    }
}

impl From<AdaTokenType> for AdaElementType {
    fn from(token_type: AdaTokenType) -> Self {
        match token_type {
            AdaTokenType::Identifier => Self::Identifier,
            AdaTokenType::StringLiteral | AdaTokenType::CharacterLiteral | AdaTokenType::NumberLiteral => Self::LiteralExpression,
            _ => Self::Error,
        }
    }
}
