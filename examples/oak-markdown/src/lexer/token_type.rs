use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type MarkdownToken = Token<MarkdownTokenType>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MarkdownTokenType {
    Text,
    Whitespace,
    Newline,
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    HeadingText,
    Emphasis,
    Strong,
    Strikethrough,
    InlineCode,
    CodeBlock,
    CodeFence,
    CodeLanguage,
    Link,
    LinkText,
    LinkUrl,
    LinkTitle,
    Image,
    ImageAlt,
    ImageUrl,
    ImageTitle,
    Blockquote,
    BlockquoteMarker,
    List,
    ListItem,
    ListMarker,
    UnorderedList,
    OrderedList,
    TaskList,
    TaskMarker,
    Table,
    TableRow,
    TableCell,
    TableHeader,
    TableSeparator,
    TableAlignment,
    HorizontalRule,
    MathInline,
    MathBlock,
    FrontMatter,
    FootnoteDefinition,
    FootnoteReference,
    DefinitionList,
    DefinitionTerm,
    DefinitionDescription,
    Superscript,
    Subscript,
    Abbreviation,
    HtmlTag,
    HtmlComment,
    XmlTag,
    XmlComment,
    Asterisk,
    Underscore,
    Backtick,
    Tilde,
    Hash,
    Less,
    Greater,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Exclamation,
    Pipe,
    Dash,
    Plus,
    Dot,
    Colon,
    Dollar,
    Caret,
    Escape,
    Error,
    Root,
    Document,
    Paragraph,
    EndOfStream,
}

impl TokenType for MarkdownTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::EndOfStream;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}
