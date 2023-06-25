use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Markdown language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum MarkdownElementType {
    /// The root of the document.
    Root,
    /// A paragraph of text.
    Paragraph,
    /// A table.
    Table,
    /// Heading level 1.
    Heading1,
    /// Heading level 2.
    Heading2,
    /// Heading level 3.
    Heading3,
    /// Heading level 4.
    Heading4,
    /// Heading level 5.
    Heading5,
    /// Heading level 6.
    Heading6,
    /// A blockquote.
    Blockquote,
    /// A code block.
    CodeBlock,
    /// A list.
    List,
    /// A list item.
    ListItem,
    /// A horizontal rule.
    HorizontalRule,
    /// A math block.
    MathBlock,
    /// Front matter.
    FrontMatter,
    /// A footnote definition.
    FootnoteDefinition,
    /// Plain text.
    Text,
    /// Whitespace.
    Whitespace,
    /// A newline.
    Newline,
    /// The text of a heading.
    HeadingText,
    /// Emphasized text.
    Emphasis,
    /// Strong text.
    Strong,
    /// Strikethrough text.
    Strikethrough,
    /// Inline code.
    InlineCode,
    /// A code fence.
    CodeFence,
    /// A code language identifier.
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
    /// An unordered list.
    UnorderedList,
    /// An ordered list.
    OrderedList,
    /// A list marker.
    ListMarker,
    /// A task list.
    TaskList,
    /// A task marker.
    TaskMarker,
    /// A blockquote marker.
    BlockquoteMarker,
    /// A table row.
    TableRow,
    /// A table cell.
    TableCell,
    /// A table header cell.
    TableHeader,
    /// A table separator.
    TableSeparator,
    /// Table alignment information.
    TableAlignment,
    /// Inline math.
    MathInline,
    /// A footnote reference.
    FootnoteReference,
    /// A definition list.
    DefinitionList,
    /// A definition term.
    DefinitionTerm,
    /// A definition description.
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
    /// An error element.
    Error,
}

impl ElementType for MarkdownElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::MarkdownTokenType> for MarkdownElementType {
    fn from(token: crate::lexer::token_type::MarkdownTokenType) -> Self {
        match token {
            crate::lexer::token_type::MarkdownTokenType::Text => MarkdownElementType::Text,
            crate::lexer::token_type::MarkdownTokenType::Whitespace => MarkdownElementType::Whitespace,
            crate::lexer::token_type::MarkdownTokenType::Newline => MarkdownElementType::Newline,
            crate::lexer::token_type::MarkdownTokenType::Heading1 => MarkdownElementType::Heading1,
            crate::lexer::token_type::MarkdownTokenType::Heading2 => MarkdownElementType::Heading2,
            crate::lexer::token_type::MarkdownTokenType::Heading3 => MarkdownElementType::Heading3,
            crate::lexer::token_type::MarkdownTokenType::Heading4 => MarkdownElementType::Heading4,
            crate::lexer::token_type::MarkdownTokenType::Heading5 => MarkdownElementType::Heading5,
            crate::lexer::token_type::MarkdownTokenType::Heading6 => MarkdownElementType::Heading6,
            crate::lexer::token_type::MarkdownTokenType::HeadingText => MarkdownElementType::HeadingText,
            crate::lexer::token_type::MarkdownTokenType::Emphasis => MarkdownElementType::Emphasis,
            crate::lexer::token_type::MarkdownTokenType::Strong => MarkdownElementType::Strong,
            crate::lexer::token_type::MarkdownTokenType::Strikethrough => MarkdownElementType::Strikethrough,
            crate::lexer::token_type::MarkdownTokenType::InlineCode => MarkdownElementType::InlineCode,
            crate::lexer::token_type::MarkdownTokenType::CodeBlock => MarkdownElementType::CodeBlock,
            crate::lexer::token_type::MarkdownTokenType::CodeFence => MarkdownElementType::CodeFence,
            crate::lexer::token_type::MarkdownTokenType::CodeLanguage => MarkdownElementType::CodeLanguage,
            crate::lexer::token_type::MarkdownTokenType::Link => MarkdownElementType::Link,
            crate::lexer::token_type::MarkdownTokenType::LinkText => MarkdownElementType::LinkText,
            crate::lexer::token_type::MarkdownTokenType::LinkUrl => MarkdownElementType::LinkUrl,
            crate::lexer::token_type::MarkdownTokenType::LinkTitle => MarkdownElementType::LinkTitle,
            crate::lexer::token_type::MarkdownTokenType::Image => MarkdownElementType::Image,
            crate::lexer::token_type::MarkdownTokenType::ImageAlt => MarkdownElementType::ImageAlt,
            crate::lexer::token_type::MarkdownTokenType::ImageUrl => MarkdownElementType::ImageUrl,
            crate::lexer::token_type::MarkdownTokenType::ImageTitle => MarkdownElementType::ImageTitle,
            crate::lexer::token_type::MarkdownTokenType::UnorderedList => MarkdownElementType::UnorderedList,
            crate::lexer::token_type::MarkdownTokenType::OrderedList => MarkdownElementType::OrderedList,
            crate::lexer::token_type::MarkdownTokenType::ListItem => MarkdownElementType::ListItem,
            crate::lexer::token_type::MarkdownTokenType::ListMarker => MarkdownElementType::ListMarker,
            crate::lexer::token_type::MarkdownTokenType::TaskList => MarkdownElementType::TaskList,
            crate::lexer::token_type::MarkdownTokenType::TaskMarker => MarkdownElementType::TaskMarker,
            crate::lexer::token_type::MarkdownTokenType::Blockquote => MarkdownElementType::Blockquote,
            crate::lexer::token_type::MarkdownTokenType::BlockquoteMarker => MarkdownElementType::BlockquoteMarker,
            crate::lexer::token_type::MarkdownTokenType::HorizontalRule => MarkdownElementType::HorizontalRule,
            crate::lexer::token_type::MarkdownTokenType::Table => MarkdownElementType::Table,
            crate::lexer::token_type::MarkdownTokenType::TableRow => MarkdownElementType::TableRow,
            crate::lexer::token_type::MarkdownTokenType::TableCell => MarkdownElementType::TableCell,
            crate::lexer::token_type::MarkdownTokenType::TableHeader => MarkdownElementType::TableHeader,
            crate::lexer::token_type::MarkdownTokenType::TableSeparator => MarkdownElementType::TableSeparator,
            crate::lexer::token_type::MarkdownTokenType::TableAlignment => MarkdownElementType::TableAlignment,
            crate::lexer::token_type::MarkdownTokenType::MathInline => MarkdownElementType::MathInline,
            crate::lexer::token_type::MarkdownTokenType::MathBlock => MarkdownElementType::MathBlock,
            crate::lexer::token_type::MarkdownTokenType::FootnoteDefinition => MarkdownElementType::FootnoteDefinition,
            crate::lexer::token_type::MarkdownTokenType::FootnoteReference => MarkdownElementType::FootnoteReference,
            crate::lexer::token_type::MarkdownTokenType::FrontMatter => MarkdownElementType::FrontMatter,
            crate::lexer::token_type::MarkdownTokenType::DefinitionList => MarkdownElementType::DefinitionList,
            crate::lexer::token_type::MarkdownTokenType::DefinitionTerm => MarkdownElementType::DefinitionTerm,
            crate::lexer::token_type::MarkdownTokenType::DefinitionDescription => MarkdownElementType::DefinitionDescription,
            crate::lexer::token_type::MarkdownTokenType::Superscript => MarkdownElementType::Superscript,
            crate::lexer::token_type::MarkdownTokenType::Subscript => MarkdownElementType::Subscript,
            crate::lexer::token_type::MarkdownTokenType::Abbreviation => MarkdownElementType::Abbreviation,
            crate::lexer::token_type::MarkdownTokenType::HtmlTag => MarkdownElementType::HtmlTag,
            crate::lexer::token_type::MarkdownTokenType::HtmlComment => MarkdownElementType::HtmlComment,
            crate::lexer::token_type::MarkdownTokenType::XmlTag => MarkdownElementType::XmlTag,
            crate::lexer::token_type::MarkdownTokenType::XmlComment => MarkdownElementType::XmlComment,
            crate::lexer::token_type::MarkdownTokenType::Error => MarkdownElementType::Error,
            _ => MarkdownElementType::Error,
        }
    }
}
