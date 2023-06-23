use crate::lexer::CobolTokenType;
use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum CobolElementType {
    /// A wrapper for tokens
    Token(CobolTokenType),
    /// Root node
    SourceFile,
    /// Identification Division
    IdentificationDivision,
    /// Program ID Paragraph
    ProgramIdParagraph,
    /// Environment Division
    EnvironmentDivision,
    /// Data Division
    DataDivision,
    /// Procedure Division
    ProcedureDivision,
    /// Display Statement
    DisplayStatement,
    /// Accept Statement
    AcceptStatement,
    /// Add Statement
    AddStatement,
    /// Stop Statement
    StopStatement,
    /// Move Statement
    MoveStatement,
    /// Perform Statement
    PerformStatement,
    /// Working-Storage Section
    WorkingStorageSection,
    /// Data Item
    DataItem,
    /// Literal
    Literal,
    /// Identifier
    Identifier,
    /// Error node
    Error,
}

impl From<CobolTokenType> for CobolElementType {
    fn from(token: CobolTokenType) -> Self {
        Self::Token(token)
    }
}

impl ElementType for CobolElementType {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::SourceFile)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
