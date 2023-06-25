use oak_core::{ElementType, UniversalElementRole};

/// HTML element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HtmlElementType {
    /// Opening tag bracket `<`.
    TagOpen,
    /// Closing tag bracket `>`.
    TagClose,
    /// Opening tag with slash `</`.
    TagSlashOpen,
    /// Self-closing tag slash `/>`.
    TagSelfClose,
    /// Tag name.
    TagName,
    /// Attribute name.
    AttributeName,
    /// Attribute value.
    AttributeValue,
    /// Attribute node.
    Attribute,
    /// Text content.
    Text,
    /// Comment node.
    Comment,
    /// Equal sign.
    Equal,
    /// Quote.
    Quote,
    /// Doctype declaration.
    Doctype,
    /// CDATA section.
    CData,
    /// Processing instruction.
    ProcessingInstruction,
    /// Entity reference.
    EntityRef,
    /// Character reference.
    CharRef,
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Document root.
    Document,
    /// HTML element.
    Element,
    /// End of file.
    Eof,
    /// Error node.
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
