use oak_core::{SyntaxKind, Token};
use serde::{Deserialize, Serialize};

pub type PascalToken = Token<PascalSyntaxKind>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PascalSyntaxKind {
    // 空白和换行
    Whitespace,
    Newline,

    // 注释
    Comment,

    // 关键字
    Program,
    Begin,
    End,
    Var,
    Const,
    Type,
    Function,
    Procedure,
    If,
    Then,
    Else,
    While,
    Do,
    For,
    To,
    Downto,
    Repeat,
    Until,
    Case,
    Of,
    With,
    Record,
    Array,
    Set,
    File,
    Packed,
    Nil,
    True,
    False,
    And,
    Or,
    Not,
    Div,
    Mod,
    In,

    // 标识符和字面量
    Identifier,
    IntegerLiteral,
    RealLiteral,
    StringLiteral,
    CharLiteral,

    // 运算符
    Plus,         // +
    Minus,        // -
    Multiply,     // *
    Divide,       // /
    Assign,       // :=
    Equal,        // =
    NotEqual,     // <>
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    Semicolon,    // ;
    Comma,        // ,
    Dot,          // .
    Colon,        // :
    Range,        // ..
    Caret,        // ^

    // 特殊
    Error,
    Eof,
}

impl SyntaxKind for PascalSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        true // Pascal doesn't have element types in this simple implementation
    }

    fn is_element_type(&self) -> bool {
        false // Pascal doesn't have element types in this simple implementation
    }
}
