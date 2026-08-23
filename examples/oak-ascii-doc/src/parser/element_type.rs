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
                match token {
            crate::lexer::token_type::AsciiDocTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::AsciiDocTokenType::Newline => Self::Newline,
            crate::lexer::token_type::AsciiDocTokenType::Header1 => Self::Header1,
            crate::lexer::token_type::AsciiDocTokenType::Header2 => Self::Header2,
            crate::lexer::token_type::AsciiDocTokenType::Header3 => Self::Header3,
            crate::lexer::token_type::AsciiDocTokenType::Header4 => Self::Header4,
            crate::lexer::token_type::AsciiDocTokenType::Header5 => Self::Header5,
            crate::lexer::token_type::AsciiDocTokenType::Header6 => Self::Header6,
            crate::lexer::token_type::AsciiDocTokenType::BoldMarker => Self::BoldMarker,
            crate::lexer::token_type::AsciiDocTokenType::ItalicMarker => Self::ItalicMarker,
            crate::lexer::token_type::AsciiDocTokenType::MonospaceMarker => Self::MonospaceMarker,
            crate::lexer::token_type::AsciiDocTokenType::CodeBlockMarker => Self::CodeBlockMarker,
            crate::lexer::token_type::AsciiDocTokenType::LinkMarker => Self::LinkMarker,
            crate::lexer::token_type::AsciiDocTokenType::ListMarker => Self::ListMarker,
            crate::lexer::token_type::AsciiDocTokenType::TableDelimiter => Self::TableDelimiter,
            crate::lexer::token_type::AsciiDocTokenType::Comment => Self::Comment,
            crate::lexer::token_type::AsciiDocTokenType::Text => Self::Text,
            crate::lexer::token_type::AsciiDocTokenType::LineBreak => Self::LineBreak,
            crate::lexer::token_type::AsciiDocTokenType::PageBreak => Self::PageBreak,
            crate::lexer::token_type::AsciiDocTokenType::Delimiter => Self::Delimiter,
            crate::lexer::token_type::AsciiDocTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::AsciiDocTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::AsciiDocTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::AsciiDocTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::AsciiDocTokenType::Colon => Self::Colon,
            crate::lexer::token_type::AsciiDocTokenType::Comma => Self::Comma,
            crate::lexer::token_type::AsciiDocTokenType::Dot => Self::Dot,
            crate::lexer::token_type::AsciiDocTokenType::Eof => Self::Eof,
            crate::lexer::token_type::AsciiDocTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
