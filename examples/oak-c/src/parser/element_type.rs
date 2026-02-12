use crate::lexer::CTokenType;
use oak_core::{ElementType, UniversalElementRole};

/// Represents all possible element kinds in the C programming language.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CElementType {
    /// A wrapper for tokens.
    Token(CTokenType),
    /// Root node representing the entire source file.
    Root,
    /// A function definition.
    FunctionDefinition,
    /// A list of parameters in a function declaration or definition.
    ParameterList,
    /// A compound statement (a block of code enclosed in braces).
    CompoundStatement,
    /// An expression statement.
    ExpressionStatement,
    /// A declaration statement.
    DeclarationStatement,
    /// A declarator.
    Declarator,
    /// An `if` statement.
    IfStatement,
    /// A `while` statement.
    WhileStatement,
    /// A `for` statement.
    ForStatement,
    /// A `return` statement.
    ReturnStatement,
    /// An error element used for recovery.
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
