use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MatlabSyntaxKind {
    // 基础标记
    Whitespace,
    Newline,
    Comment,
    BlockComment,

    // 标识符和字面量
    Identifier,
    Number,
    String,
    Character,

    // 关键字
    Function,
    End,
    If,
    Else,
    Elseif,
    While,
    For,
    Break,
    Continue,
    Return,
    Switch,
    Case,
    Otherwise,
    Try,
    Catch,
    Global,
    Persistent,
    Classdef,
    Properties,
    Methods,
    Events,

    // 运算符
    Plus,          // +
    Minus,         // -
    Times,         // *
    Divide,        // /
    Power,         // ^
    LeftDivide,    // \
    DotTimes,      // .*
    DotDivide,     // ./
    DotPower,      // .^
    DotLeftDivide, // .\

    // 比较运算符
    Equal,        // ==
    NotEqual,     // ~=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // 逻辑运算符
    And,    // &
    Or,     // |
    Not,    // ~
    AndAnd, // &&
    OrOr,   // ||

    // 赋值运算符
    Assign, // =

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Semicolon,    // ;
    Comma,        // ,
    Dot,          // .
    Colon,        // :
    Question,     // ?
    At,           // @

    // 特殊运算符
    Transpose,    // '
    DotTranspose, // .'

    // 泛化类型
    Operator,
    Delimiter,

    // 错误处理
    Error,

    // 文档结构
    Script,
    FunctionDef,
    ClassDef,
    Block,
    Expression,
    Statement,

    // EOF
    Eof,
}

impl MatlabSyntaxKind {
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Self::Script | Self::FunctionDef | Self::ClassDef | Self::Block | Self::Expression | Self::Statement)
    }
}

impl TokenType for MatlabSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment | Self::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for MatlabSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            Self::Script => UniversalElementRole::Root,
            Self::FunctionDef | Self::ClassDef => UniversalElementRole::Detail,
            _ => UniversalElementRole::None,
        }
    }
}
