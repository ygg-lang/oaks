use crate::lexer::CTokenType;
use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents all possible element kinds in the C programming language.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CElementType {
    /// A wrapper for tokens
    Token(CTokenType),
    /// Root node representing the entire source file
    Root,
    /// Function definition
    FunctionDefinition,
    /// Parameter list
    ParameterList,
    /// Compound statement (block)
    CompoundStatement,
    /// Expression statement
    ExpressionStatement,
    /// Declaration statement
    DeclarationStatement,
    /// If statement
    IfStatement,
    /// While statement
    WhileStatement,
    /// For statement
    ForStatement,
    /// Return statement
    ReturnStatement,
    /// Error element
    Error,
}

impl From<CTokenType> for CElementType {
    fn from(token: CTokenType) -> Self {
        Self::Token(token)
    }
}

impl ElementType for CElementType {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::FunctionDefinition => UniversalElementRole::Definition,
            Self::CompoundStatement | Self::ExpressionStatement | Self::DeclarationStatement | Self::IfStatement | Self::WhileStatement | Self::ForStatement | Self::ReturnStatement => UniversalElementRole::Statement,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
