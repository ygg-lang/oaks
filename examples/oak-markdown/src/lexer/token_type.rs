use oak_core::{Token, TokenType, UniversalTokenRole};

/// Represents a token in a Markdown document.
pub type MarkdownToken = Token<MarkdownTokenType>;

/// Token types for the Markdown language.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MarkdownTokenType {
    /// Plain text.
    Text,
    /// Whitespace characters.
    Whitespace,
    /// A newline character.
    Newline,
    /// Heading level 1: `#`.
    Heading1,
    /// Heading level 2: `##`.
    Heading2,
    /// Heading level 3: `###`.
    Heading3,
    /// Heading level 4: `####`.
    Heading4,
    /// Heading level 5: `#####`.
    Heading5,
    /// Heading level 6: `######`.
    Heading6,
    /// The text content of a heading.
    HeadingText,
    /// Emphasized text: `*` or `_`.
    Emphasis,
    /// Strong text: `**` or `__`.
    Strong,
    /// Strikethrough text: `~~`.
    Strikethrough,
    /// Inline code: `` ` ``.
    InlineCode,
    /// A code block.
    CodeBlock,
    /// A code fence: ` ``` ` or `~~~`.
    CodeFence,
    /// The language identifier for a code block.
    CodeLanguage,
    /// A link.
    Link,
    /// The text of a link.
    LinkText,
    /// The URL of a link.
    LinkUrl,
    /// The title of a link.
    LinkTitle,
    /// An image.
    Image,
    /// The alt text of an image.
    ImageAlt,
    /// The URL of an image.
    ImageUrl,
    /// The title of an image.
    ImageTitle,
    /// A blockquote.
    Blockquote,
    /// The marker for a blockquote: `>`.
    BlockquoteMarker,
    /// A list.
    List,
    /// An item in a list.
    ListItem,
    /// The marker for a list item: `-`, `*`, `+`, or `1.`.
    ListMarker,
    /// An unordered list.
    UnorderedList,
    /// An ordered list.
    OrderedList,
    /// A task list.
    TaskList,
    /// The marker for a task: `[ ]` or `[x]`.
    TaskMarker,
    /// A table.
    Table,
    /// A row in a table.
    TableRow,
    /// A cell in a table.
    TableCell,
    /// A table header cell.
    TableHeader,
    /// A table separator row.
    TableSeparator,
    /// Table column alignment.
    TableAlignment,
    /// A horizontal rule: `---`, `***`, or `___`.
    HorizontalRule,
    /// Inline math: `$`.
    MathInline,
    /// Block math: `$$`.
    MathBlock,
    /// Front matter: `---`.
    FrontMatter,
    /// A footnote definition.
    FootnoteDefinition,
    /// A footnote reference.
    FootnoteReference,
    /// A definition list.
    DefinitionList,
    /// A term in a definition list.
    DefinitionTerm,
    /// A description in a definition list.
    DefinitionDescription,
    /// Superscript text.
    Superscript,
    /// Subscript text.
    Subscript,
    /// An abbreviation.
    Abbreviation,
    /// An HTML tag.
    HtmlTag,
    /// An HTML comment.
    HtmlComment,
    /// An XML tag.
    XmlTag,
    /// An XML comment.
    XmlComment,
    /// MDX import statement.
    MdxImport,
    /// MDX export statement.
    MdxExport,
    /// MDX JSX expression.
    MdxExpression,
    /// MDX JSX component.
    MdxComponent,
    /// An asterisk: `*`.
    Asterisk,
    /// An underscore: `_`.
    Underscore,
    /// A backtick: `` ` ``.
    Backtick,
    /// A tilde: `~`.
    Tilde,
    /// A hash sign: `#`.
    Hash,
    /// A less than sign: `<`.
    Less,
    /// A greater than sign: `>`.
    Greater,
    /// A left bracket: `[`.
    LBracket,
    /// A right bracket: `]`.
    RBracket,
    /// A left parenthesis: `(`.
    LParen,
    /// A right parenthesis: `)`.
    RParen,
    /// An exclamation mark: `!`.
    Exclamation,
    /// A pipe symbol: `|`.
    Pipe,
    /// A dash: `-`.
    Dash,
    /// A plus sign: `+`.
    Plus,
    /// A dot: `.`.
    Dot,
    /// A colon: `:`.
    Colon,
    /// A dollar sign: `$`.
    Dollar,
    /// A caret symbol: `^`.
    Caret,
    /// An escape character: `\`.
    Escape,
    /// An error token.
    Error,
    /// The root node of a Markdown document.
    Root,
    /// A Markdown document.
    Document,
    /// A paragraph of text.
    Paragraph,
    /// An automatic link (HTTP/HTTPS URL).
    AutoLink,
    /// End of stream.
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
