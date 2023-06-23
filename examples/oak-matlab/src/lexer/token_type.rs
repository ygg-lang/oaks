use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type MatlabToken = Token<MatlabTokenType>;

impl MatlabTokenType {
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Self::Script | Self::FunctionDef | Self::ClassDef | Self::Block | Self::Expression | Self::Statement)
    }
}

impl TokenType for MatlabTokenType {
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
pub enum MatlabTokenType {
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
    At,           // ↯

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
