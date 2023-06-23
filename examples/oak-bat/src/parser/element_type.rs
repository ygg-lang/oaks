use crate::lexer::token_type::BatTokenType;
use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents all possible element kinds in the Windows Batch (BAT) scripting language.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BatElementType {
    /// A wrapper for tokens
    Token(BatTokenType),
    /// Root node representing the entire source file
    Root,
    /// A single command or a pipeline
    CommandStatement,
    /// An if statement
    IfStatement,
    /// A for loop
    ForStatement,
    /// A set statement
    SetStatement,
    /// A label definition
    LabelDefinition,
    /// Error node for syntax errors
    Error,
}

impl From<BatTokenType> for BatElementType {
    fn from(token: BatTokenType) -> Self {
        Self::Token(token)
    }
}

impl oak_core::ElementType for BatElementType {
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
            Self::LabelDefinition => UniversalElementRole::Definition,
            Self::CommandStatement | Self::IfStatement | Self::ForStatement | Self::SetStatement => UniversalElementRole::Statement,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
