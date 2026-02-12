use crate::lexer::DsvTokenType;
use oak_core::{ElementType, UniversalElementRole};

/// DSV element type
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DsvElementType {
    /// Source file (root)
    SourceFile,
    /// Record (row)
    Record,
    /// Field
    Field,
}

impl ElementType for DsvElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Record => UniversalElementRole::Container,
            Self::Field => UniversalElementRole::Value,
        }
    }
}

impl From<DsvTokenType> for DsvElementType {
    fn from(token: DsvTokenType) -> Self {
        match token {
            _ => Self::Field,
        }
    }
}
