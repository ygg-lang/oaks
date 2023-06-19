use oak_core::SyntaxKind;
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

impl SyntaxKind for TypstSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(
            self,
            TypstSyntaxKind::Whitespace
                | TypstSyntaxKind::Newline
                | TypstSyntaxKind::LineComment
                | TypstSyntaxKind::BlockComment
        )
    }

    fn is_comment(&self) -> bool {
        matches!(self, TypstSyntaxKind::LineComment | TypstSyntaxKind::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, TypstSyntaxKind::Whitespace | TypstSyntaxKind::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(
            self,
            TypstSyntaxKind::Root
                | TypstSyntaxKind::Document
                | TypstSyntaxKind::Block
                | TypstSyntaxKind::Heading
                | TypstSyntaxKind::Paragraph
                | TypstSyntaxKind::List
                | TypstSyntaxKind::ListItem
                | TypstSyntaxKind::Table
                | TypstSyntaxKind::TableRow
                | TypstSyntaxKind::TableCell
                | TypstSyntaxKind::Figure
                | TypstSyntaxKind::Image
                | TypstSyntaxKind::Link
                | TypstSyntaxKind::Text
                | TypstSyntaxKind::Strong
                | TypstSyntaxKind::Emphasis
                | TypstSyntaxKind::Code
                | TypstSyntaxKind::Math
                | TypstSyntaxKind::InlineMath
                | TypstSyntaxKind::DisplayMath
                | TypstSyntaxKind::Raw
                | TypstSyntaxKind::Quote
                | TypstSyntaxKind::Script
                | TypstSyntaxKind::Expression
                | TypstSyntaxKind::FunctionCall
                | TypstSyntaxKind::Variable
                | TypstSyntaxKind::Assignment
                | TypstSyntaxKind::Conditional
                | TypstSyntaxKind::Loop
                | TypstSyntaxKind::Style
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            TypstSyntaxKind::Root
                | TypstSyntaxKind::Document
                | TypstSyntaxKind::Block
                | TypstSyntaxKind::Heading
                | TypstSyntaxKind::Paragraph
                | TypstSyntaxKind::List
                | TypstSyntaxKind::ListItem
                | TypstSyntaxKind::Table
                | TypstSyntaxKind::TableRow
                | TypstSyntaxKind::TableCell
                | TypstSyntaxKind::Figure
                | TypstSyntaxKind::Image
                | TypstSyntaxKind::Link
                | TypstSyntaxKind::Text
                | TypstSyntaxKind::Strong
                | TypstSyntaxKind::Emphasis
                | TypstSyntaxKind::Code
                | TypstSyntaxKind::Math
                | TypstSyntaxKind::InlineMath
                | TypstSyntaxKind::DisplayMath
                | TypstSyntaxKind::Raw
                | TypstSyntaxKind::Quote
                | TypstSyntaxKind::Script
                | TypstSyntaxKind::Expression
                | TypstSyntaxKind::FunctionCall
                | TypstSyntaxKind::Variable
                | TypstSyntaxKind::Assignment
                | TypstSyntaxKind::Conditional
                | TypstSyntaxKind::Loop
                | TypstSyntaxKind::Style
        )
    }
}
