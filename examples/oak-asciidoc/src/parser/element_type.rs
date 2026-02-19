use oak_core::{ElementType, UniversalElementRole};

/// Element types for the AsciiDoc language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum AsciidocElementType {
    /// The root of the document.
    Root,
    /// A paragraph of text.
    Paragraph,
    /// A heading.
    Heading,
    /// A comment.
    Comment,
    /// A block.
    Block,
    /// A list.
    List,
    /// A list item.
    ListItem,
    /// A table.
    Table,
    /// A table row.
    TableRow,
    /// A table cell.
    TableCell,
    /// A table separator.
    TableSeparator,
    /// A table caption.
    TableCaption,
    /// A code block.
    CodeBlock,
    /// A horizontal rule.
    HorizontalRule,
    /// A macro.
    Macro,
    /// An attribute.
    Attribute,
    /// A cross-reference.
    CrossReference,
    /// A footnote reference.
    FootnoteReference,
    /// A footnote definition.
    FootnoteDefinition,
    /// Emphasized text.
    Emphasis,
    /// Strong text.
    Strong,
    /// Monospace text.
    Monospace,
    /// A link.
    Link,
    /// An image.
    Image,
    /// An include directive.
    Include,
    /// A conditional directive (ifdef).
    Ifdef,
    /// A conditional directive (ifndef).
    Ifndef,
    /// A conditional directive (endif).
    Endif,
    /// Plain text.
    Text,
    /// Whitespace.
    Whitespace,
    /// A newline.
    Newline,
    /// An error element.
    Error,
}

impl ElementType for AsciidocElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::AsciidocTokenType> for AsciidocElementType {
    fn from(token: crate::lexer::token_type::AsciidocTokenType) -> Self {
        match token {
            crate::lexer::token_type::AsciidocTokenType::Text => AsciidocElementType::Text,
            crate::lexer::token_type::AsciidocTokenType::Whitespace => AsciidocElementType::Whitespace,
            crate::lexer::token_type::AsciidocTokenType::Newline => AsciidocElementType::Newline,
            crate::lexer::token_type::AsciidocTokenType::Comment => AsciidocElementType::Comment,
            crate::lexer::token_type::AsciidocTokenType::Heading => AsciidocElementType::Heading,
            crate::lexer::token_type::AsciidocTokenType::BlockDelimiter => AsciidocElementType::Block,
            crate::lexer::token_type::AsciidocTokenType::ListItemMarker => AsciidocElementType::ListItem,
            crate::lexer::token_type::AsciidocTokenType::Table => AsciidocElementType::Table,
            crate::lexer::token_type::AsciidocTokenType::TableCell => AsciidocElementType::TableCell,
            crate::lexer::token_type::AsciidocTokenType::TableSeparator => AsciidocElementType::TableSeparator,
            crate::lexer::token_type::AsciidocTokenType::TableCaption => AsciidocElementType::TableCaption,
            crate::lexer::token_type::AsciidocTokenType::CodeBlock => AsciidocElementType::CodeBlock,
            crate::lexer::token_type::AsciidocTokenType::HorizontalRule => AsciidocElementType::HorizontalRule,
            crate::lexer::token_type::AsciidocTokenType::Macro => AsciidocElementType::Macro,
            crate::lexer::token_type::AsciidocTokenType::Attribute => AsciidocElementType::Attribute,
            crate::lexer::token_type::AsciidocTokenType::CrossReference => AsciidocElementType::CrossReference,
            crate::lexer::token_type::AsciidocTokenType::FootnoteReference => AsciidocElementType::FootnoteReference,
            crate::lexer::token_type::AsciidocTokenType::FootnoteDefinition => AsciidocElementType::FootnoteDefinition,
            crate::lexer::token_type::AsciidocTokenType::Emphasis => AsciidocElementType::Emphasis,
            crate::lexer::token_type::AsciidocTokenType::Strong => AsciidocElementType::Strong,
            crate::lexer::token_type::AsciidocTokenType::Monospace => AsciidocElementType::Monospace,
            crate::lexer::token_type::AsciidocTokenType::Link => AsciidocElementType::Link,
            crate::lexer::token_type::AsciidocTokenType::Image => AsciidocElementType::Image,
            crate::lexer::token_type::AsciidocTokenType::Include => AsciidocElementType::Include,
            crate::lexer::token_type::AsciidocTokenType::Ifdef => AsciidocElementType::Ifdef,
            crate::lexer::token_type::AsciidocTokenType::Ifndef => AsciidocElementType::Ifndef,
            crate::lexer::token_type::AsciidocTokenType::Endif => AsciidocElementType::Endif,
            _ => AsciidocElementType::Error,
        }
    }
}
