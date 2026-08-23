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
                match token {
            crate::lexer::token_type::HtmlTokenType::TagOpen => Self::TagOpen,
            crate::lexer::token_type::HtmlTokenType::TagClose => Self::TagClose,
            crate::lexer::token_type::HtmlTokenType::TagSlashOpen => Self::TagSlashOpen,
            crate::lexer::token_type::HtmlTokenType::TagSelfClose => Self::TagSelfClose,
            crate::lexer::token_type::HtmlTokenType::TagName => Self::TagName,
            crate::lexer::token_type::HtmlTokenType::AttributeName => Self::AttributeName,
            crate::lexer::token_type::HtmlTokenType::AttributeValue => Self::AttributeValue,
            crate::lexer::token_type::HtmlTokenType::Attribute => Self::Attribute,
            crate::lexer::token_type::HtmlTokenType::Text => Self::Text,
            crate::lexer::token_type::HtmlTokenType::Comment => Self::Comment,
            crate::lexer::token_type::HtmlTokenType::Equal => Self::Equal,
            crate::lexer::token_type::HtmlTokenType::Quote => Self::Quote,
            crate::lexer::token_type::HtmlTokenType::Doctype => Self::Doctype,
            crate::lexer::token_type::HtmlTokenType::CData => Self::CData,
            crate::lexer::token_type::HtmlTokenType::ProcessingInstruction => Self::ProcessingInstruction,
            crate::lexer::token_type::HtmlTokenType::EntityRef => Self::EntityRef,
            crate::lexer::token_type::HtmlTokenType::CharRef => Self::CharRef,
            crate::lexer::token_type::HtmlTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::HtmlTokenType::Newline => Self::Newline,
            crate::lexer::token_type::HtmlTokenType::Document => Self::Document,
            crate::lexer::token_type::HtmlTokenType::Element => Self::Element,
            crate::lexer::token_type::HtmlTokenType::Eof => Self::Eof,
            crate::lexer::token_type::HtmlTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
