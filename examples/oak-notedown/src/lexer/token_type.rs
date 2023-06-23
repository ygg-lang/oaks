use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type NoteToken = Token<NoteTokenType>;

impl TokenType for NoteTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum NoteTokenType {
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
    Metadata,

    // EOF
    Eof,
}
