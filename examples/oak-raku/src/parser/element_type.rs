use crate::lexer::token_type::RakuTokenType;
use oak_core::{ElementType, UniversalElementRole};

/// Raku element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RakuElementType {
    /// A token element.
    Token(RakuTokenType),
    /// The root element.
    Root,
    /// An expression element.
    Expression,
    /// A statement element.
    Statement,
    /// A block element.
    Block,
    /// A function definition.
    FunctionDefinition,
    /// A class definition.
    ClassDefinition,
    /// A module definition.
    ModuleDefinition,
    /// A variable declaration.
    VariableDeclaration,
    /// A call expression.
    CallExpression,
    /// A binary expression.
    BinaryExpression,
    /// A unary expression.
    UnaryExpression,
    /// A literal expression.
    LiteralExpression,
    /// An identifier expression.
    IdentifierExpression,
    /// An error element.
    Error,
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
            Self::Expression | Self::CallExpression | Self::BinaryExpression | Self::UnaryExpression | Self::LiteralExpression | Self::IdentifierExpression => UniversalElementRole::Expression,
            Self::Statement | Self::VariableDeclaration => UniversalElementRole::Statement,
            Self::Block => UniversalElementRole::Statement,
            Self::FunctionDefinition | Self::ClassDefinition | Self::ModuleDefinition => UniversalElementRole::Definition,
            Self::Error => UniversalElementRole::Error,
        }
    }
}

impl Default for RakuElementType {
    fn default() -> Self {
        Self::Root
    }
}
