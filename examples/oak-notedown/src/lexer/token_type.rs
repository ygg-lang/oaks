use oak_core::{Token, TokenType, UniversalTokenRole};

/// Token type for Notedown
pub type NoteToken = Token<NoteTokenType>;

impl TokenType for NoteTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Text => UniversalTokenRole::None,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

/// Enum representing all possible token types in Notedown
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum NoteTokenType {
    /// Plain text
    Text,
    /// Whitespace characters
    Whitespace,
    /// Newline characters
    Newline,

    /// Level 1 heading marker (#)
    Heading1,
    /// Level 2 heading marker (##)
    Heading2,
    /// Level 3 heading marker (###)
    Heading3,
    /// Level 4 heading marker (####)
    Heading4,
    /// Level 5 heading marker (#####)
    Heading5,
    /// Level 6 heading marker (######)
    Heading6,
    /// Text content within a heading
    HeadingText,

    /// Emphasized text marker (* or _)
    Emphasis,
    /// Strong text marker (** or __)
    Strong,
    /// Strikethrough text marker (~~)
    Strikethrough,

    /// Inline code marker (`)
    InlineCode,
    /// Entire code block
    CodeBlock,
    /// Code block fence (``` or ~~~)
    CodeFence,
    /// Language identifier in a code block
    CodeLanguage,

    /// Link marker ([)
    Link,
    /// Text within a link
    LinkText,
    /// URL of a link
    LinkUrl,
    /// Title attribute of a link
    LinkTitle,
    /// Image marker (![)
    Image,
    /// Alt text of an image
    ImageAlt,
    /// URL of an image
    ImageUrl,
    /// Title attribute of an image
    ImageTitle,

    /// Unordered list container
    UnorderedList,
    /// Ordered list container
    OrderedList,
    /// Single list item
    ListItem,
    /// List marker (-, *, +, 1., etc.)
    ListMarker,
    /// Task list container
    TaskList,
    /// Task list marker ([ ] or [x])
    TaskMarker,

    /// Blockquote container
    Blockquote,
    /// Blockquote marker (>)
    BlockquoteMarker,

    /// Horizontal rule (---, ***, ___)
    HorizontalRule,

    /// Table container
    Table,
    /// Table row
    TableRow,
    /// Table cell
    TableCell,
    /// Table header cell
    TableHeader,
    /// Table separator (|)
    TableSeparator,
    /// Table alignment marker (:---, :---:, ---:)
    TableAlignment,

    /// HTML tag
    HtmlTag,
    /// HTML comment
    HtmlComment,

    /// Escape character (\)
    Escape,

    /// Left bracket ([)
    LeftBracket,
    /// Right bracket (])
    RightBracket,
    /// Left parenthesis (()
    LeftParen,
    /// Right parenthesis ())
    RightParen,
    /// Left angle bracket (<)
    LeftAngle,
    /// Right angle bracket (>)
    RightAngle,
    /// Asterisk (*)
    Asterisk,
    /// Underscore (_)
    Underscore,
    /// Backtick (`)
    Backtick,
    /// Tilde (~)
    Tilde,
    /// Hash marker (#)
    Hash,
    /// Pipe separator (|)
    Pipe,
    /// Dash or hyphen (-)
    Dash,
    /// Plus sign (+)
    Plus,
    /// Dot or period (.)
    Dot,
    /// Colon (:)
    Colon,
    /// Exclamation mark (!)
    Exclamation,

    /// Error token
    Error,

    /// Root node
    Root,
    /// Document node
    Document,
    /// Paragraph node
    Paragraph,
    /// Metadata section
    Metadata,

    /// End of file
    Eof,
}
