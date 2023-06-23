#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Root node of the Markdown Abstract Syntax Tree.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarkdownRoot {
    /// List of blocks in the document.
    pub blocks: Vec<Block>,
}

/// Block-level elements in Markdown.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Block {
    /// Heading (h1-h6).
    Heading(Heading),
    /// Paragraph.
    Paragraph(Paragraph),
    /// Code block.
    CodeBlock(CodeBlock),
    /// List.
    List(List),
    /// Blockquote.
    Blockquote(Blockquote),
    /// Horizontal rule.
    HorizontalRule(HorizontalRule),
    /// Table.
    Table(Table),
    /// HTML block.
    Html(Html),
}

/// Heading element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Heading {
    /// Heading level (1-6).
    pub level: u32,
    /// Heading text content.
    pub content: String,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Paragraph element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Paragraph {
    /// Paragraph text content.
    pub content: String,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Code block element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CodeBlock {
    /// Programming language identifier.
    pub language: Option<String>,
    /// Code content.
    pub content: String,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// List element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct List {
    /// Whether it's an ordered list.
    pub is_ordered: bool,
    /// List items.
    pub items: Vec<ListItem>,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// List item element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListItem {
    /// List item content blocks.
    pub content: Vec<Block>,
    /// Whether it's a task list item.
    pub is_task: bool,
    /// Task completion status (if is_task is true).
    pub is_checked: Option<bool>,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Blockquote element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Blockquote {
    /// Blockquote content blocks.
    pub content: Vec<Block>,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Horizontal rule element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HorizontalRule {
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Table element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Table {
    /// Table header row.
    pub header: TableRow,
    /// Table data rows.
    pub rows: Vec<TableRow>,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Table row element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TableRow {
    /// List of cells in the row.
    pub cells: Vec<TableCell>,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Table cell element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TableCell {
    /// Cell content string.
    pub content: String,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// HTML block element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Html {
    /// HTML content string.
    pub content: String,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
