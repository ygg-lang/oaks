use oak_core::{ElementType, UniversalElementRole};

/// XML element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum XmlElementType {
    /// Root element.
    Root,
    /// Prolog.
    Prolog,
    /// Element.
    Element,
    /// Start tag.
    StartTag,
    /// End tag.
    EndTag,
    /// Self-closing tag.
    SelfClosingTag,
    /// Attribute.
    Attribute,
    /// Text content.
    Text,
    /// Comment.
    Comment,
    /// CDATA section.
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
