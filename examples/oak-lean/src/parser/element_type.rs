use oak_core::{ElementType, Parser, UniversalElementRole};

/// Element types for the Lean language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LeanElementType {
    /// Root node of the Lean AST.
    Root,
    /// Error node for representing syntax errors.
    Error,
}

impl ElementType for LeanElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::LeanTokenType> for LeanElementType {
    fn from(token: crate::lexer::token_type::LeanTokenType) -> Self {
        use crate::lexer::token_type::LeanTokenType;
        match token {
            LeanTokenType::Root => LeanElementType::Root,
            _ => LeanElementType::Error,
        }
    }
}
