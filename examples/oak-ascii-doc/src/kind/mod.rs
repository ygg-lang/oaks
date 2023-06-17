use oak_core::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum AsciiDocSyntaxKind {
    // Trivia
    Whitespace,
    Newline,

    // Headers
    Header1,
    Header2,
    Header3,
    Header4,
    Header5,
    Header6,

    // Text formatting
    Bold,
    Italic,
    Monospace,
    Superscript,
    Subscript,
    Highlight,
    Strikethrough,

    // Lists
    UnorderedListItem,
    OrderedListItem,
    DescriptionListItem,

    // Blocks
    CodeBlock,
    QuoteBlock,
    SidebarBlock,
    ExampleBlock,
    LiteralBlock,
    PassthroughBlock,

    // Links and references
    Link,
    Image,
    CrossReference,
    Anchor,

    // Tables
    TableDelimiter,
    TableCell,

    // Attributes and macros
    Attribute,
    Macro,
    Include,

    // Comments
    Comment,

    // Text content
    Text,

    // Special characters
    LineBreak,
    PageBreak,
    Delimiter,

    // Composite nodes
    SourceFile,

    // Error handling
    Error,
    Eof,
}

impl SyntaxKind for AsciiDocSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Error)
    }
}
