#![doc = include_str!("readme.md")]
use core::range::Range;

/// Root node of the Nginx AST.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct NginxRoot {
    /// The source range covered by this root node.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub range: Range<usize>,
    /// The list of top-level items in the Nginx configuration.
    pub items: Vec<NginxItem>,
}

/// Represents an item in the Nginx configuration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum NginxItem {
    /// A simple directive (e.g., `user nginx;`).
    Directive(Directive),
    /// A block directive (e.g., `http { ... }`).
    Block(Block),
    /// A comment.
    Comment(Comment),
}

/// A simple Nginx directive.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Directive {
    /// The name of the directive.
    pub name: String,
    /// The parameters of the directive.
    pub parameters: Vec<String>,
    /// The source range of the directive.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub range: Range<usize>,
}

/// An Nginx block directive.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Block {
    /// The name of the block.
    pub name: String,
    /// The parameters of the block.
    pub parameters: Vec<String>,
    /// The items inside the block.
    pub items: Vec<NginxItem>,
    /// The source range of the block.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub range: Range<usize>,
}

/// An Nginx comment.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Comment {
    /// The text content.
    pub text: String,
    /// The source range of the text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub range: Range<usize>,
}

impl NginxRoot {
    /// Creates a new `NginxRoot` with the specified range.
    pub fn new(range: Range<usize>, items: Vec<NginxItem>) -> Self {
        Self { range, items }
    }
}
