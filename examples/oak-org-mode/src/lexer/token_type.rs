use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OrgModeTokenType {
    // Node types
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

    // Lexical types
    // Headings
    HeadingLevel1,
    HeadingLevel2,
    HeadingLevel3,
    HeadingLevel4,
    HeadingLevel5,
    HeadingLevel6,

    // Keywords
    Todo,
    Done,
    Next,
    Waiting,
    Cancelled,

    // Priorities
    PriorityA,
    PriorityB,
    PriorityC,

    // Symbols
    Star,
    Plus,
    Minus,
    Hash,
    Pipe,
    Colon,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LessThan,
    GreaterThan,
    Equal,
    Underscore,
    Tilde,
    Slash,
    Caret,
    Backslash,
    Comma,
    Semicolon,
    Dot,
    Whitespace,
    Newline,
    Comment,
    Number,
    Date,
    Text,
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
