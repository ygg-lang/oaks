#![doc = include_str!("readme.md")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Javadoc root node.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct JavadocRoot {
    /// Description items
    pub description: Vec<JavadocItem>,
    /// Block tags
    pub tags: Vec<JavadocBlockTag>,
}

/// Javadoc content item (text or inline tag).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum JavadocItem {
    /// Raw text
    Text(String),
    /// Inline tag
    InlineTag(JavadocInlineTag),
}

/// Javadoc inline tag (e.g., {@link ...}).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct JavadocInlineTag {
    /// Tag name
    pub tag: String,
    /// Tag content
    pub content: String,
}

/// Javadoc block tag (e.g., @param, @return).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct JavadocBlockTag {
    /// Tag name
    pub tag: String,
    /// Tag content
    pub content: Vec<JavadocItem>,
}
