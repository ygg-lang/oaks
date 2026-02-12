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
    pub name: String,
    pub parameters: Vec<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub range: Range<usize>,
}

/// An Nginx block directive.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Block {
    pub name: String,
    pub parameters: Vec<String>,
    pub items: Vec<NginxItem>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub range: Range<usize>,
}

/// An Nginx comment.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Comment {
    pub text: String,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub range: Range<usize>,
}

impl NginxRoot {
    /// Creates a new `NginxRoot` with the specified range.
    pub fn new(range: Range<usize>, items: Vec<NginxItem>) -> Self {
        Self { range, items }
    }
}
