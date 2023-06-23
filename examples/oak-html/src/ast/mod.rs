#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// HTML attribute consisting of a name, an optional value, and its source span.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Attribute {
    /// The name of the attribute (e.g., "class").
    pub name: String,
    /// The optional value of the attribute (e.g., "container").
    pub value: Option<String>,
    /// The range in the source code where this attribute is defined.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents different types of nodes in an HTML document.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HtmlNode {
    /// An HTML element with a tag, attributes, and children.
    Element(Element),
    /// Plain text content.
    Text(Text),
    /// An HTML comment.
    Comment(String),
}

/// An HTML element consisting of a tag name, attributes, children nodes, and its source span.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Element {
    /// The name of the HTML tag (e.g., "div").
    pub tag_name: String,
    /// A list of attributes for this element.
    pub attributes: Vec<Attribute>,
    /// The child nodes contained within this element.
    pub children: Vec<HtmlNode>,
    /// The range in the source code where this element is defined.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents plain text content within an HTML document.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Text {
    /// The text content.
    pub content: String,
    /// The range in the source code where this text is located.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// The root node of an HTML document, containing a sequence of top-level nodes.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HtmlDocument {
    /// The top-level nodes in the document.
    pub nodes: Vec<HtmlNode>,
}
