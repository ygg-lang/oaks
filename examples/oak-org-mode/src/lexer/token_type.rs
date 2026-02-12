use oak_core::{Token, TokenType, UniversalTokenRole};

/// Org-mode token types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrgModeTokenType {
    // Node types
    /// Document.
    Document,
    /// Heading.
    Heading,
    /// Section.
    Section,
    /// Paragraph.
    Paragraph,
    /// List.
    List,
    /// List item.
    ListItem,
    /// Table.
    Table,
    /// Table row.
    TableRow,
    /// Table cell.
    TableCell,
    /// Block.
    Block,
    /// Code block.
    CodeBlock,
    /// Quote block.
    QuoteBlock,
    /// Example block.
    ExampleBlock,
    /// Verse block.
    VerseBlock,
    /// Comment block.
    CommentBlock,
    /// Drawer block.
    DrawerBlock,
    /// Property drawer.
    PropertyDrawer,
    /// Logbook drawer.
    LogbookDrawer,
    /// Link.
    Link,
    /// Inline code.
    InlineCode,
    /// Bold.
    Bold,
    /// Italic.
    Italic,
    /// Underline.
    Underline,
    /// Strikethrough.
    Strikethrough,
    /// Verbatim.
    Verbatim,
    /// Timestamp.
    Timestamp,
    /// Tag.
    Tag,
    /// Priority.
    Priority,
    /// TODO keyword.
    TodoKeyword,
    /// DONE keyword.
    DoneKeyword,

    // Lexical types
    // Headings
    /// Heading level 1.
    HeadingLevel1,
    /// Heading level 2.
    HeadingLevel2,
    /// Heading level 3.
    HeadingLevel3,
    /// Heading level 4.
    HeadingLevel4,
    /// Heading level 5.
    HeadingLevel5,
    /// Heading level 6.
    HeadingLevel6,

    // Keywords
    /// TODO.
    Todo,
    /// DONE.
    Done,
    /// NEXT.
    Next,
    /// WAITING.
    Waiting,
    /// CANCELLED.
    Cancelled,

    // Priorities
    /// Priority A.
    PriorityA,
    /// Priority B.
    PriorityB,
    /// Priority C.
    PriorityC,

    // Symbols
    /// `*`.
    Star,
    /// `+`.
    Plus,
    /// `-`.
    Minus,
    /// `#`.
    Hash,
    /// `|`.
    Pipe,
    /// `:`.
    Colon,
    /// `[`.
    LeftBracket,
    /// `]`.
    RightBracket,
    /// `(`.
    LeftParen,
    /// `)`.
    RightParen,
    /// `{`.
    LeftBrace,
    /// `}`.
    RightBrace,
    /// `<`.
    LessThan,
    /// `>`.
    GreaterThan,
    /// `=`.
    Equal,
    /// `_`.
    Underscore,
    /// `~`.
    Tilde,
    /// `/`.
    Slash,
    /// `^`.
    Caret,
    /// `\`.
    Backslash,
    /// `,`.
    Comma,
    /// `;`.
    Semicolon,
    /// `.`.
    Dot,
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    /// Number.
    Number,
    /// Date.
    Date,
    /// Text.
    Text,
    /// Error.
    Error,
}

impl TokenType for OrgModeTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            Self::Whitespace | Self::Newline => Whitespace,
            Self::Comment => Comment,
            Self::Number | Self::Date => Literal,
            Self::HeadingLevel1 | Self::HeadingLevel2 | Self::HeadingLevel3 | Self::HeadingLevel4 | Self::HeadingLevel5 | Self::HeadingLevel6 => Keyword,
            Self::Todo | Self::Done | Self::Next | Self::Waiting | Self::Cancelled => Keyword,
            Self::PriorityA | Self::PriorityB | Self::PriorityC => Literal,
            Self::Text => None,
            Self::Error => Error,
            _ => Punctuation,
        }
    }
}
