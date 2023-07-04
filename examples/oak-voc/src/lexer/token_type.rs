use oak_core::{TokenType, UniversalTokenRole};

/// VOC token types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum VocTokenType {
    /// End of file.
    Eof,
    /// Whitespace.
    Whitespace,
    /// Section open tag (e.g., <template>).
    SectionOpen,
    /// Section close tag (e.g., </template>).
    SectionClose,
    /// Tag open (e.g., <div>).
    TagOpen,
    /// Tag close (e.g., </div>).
    TagClose,
    /// Self-closing tag (e.g., <br />).
    SelfCloseTag,
    /// Attribute (e.g., class="container").
    Attribute,
    /// Text content.
    Text,
    /// Style selector (e.g., .title).
    Selector,
    /// Style property (e.g., color).
    Property,
    /// Style value (e.g., #fff).
    Value,
    /// Block open (e.g., {).
    BlockOpen,
    /// Block close (e.g., }).
    BlockClose,
    /// Variable (e.g., $primary).
    Variable,
}

impl TokenType for VocTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Eof => UniversalTokenRole::Eof,
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::SectionOpen => UniversalTokenRole::Punctuation,
            Self::SectionClose => UniversalTokenRole::Punctuation,
            Self::TagOpen => UniversalTokenRole::Punctuation,
            Self::TagClose => UniversalTokenRole::Punctuation,
            Self::SelfCloseTag => UniversalTokenRole::Punctuation,
            Self::Attribute => UniversalTokenRole::Name,
            Self::Text => UniversalTokenRole::Literal,
            Self::Selector => UniversalTokenRole::Name,
            Self::Property => UniversalTokenRole::Name,
            Self::Value => UniversalTokenRole::Literal,
            Self::BlockOpen => UniversalTokenRole::Punctuation,
            Self::BlockClose => UniversalTokenRole::Punctuation,
            Self::Variable => UniversalTokenRole::Name,
        }
    }
}
