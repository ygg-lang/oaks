use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HtmlElementType {
    TagOpen,
    TagClose,
    TagSlashOpen,
    TagSelfClose,
    TagName,
    AttributeName,
    AttributeValue,
    Attribute,
    Text,
    Comment,
    Equal,
    Quote,
    Doctype,
    CData,
    ProcessingInstruction,
    EntityRef,
    CharRef,
    Whitespace,
    Newline,
    Document,
    Element,
    Eof,
    Error,
}

impl ElementType for HtmlElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Document => UniversalElementRole::Root,
            Self::Element => UniversalElementRole::Container,
            Self::Attribute => UniversalElementRole::Attribute,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::HtmlTokenType> for HtmlElementType {
    fn from(token: crate::lexer::token_type::HtmlTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
