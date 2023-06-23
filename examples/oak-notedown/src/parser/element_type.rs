use crate::lexer::token_type::NoteTokenType;
use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum NoteElementType {
    Root,
    Heading,
    List,
    ListItem,
    Table,
    TableRow,
    CodeBlock,
    Paragraph,
    Blockquote,
    HorizontalRule,
    Link,
    Image,
    Html,
    Error,

    // Token-derived elements
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
