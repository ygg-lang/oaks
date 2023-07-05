use oak_core::{ElementType, UniversalElementRole};

/// Element types for the AsciiDoc AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AsciiDocElementType {
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Level 1 header.
    Header1,
    /// Level 2 header.
    Header2,
    /// Level 3 header.
    Header3,
    /// Level 4 header.
    Header4,
    /// Level 5 header.
    Header5,
    /// Level 6 header.
    Header6,
    /// Bold text.
    BoldMarker,
    /// Italic text.
    ItalicMarker,
    /// Monospace text.
    MonospaceMarker,
    /// Code block.
    CodeBlockMarker,
    /// Hyperlink.
    LinkMarker,
    /// List item.
    ListMarker,
    /// Table cell delimiter.
    TableDelimiter,
    /// Comment block or line.
    Comment,
    /// Text content.
    Text,
    /// Hard line break.
    LineBreak,
    /// Page break.
    PageBreak,
    /// Attribute.
    Attribute,
    /// Admonition.
    Admonition,
    /// Paragraph.
    Paragraph,
    /// Section.
    Section,
    /// List.
    List,
    /// List item.
    ListItem,
    /// Code block.
    CodeBlock,
    /// Bold text element.
    Bold,
    /// Italic text element.
    Italic,
    /// Monospace text element.
    Monospace,
    /// Link element.
    Link,
    /// Generic delimiter.
    Delimiter,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Colon `:`.
    Colon,
    /// Comma `,`.
    Comma,
    /// Dot `.`.
    Dot,
    /// End of stream.
    Eof,
    /// Parsing error.
    Error,
    /// Root node of the document.
    Root,
}

impl ElementType for AsciiDocElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::AsciiDocTokenType> for AsciiDocElementType {
    fn from(token: crate::lexer::token_type::AsciiDocTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
