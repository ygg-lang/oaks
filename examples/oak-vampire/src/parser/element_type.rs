use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum VampireElementType {
    Root,
}

impl ElementType for VampireElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
        }
    }
}

impl From<crate::lexer::token_type::VampireTokenType> for VampireElementType {
    fn from(_token: crate::lexer::token_type::VampireTokenType) -> Self {
        Self::Root
    }
}
