use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SolidityElementType {
    SourceFile,
    Eof,
    Error,
}

impl oak_core::TokenType for SolidityElementType {
    type Role = oak_core::UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            _ => oak_core::UniversalTokenRole::None,
        }
    }
}

impl SolidityElementType {
    pub fn is_token_type(&self) -> bool {
        true
    }

    pub fn is_element_type(&self) -> bool {
        false
    }
}

impl ElementType for SolidityElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::SolidityTokenType> for SolidityElementType {
    fn from(token: crate::lexer::token_type::SolidityTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
