#![doc = include_str!("readme.md")]
use oak_core::Range;

/// Typst AST root node.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypstRoot {
    /// The byte range span of this root node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The list of child items in this root node.
    pub items: Vec<TypstItem>,
}

impl TypstRoot {
    /// Creates a new TypstRoot with the given span.
    pub fn new(span: Range<usize>) -> Self {
        Self { span, items: Vec::new() }
    }
}

/// A syntax item in a Typst document.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypstItem {
    /// Plain text content.
    Text(String),
    /// Whitespace between items.
    Space,
    /// Paragraph break.
    Parbreak,
    /// A heading with level and content.
    Heading(TypstHeading),
    /// Strong (bold) text content.
    Strong(TypstRoot),
    /// Emphasized (italic) text content.
    Emphasis(TypstRoot),
    /// Mathematical content.
    Math(TypstRoot),
    /// Quoted content.
    Quote(TypstRoot),
    /// A list item.
    ListItem(TypstRoot),
    /// An enumerated list item.
    EnumItem(TypstRoot),
    /// A hyperlink.
    Link(TypstLink),
    /// Raw (verbatim) text content.
    Raw(String),
    /// A block element.
    Block(TypstRoot),
}

/// A heading in a Typst document.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypstHeading {
    /// The heading level (1-6).
    pub level: usize,
    /// The heading content.
    pub content: TypstRoot,
}

/// A hyperlink in a Typst document.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypstLink {
    /// The URL of the link.
    pub url: String,
    /// Optional link content (display text).
    pub content: Option<TypstRoot>,
}
