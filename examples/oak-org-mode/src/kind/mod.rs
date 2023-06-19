use oak_core::{SyntaxKind, Token};
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

impl SyntaxKind for OrgModeSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        todo!()
    }

    fn is_whitespace(&self) -> bool {
        todo!()
    }

    fn is_token_type(&self) -> bool {
        todo!()
    }

    fn is_element_type(&self) -> bool {
        todo!()
    }
}
