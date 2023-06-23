use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Element types for the Svelte language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SvelteElementType {
    /// The root of a Svelte component.
    Root,
    /// An HTML or Svelte element.
    Element,
    /// An opening tag of an element.
    StartTag,
    /// A closing tag of an element.
    EndTag,
    /// An attribute of an element.
    Attribute,
    /// A Svelte directive (e.g., `on:click`, `bind:value`).
    Directive,
    /// A JavaScript expression.
    Expression,
    /// A Svelte interpolation `{...}`.
    Interpolation,
    /// A Svelte control block (e.g., `{#if}`, `{#each}`).
    ControlBlock,
    /// A middle part of a control block (e.g., `{:else}`).
    MiddleBlock,
    /// A Svelte 5 snippet.
    Snippet,
    /// A Svelte or HTML comment.
    Comment,
    /// Plain text content.
    Text,
    /// An error element.
    Error,
}

impl ElementType for SvelteElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Element => UniversalElementRole::Container,
            Self::StartTag | Self::EndTag => UniversalElementRole::Detail,
            Self::Attribute | Self::Directive => UniversalElementRole::Attribute,
            Self::Expression | Self::Interpolation => UniversalElementRole::Value,
            Self::ControlBlock | Self::MiddleBlock | Self::Snippet => UniversalElementRole::Container,
            Self::Comment => UniversalElementRole::Documentation,
            Self::Text => UniversalElementRole::Value,
            Self::Error => UniversalElementRole::Error,
        }
    }
}

impl From<crate::lexer::token_type::SvelteTokenType> for SvelteElementType {
    fn from(_token: crate::lexer::token_type::SvelteTokenType) -> Self {
        Self::Text // Placeholder
    }
}
