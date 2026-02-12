use crate::lexer::token_type::NoteTokenType;
use oak_core::{ElementType, UniversalElementRole};

/// Element type for Notedown AST
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum NoteElementType {
    /// Root node of the document
    Root,
    /// Heading node
    Heading,
    /// List container
    List,
    /// Single list item
    ListItem,
    /// Table container
    Table,
    /// Single table row
    TableRow,
    /// Code block node
    CodeBlock,
    /// Paragraph of text
    Paragraph,
    /// Blockquote container
    Blockquote,
    /// Horizontal rule separator
    HorizontalRule,
    /// Link container
    Link,
    /// Image container
    Image,
    /// HTML content
    Html,
    /// Error node
    Error,

    /// Token-derived elements
    Token(NoteTokenType),
}

impl ElementType for NoteElementType {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Heading | Self::Paragraph | Self::Blockquote | Self::HorizontalRule | Self::CodeBlock => UniversalElementRole::Statement,
            Self::List | Self::Table | Self::Link | Self::Image | Self::Html => UniversalElementRole::Container,
            Self::ListItem | Self::TableRow => UniversalElementRole::Container,
            Self::Error => UniversalElementRole::Error,
            Self::Token(_) => UniversalElementRole::None,
        }
    }
}

impl From<NoteTokenType> for NoteElementType {
    fn from(token: NoteTokenType) -> Self {
        Self::Token(token)
    }
}
