use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum YamlElementType {
    Token(crate::lexer::token_type::YamlTokenType),
    Document,
    Root,
    Error,
}

impl ElementType for YamlElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Document => UniversalElementRole::Container,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::YamlTokenType> for YamlElementType {
    fn from(token: crate::lexer::token_type::YamlTokenType) -> Self {
        Self::Token(token)
    }
}
