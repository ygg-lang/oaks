use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

pub type OrgModeToken = Token<OrgModeSyntaxKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrgModeSyntaxKind {
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
    Backslash,

    // Literals
    Text,
    Number,
    Date,
    Time,
    Url,
    Email,

    // Special
    Newline,
    Whitespace,
    Comment,
    BlockDelimiter,
    PropertyName,
    PropertyValue,
    TagName,

    // Error and EOF
    Error,
    Eof,
}

impl TokenType for OrgModeSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment | Self::CommentBlock => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for OrgModeSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
