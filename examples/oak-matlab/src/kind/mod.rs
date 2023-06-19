use oak_core::SyntaxKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

impl SyntaxKind for MatlabSyntaxKind {
    fn is_trivia(&self) -> bool {
        todo!()
    }

    fn is_comment(&self) -> bool {
        todo!()
    }

    fn is_whitespace(&self) -> bool {
        todo!()
    }

    fn is_token_type(&self) -> bool {
        todo!()
    }

    fn is_element_type(&self) -> bool {
        todo!()
    }
}
