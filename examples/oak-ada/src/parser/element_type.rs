use crate::lexer::AdaTokenType;
use oak_core::{ElementType, GreenNode, UniversalElementRole};
use std::sync::Arc;

/// Type alias for Ada syntax tree elements
pub type AdaElement<'a> = Arc<GreenNode<'a, AdaElementType>>;

/// Ada parser element types.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AdaElementType {
    /// Root node of the parse tree.
    Root,
    /// Compilation unit node.
    CompilationUnit,
    /// Context clause node (e.g., `with`, `use`).
    ContextClause,
    /// Pragma node.
    Pragma,
    /// Subprogram declaration node.
    SubprogramDeclaration,
    /// Package declaration node.
    PackageDeclaration,
    /// Type declaration node.
    TypeDeclaration,
    /// Object declaration node.
    ObjectDeclaration,
    /// Statement node.
    Statement,
    /// Expression node.
    Expression,
    /// Error node in the parse tree.
    Error,

    /// Identifier node.
    Identifier,
    /// Literal expression node.
    LiteralExpression,
    /// Identifier expression node.
    IdentifierExpression,
    /// Parenthesized expression node.
    ParenthesizedExpression,
    /// Source file node.
    SourceFile,
    /// Parameter list node.
    ParameterList,
    /// Block expression node.
    BlockExpression,
    /// Use item node.
    UseItem,
    /// Module item node.
    ModuleItem,
    /// Struct item node.
    StructItem,
    /// Enum item node.
    EnumItem,
    /// Let statement node.
    LetStatement,
    /// If expression node.
    IfExpression,
    /// While expression node.
    WhileExpression,
    /// Loop expression node.
    LoopExpression,
    /// For expression node.
    ForExpression,
    /// Call expression node.
    CallExpression,
    /// Index expression node.
    IndexExpression,
    /// Field expression node.
    FieldExpression,
    /// Binary expression node.
    BinaryExpression,
    /// Unary expression node.
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
