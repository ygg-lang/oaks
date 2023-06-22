use crate::lexer::CppTokenType;
use oak_core::{ElementType, UniversalElementRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum CppElementType {
    /// A wrapper for tokens
    Token(CppTokenType),
    /// Root node of the source file
    SourceFile,
    /// Error token
    Error,
}

impl From<CppTokenType> for CppElementType {
    fn from(token: CppTokenType) -> Self {
        Self::Token(token)
    }
}

impl ElementType for CppElementType {
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
            _ => UniversalElementRole::Container,
        }
    }
}
