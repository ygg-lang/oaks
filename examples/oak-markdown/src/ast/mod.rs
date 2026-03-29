#![doc = include_str!("readme.md")]
use core::range::Range;

/// Root node of the Markdown Abstract Syntax Tree.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarkdownRoot {
    /// List of blocks in the document.
    pub blocks: Vec<Block>,
}

/// Block-level elements in Markdown.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// Abbreviation definition.
    AbbreviationDefinition(AbbreviationDefinition),
    /// MDX import statement.
    MdxImport(MdxImport),
    /// MDX export statement.
    MdxExport(MdxExport),
    /// MDX JSX component.
    MdxComponent(MdxComponent),
}

/// Abbreviation definition element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AbbreviationDefinition {
    /// Abbreviation key.
    pub key: String,
    /// Abbreviation definition.
    pub definition: String,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Heading element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Blockquote {
    /// Blockquote content blocks.
    pub content: Vec<Block>,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Horizontal rule element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HorizontalRule {
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

/// HTML block element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Html {
    /// HTML content string.
    pub content: String,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Inline-level elements in Markdown.
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
        /// Link title (optional).
        title: Option<String>,
    },
    /// Image.
    Image {
        /// Alt text.
        alt: String,
        /// Image URL.
        url: String,
        /// Image title (optional).
        title: Option<String>,
    },
    /// Abbreviation usage.
    Abbreviation {
        /// Abbreviation key.
        key: String,
        /// Abbreviation definition.
        definition: String,
    },
    /// MDX JSX expression.
    MdxExpression {
        /// Expression content.
        content: String,
        /// Source code range.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

/// MDX import statement.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MdxImport {
    /// Import statement content.
    pub content: String,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// MDX export statement.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MdxExport {
    /// Export statement content.
    pub content: String,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// MDX JSX component.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MdxComponent {
    /// Component name.
    pub name: String,
    /// Component attributes.
    pub attributes: Vec<MdxAttribute>,
    /// Whether it's a self-closing tag.
    pub is_self_closing: bool,
    /// Child components (if not self-closing).
    pub children: Vec<Block>,
    /// Source code range.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// MDX component attribute.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MdxAttribute {
    /// Attribute name.
    pub name: String,
    /// Attribute value.
    pub value: Option<String>,
}
