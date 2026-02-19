#![doc = include_str!("readme.md")]
use core::range::Range;

/// Stylus document root node
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusRoot {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the root node.
    pub span: Range<usize>,
    /// The list of top-level items in the document.
    pub items: Vec<StylusItem>,
}

/// Stylus top-level item
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StylusItem {
    /// CSS rule item.
    Rule(StylusRule),
    /// Comment item.
    Comment(StylusComment),
}

/// Stylus rule
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusRule {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the rule.
    pub span: Range<usize>,
    /// Selector of the rule.
    pub selector: String,
    /// Properties of the rule.
    pub properties: Vec<StylusProperty>,
}

/// Stylus comment
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusComment {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the comment.
    pub span: Range<usize>,
    /// Text of the comment.
    pub text: String,
}

/// Stylus property
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusProperty {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the property.
    pub span: Range<usize>,
    /// Name of the property.
    pub name: String,
    /// Value of the property.
    pub value: String,
}

impl StylusRoot {
    /// Creates a new StylusRoot with the given span.
    pub fn new(span: Range<usize>) -> Self {
        Self { span, items: Vec::new() }
    }
}
