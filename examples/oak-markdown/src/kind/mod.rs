use oak_core::{TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum MarkdownSyntaxKind {
    // 基础文本
    Text,
    Whitespace,
    Newline,

    // 标题
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    HeadingText,

    // 强调和加
    Emphasis,      // *text* or _text_
    Strong,        // **text** or __text__
    Strikethrough, // ~~text~~

    // 代码
    InlineCode,   // `code`
    CodeBlock,    // ```code```
    CodeFence,    // ``` or ~~~
    CodeLanguage, // language identifier in code block

    // 链接和图
    Link,
    LinkText,
    LinkUrl,
    LinkTitle,
    Image,
    ImageAlt,
    ImageUrl,
    ImageTitle,

    // 列表
    UnorderedList,
    OrderedList,
    ListItem,
    ListMarker, // -, *, +, 1., 2., etc.
    TaskList,
    TaskMarker, // [x] or [ ]

    // 引用
    Blockquote,
    BlockquoteMarker, // >

    // 分隔
    HorizontalRule, // --- or *** or ___

    // 表格
    Table,
    TableRow,
    TableCell,
    TableHeader,
    TableSeparator, // |
    TableAlignment, // :---, :---:, ---:

    // HTML
    HtmlTag,
    HtmlComment,

    // 转义字符
    Escape, // \

    // 特殊字符
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )
    LeftAngle,    // <
    RightAngle,   // >
    Asterisk,     // *
    Underscore,   // _
    Backtick,     // `
    Tilde,        // ~
    Hash,         // #
    Pipe,         // |
    Dash,         // -
    Plus,         // +
    Dot,          // .
    Colon,        // :
    Exclamation,  // !

    // 错误处理
    Error,

    // 文档结构
    Root,
    Document,
    Paragraph,

    // EOF
    EndOfStream,
}

impl TokenType for MarkdownSyntaxKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::EndOfStream;

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            Self::LeftBracket
            | Self::RightBracket
            | Self::LeftParen
            | Self::RightParen
            | Self::LeftAngle
            | Self::RightAngle
            | Self::Asterisk
            | Self::Underscore
            | Self::Backtick
            | Self::Tilde
            | Self::Hash
            | Self::Pipe
            | Self::Dash
            | Self::Plus
            | Self::Dot
            | Self::Colon
            | Self::Exclamation
            | Self::CodeFence
            | Self::BlockquoteMarker
            | Self::ListMarker
            | Self::TableSeparator => Punctuation,

            Self::Text | Self::HeadingText | Self::LinkText | Self::ImageAlt | Self::CodeLanguage => Name,

            Self::LinkUrl | Self::LinkTitle | Self::ImageUrl | Self::ImageTitle => Literal,

            Self::Escape => Escape,
            Self::Whitespace | Self::Newline => Whitespace,
            Self::HtmlComment => Comment,
            Self::Error => Error,
            _ => None,
        }
    }
}

impl oak_core::ElementType for MarkdownSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        use UniversalElementRole::*;
        match self {
            Self::Root | Self::Document => Root,

            // Hierarchy & Scoping
            Self::UnorderedList | Self::OrderedList | Self::TaskList | Self::Blockquote | Self::Table | Self::TableRow => Container,

            // Flow Control & Logic (Markup units)
            Self::Paragraph | Self::ListItem | Self::TableCell | Self::HorizontalRule => Statement,

            // Symbol Management
            Self::Heading1 | Self::Heading2 | Self::Heading3 | Self::Heading4 | Self::Heading5 | Self::Heading6 => Definition,

            Self::Link | Self::Image => Reference,

            // Atomic Values
            Self::Emphasis | Self::Strong | Self::Strikethrough | Self::InlineCode => Value,

            // Metadata
            Self::HtmlTag => Metadata,

            // Embedded
            Self::CodeBlock => Embedded,

            Self::Error => Error,
            _ => None,
        }
    }
}
