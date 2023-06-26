use oak_core::{TokenType, UniversalTokenRole};

/// Token types for the AsciiDoc language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum AsciidocTokenType {
    /// Plain text.
    Text,
    /// Whitespace.
    Whitespace,
    /// A newline.
    Newline,
    /// A comment.
    Comment,
    /// A heading.
    Heading,
    /// A block delimiter.
    BlockDelimiter,
    /// A list item marker.
    ListItemMarker,
    /// A table.
    Table,
    /// A table cell.
    TableCell,
    /// A table separator.
    TableSeparator,
    /// A table caption.
    TableCaption,
    /// A code block.
    CodeBlock,
    /// A code block language.
    CodeBlockLanguage,
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
    /// An error token.
    Error,
    /// End of stream.
    EndOfStream,
}

impl TokenType for AsciidocTokenType {
    const END_OF_STREAM: Self = Self::EndOfStream;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}
