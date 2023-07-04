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
                match token {
            crate::lexer::token_type::XmlTokenType::Root => Self::Root,
            crate::lexer::token_type::XmlTokenType::Whitespace => Self::Root,
            crate::lexer::token_type::XmlTokenType::Newline => Self::Root,
            crate::lexer::token_type::XmlTokenType::Comment => Self::Comment,
            crate::lexer::token_type::XmlTokenType::Text => Self::Text,
            crate::lexer::token_type::XmlTokenType::Error => Self::Root,
            crate::lexer::token_type::XmlTokenType::Eof => Self::Root,
            crate::lexer::token_type::XmlTokenType::XmlDeclaration => Self::Root,
            crate::lexer::token_type::XmlTokenType::DoctypeDeclaration => Self::Root,
            crate::lexer::token_type::XmlTokenType::ProcessingInstruction => Self::Root,
            crate::lexer::token_type::XmlTokenType::CData => Self::CData,
            crate::lexer::token_type::XmlTokenType::StartTag => Self::StartTag,
            crate::lexer::token_type::XmlTokenType::EndTag => Self::EndTag,
            crate::lexer::token_type::XmlTokenType::SelfClosingTag => Self::SelfClosingTag,
            crate::lexer::token_type::XmlTokenType::TagName => Self::Root,
            crate::lexer::token_type::XmlTokenType::AttributeName => Self::Root,
            crate::lexer::token_type::XmlTokenType::AttributeValue => Self::Root,
            crate::lexer::token_type::XmlTokenType::StringLiteral => Self::Root,
            crate::lexer::token_type::XmlTokenType::LeftAngle => Self::Root,
            crate::lexer::token_type::XmlTokenType::RightAngle => Self::Root,
            crate::lexer::token_type::XmlTokenType::LeftAngleSlash => Self::Root,
            crate::lexer::token_type::XmlTokenType::SlashRightAngle => Self::Root,
            crate::lexer::token_type::XmlTokenType::Equals => Self::Root,
            crate::lexer::token_type::XmlTokenType::Quote => Self::Root,
            crate::lexer::token_type::XmlTokenType::SingleQuote => Self::Root,
            crate::lexer::token_type::XmlTokenType::Exclamation => Self::Root,
            crate::lexer::token_type::XmlTokenType::Question => Self::Root,
            crate::lexer::token_type::XmlTokenType::Ampersand => Self::Root,
            crate::lexer::token_type::XmlTokenType::Semicolon => Self::Root,
            crate::lexer::token_type::XmlTokenType::EntityReference => Self::Root,
            crate::lexer::token_type::XmlTokenType::CharacterReference => Self::Root,
            crate::lexer::token_type::XmlTokenType::Identifier => Self::Root,
            crate::lexer::token_type::XmlTokenType::SourceFile => Self::Root,
            crate::lexer::token_type::XmlTokenType::Element => Self::Element,
            crate::lexer::token_type::XmlTokenType::Attribute => Self::Attribute,
            crate::lexer::token_type::XmlTokenType::Prolog => Self::Prolog,
            _ => Self::Root,
        }
    }
}
