use oak_core::{ElementType, UniversalElementRole};

/// Element types for Javadoc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum JavadocElementType {
    /// The root of the parse tree.
    Root,
    /// Whitespace characters.
    Whitespace,
    /// Line breaks.
    Newline,
    /// A Javadoc comment block.
    Comment,
    /// A Javadoc tag.
    Tag,
    /// A Javadoc block tag (e.g., @param, @return).
    BlockTag,
    /// A Javadoc inline tag (e.g., {@link}, {@code}).
    InlineTag,
    /// Main description of the Javadoc.
    Description,
    /// Text content.
    Text,
    /// HTML element.
    HtmlElement,
    /// An error in the parse tree.
    Error,
}

impl ElementType for JavadocElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Comment => UniversalElementRole::Documentation,
            Self::Description => UniversalElementRole::Documentation,
            Self::BlockTag | Self::InlineTag | Self::Tag => UniversalElementRole::Metadata,
            Self::Text => UniversalElementRole::Documentation,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::JavadocTokenType> for JavadocElementType {
    fn from(token: crate::lexer::token_type::JavadocTokenType) -> Self {
        use crate::lexer::token_type::JavadocTokenType as T;
        match token {
            T::Root => Self::Root,
            T::Whitespace => Self::Whitespace,
            T::Newline => Self::Newline,
            T::Text => Self::Text,
            T::Error => Self::Error,
            T::CommentStart | T::CommentEnd | T::Asterisk => Self::Whitespace,
            T::HtmlTag | T::HtmlEndTag | T::HtmlPTag | T::HtmlBrTag | T::HtmlCodeTag | T::HtmlPreTag | T::HtmlBTag | T::HtmlITag | T::HtmlEmTag | T::HtmlStrongTag | T::HtmlUlTag | T::HtmlOlTag | T::HtmlLiTag => Self::HtmlElement,
            _ => Self::Tag,
        }
    }
}
