use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OrgModeElementType {
    Document,
    Heading,
    Section,
    Paragraph,
    List,
    ListItem,
    Table,
    TableRow,
    TableCell,
    Block,
    CodeBlock,
    QuoteBlock,
    ExampleBlock,
    VerseBlock,
    CommentBlock,
    DrawerBlock,
    PropertyDrawer,
    LogbookDrawer,
    Link,
    InlineCode,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Verbatim,
    Timestamp,
    Tag,
    Priority,
    TodoKeyword,
    DoneKeyword,
}

impl ElementType for OrgModeElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        use UniversalElementRole::*;
        match self {
            Self::Document => Root,
            Self::Heading => Definition,
            Self::Paragraph => Statement,
            Self::List => Container,
            Self::ListItem => Statement,
            Self::Table => Container,
            Self::TableRow => Statement,
            Self::TableCell => Statement,
            Self::CodeBlock | Self::QuoteBlock | Self::ExampleBlock | Self::VerseBlock | Self::CommentBlock | Self::Block => Embedded,
            Self::Link => Reference,
            Self::InlineCode | Self::Verbatim => Value,
            Self::Bold => Value,
            Self::Italic => Value,
            Self::Underline => Value,
            Self::Strikethrough => Value,
            _ => None,
        }
    }
}

impl From<crate::lexer::token_type::OrgModeTokenType> for OrgModeElementType {
    fn from(token: crate::lexer::token_type::OrgModeTokenType) -> Self {
        use crate::lexer::token_type::OrgModeTokenType::*;
        match token {
            Document => Self::Document,
            Heading => Self::Heading,
            Section => Self::Section,
            Paragraph => Self::Paragraph,
            List => Self::List,
            ListItem => Self::ListItem,
            Table => Self::Table,
            TableRow => Self::TableRow,
            TableCell => Self::TableCell,
            Block => Self::Block,
            CodeBlock => Self::CodeBlock,
            QuoteBlock => Self::QuoteBlock,
            ExampleBlock => Self::ExampleBlock,
            VerseBlock => Self::VerseBlock,
            CommentBlock => Self::CommentBlock,
            DrawerBlock => Self::DrawerBlock,
            PropertyDrawer => Self::PropertyDrawer,
            LogbookDrawer => Self::LogbookDrawer,
            Link => Self::Link,
            InlineCode => Self::InlineCode,
            Bold => Self::Bold,
            Italic => Self::Italic,
            Underline => Self::Underline,
            Strikethrough => Self::Strikethrough,
            Verbatim => Self::Verbatim,
            Timestamp => Self::Timestamp,
            Tag => Self::Tag,
            Priority => Self::Priority,
            TodoKeyword => Self::TodoKeyword,
            DoneKeyword => Self::DoneKeyword,
            _ => Self::Paragraph, // Default to Paragraph for other tokens when treated as element
        }
    }
}
