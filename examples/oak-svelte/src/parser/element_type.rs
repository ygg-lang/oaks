//! Svelte element types.

use oak_core::{ElementType, UniversalElementRole};

/// Svelte element type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SvelteElementType {
    /// Root node.
    Root,
    /// Element node.
    Element,
    /// Tag node.
    Tag,
    /// Closing tag node.
    CloseTag,
    /// Attribute node.
    Attribute,
    /// Attribute name.
    AttributeName,
    /// Attribute value.
    AttributeValue,
    /// Expression node (interpolation).
    Expression,
    /// Svelte block (if, each, await).
    Block,
    /// Block header (e.g., {#if expression}).
    BlockHeader,
    /// Block footer (e.g., {/if}).
    BlockFooter,
    /// Block content.
    BlockContent,
    /// Block branch (e.g., {:else}).
    BlockBranch,
    /// Text node.
    TextNode,
    /// Comment node.
    CommentNode,
    /// Identifier node.
    Identifier,
    /// Whitespace.
    Whitespace,
    /// Error node.
    Error,
}

impl ElementType for SvelteElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Element => UniversalElementRole::Container,
            Self::Tag | Self::CloseTag => UniversalElementRole::Name,
            Self::Attribute => UniversalElementRole::Attribute,
            Self::Expression => UniversalElementRole::Expression,
            Self::Block => UniversalElementRole::Container,
            Self::TextNode => UniversalElementRole::Value,
            Self::CommentNode => UniversalElementRole::None,
            Self::Identifier => UniversalElementRole::Name,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::SvelteTokenType> for SvelteElementType {
    fn from(token: crate::lexer::token_type::SvelteTokenType) -> Self {
        use crate::lexer::token_type::SvelteTokenType as T;
        match token {
            T::Root => Self::Root,
            T::Element => Self::Element,
            T::Attribute => Self::Attribute,
            T::Expression => Self::Expression,
            T::Block => Self::Block,
            T::Identifier => Self::Identifier,
            T::Whitespace => Self::Whitespace,
            T::Comment => Self::CommentNode,
            T::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
