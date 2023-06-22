use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypstSyntaxKind {
    // 节点种类
    Root,
    Document,
    Block,

    // 内容元素
    Heading,
    Paragraph,
    List,
    ListItem,
    Table,
    TableRow,
    TableCell,
    Figure,
    Image,
    Link,

    // 文本元素
    Text,
    Strong,
    Emphasis,
    Code,
    Math,
    InlineMath,
    DisplayMath,
    Raw,
    Quote,

    // 脚本元素
    Script,
    Expression,
    FunctionCall,
    Variable,
    Assignment,
    Conditional,
    Loop,
    Import,
    Include,

    // 样式元素
    Set,
    Show,
    Style,
    Color,
    Font,
    Size,

    // 关键字
    Let,
    If,
    Else,
    For,
    While,
    Break,
    Continue,
    Return,
    True,
    False,

    // 操作符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,

    // 标点符号
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    Hash,
    At,
    Dollar,
    Underscore,

    // 字面量
    StringLiteral,
    NumericLiteral,
    Identifier,

    // 注释和空白
    LineComment,
    BlockComment,
    Whitespace,
    Newline,

    // 特殊符号
    Eof,
    Error,
}

impl TokenType for TypstSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            TypstSyntaxKind::Whitespace | TypstSyntaxKind::Newline => UniversalTokenRole::Whitespace,
            TypstSyntaxKind::LineComment | TypstSyntaxKind::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for TypstSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
