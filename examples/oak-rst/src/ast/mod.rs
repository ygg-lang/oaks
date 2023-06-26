use core::range::Range;

/// Root node of the reStructuredText Abstract Syntax Tree.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RstRoot {
    /// List of blocks in the document.
    pub blocks: Vec<Block>,
}

/// Block-level elements in reStructuredText.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Block {
    /// Heading.
    Heading(Heading),
    /// Paragraph.
    Paragraph(Paragraph),
    /// Code block.
    CodeBlock(CodeBlock),
    /// List.
    List(List),
    /// Table.
    Table(Table),
}

/// Heading element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Heading {
    /// Heading level.
    pub level: u32,
    /// Heading text content.
    pub content: String,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Paragraph element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Paragraph {
    /// Paragraph text content.
    pub content: String,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Code block element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListItem {
    /// List item content blocks.
    pub content: Vec<Block>,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Table element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TableRow {
    /// List of cells in the row.
    pub cells: Vec<TableCell>,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Table cell element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TableCell {
    /// Cell content string.
    pub content: String,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Inline-level elements in reStructuredText.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Inline {
    /// Plain text.
    Text(String),
    /// Bold text.
    Bold(String),
    /// Italic text.
    Italic(String),
    /// Code span.
    Code(String),
    /// Link.
    Link {
        /// Link text.
        text: String,
        /// Link URL.
        url: String,
    },
}

/// The reStructuredText language type.
pub type RstLanguage = crate::language::RstLanguage;
