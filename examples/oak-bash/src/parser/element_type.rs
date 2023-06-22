use crate::lexer::BashTokenType;
use oak_core::{ElementType, UniversalElementRole};
use serde::{Deserialize, Serialize};

/// Represents all possible element kinds in the Bash shell scripting language.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BashElementType {
    /// A wrapper for tokens
    Token(BashTokenType),
    /// Root node representing the entire source file
    Root,
    /// A single command or a pipeline
    CommandStatement,
    /// An if statement
    IfStatement,
    /// A for loop
    ForStatement,
    /// A while loop
    WhileStatement,
    /// A function definition
    FunctionDefinition,
    /// Error node for syntax errors
    Error,
}

impl From<BashTokenType> for BashElementType {
    fn from(token: BashTokenType) -> Self {
        Self::Token(token)
    }
}

impl ElementType for BashElementType {
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
            Self::CommandStatement | Self::IfStatement | Self::ForStatement | Self::WhileStatement => UniversalElementRole::Statement,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
