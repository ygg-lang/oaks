use crate::lexer::CsvTokenType;
use oak_core::{ElementType, UniversalElementRole};
use serde::{Deserialize, Serialize};

/// CSV element type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CsvElementType {
    /// Source file (root)
    SourceFile,
    /// Record (row)
    Record,
    /// Field
    Field,
}

impl ElementType for CsvElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Record => UniversalElementRole::Container,
            Self::Field => UniversalElementRole::Value,
        }
    }
}

impl From<CsvTokenType> for CsvElementType {
    fn from(token: CsvTokenType) -> Self {
        match token {
            _ => Self::Field,
        }
    }
}
