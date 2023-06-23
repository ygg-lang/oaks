use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum XmlElementType {
    Root,
    Prolog,
    Element,
    StartTag,
    EndTag,
    SelfClosingTag,
    Attribute,
    Text,
    Comment,
    CData,
}

impl ElementType for XmlElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::XmlTokenType> for XmlElementType {
    fn from(token: crate::lexer::token_type::XmlTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
