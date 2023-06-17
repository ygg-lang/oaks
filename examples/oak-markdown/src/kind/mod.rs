use oak_core::SyntaxKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    Document,
    Paragraph,

    // EOF
    Eof,
}

impl SyntaxKind for MarkdownSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::HtmlComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Document | Self::Paragraph)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Document | Self::Paragraph)
    }
}
