use crate::lexer::token_type::VLangTokenType;
use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VLangElementType {
    SourceFile,
    Token(VLangTokenType),
}

impl ElementType for VLangElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Token(_) => UniversalElementRole::None,
        }
    }
}

impl From<VLangTokenType> for VLangElementType {
    fn from(token: VLangTokenType) -> Self {
        Self::Token(token)
    }
}
