use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type HtmlToken = Token<HtmlTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HtmlTokenType {
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

impl TokenType for HtmlTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::TagOpen | Self::TagClose | Self::TagSlashOpen | Self::TagSelfClose => UniversalTokenRole::Operator,
            Self::TagName => UniversalTokenRole::Name,
            Self::AttributeName => UniversalTokenRole::Name,
            Self::AttributeValue => UniversalTokenRole::Literal,
            Self::Text => UniversalTokenRole::None,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Equal => UniversalTokenRole::Operator,
            Self::Quote => UniversalTokenRole::Operator,
            Self::Doctype => UniversalTokenRole::Keyword,
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            _ => UniversalTokenRole::None,
        }
    }
}
