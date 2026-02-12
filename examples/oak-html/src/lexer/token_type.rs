use oak_core::{Token, TokenType, UniversalTokenRole};

/// HTML token type alias.
pub type HtmlToken = Token<HtmlTokenType>;

/// HTML token types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HtmlTokenType {
    /// Opening tag bracket `<`.
    TagOpen,
    /// Closing tag bracket `>`.
    TagClose,
    /// Opening tag with slash `</`.
    TagSlashOpen,
    /// Self-closing tag slash `/>`.
    TagSelfClose,
    /// Tag name (e.g., `div`, `p`).
    TagName,
    /// Attribute name (e.g., `id`, `class`).
    AttributeName,
    /// Attribute value.
    AttributeValue,
    /// Attribute node.
    Attribute,
    /// Text content between tags.
    Text,
    /// HTML comment `<!-- ... -->`.
    Comment,
    /// Equal sign `=` in attributes.
    Equal,
    /// Quote `"` or `'`.
    Quote,
    /// Doctype declaration `<!DOCTYPE ...>`.
    Doctype,
    /// CDATA section `<![CDATA[ ... ]]>`.
    CData,
    /// Processing instruction `<? ... ?>`.
    ProcessingInstruction,
    /// Entity reference `&name;`.
    EntityRef,
    /// Character reference `&#123;` or `&#xabc;`.
    CharRef,
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Root document node.
    Document,
    /// HTML element.
    Element,
    /// End of file.
    Eof,
    /// Error token.
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
