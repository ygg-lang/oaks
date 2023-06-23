use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type TypstToken = Token<TypstTokenType>;

impl TokenType for TypstTokenType {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypstTokenType {
    // 节点种类
    Root,
    Document,
    Block,

    // 内容元素
    Heading,
    Paragraph,
    List,
    ListItem,
    EnumItem,
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
    Backtick,

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
