use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypstElementType {
    Root,
    Heading,
    Quote,
    Math,
    Strong,
    Emphasis,
    ListItem,
    EnumItem,
    Raw,
}

impl ElementType for TypstElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::TypstTokenType> for TypstElementType {
    fn from(_token: crate::lexer::token_type::TypstTokenType) -> Self {
        Self::Root
    }
}
