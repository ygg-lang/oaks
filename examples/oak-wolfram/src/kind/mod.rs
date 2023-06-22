use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WolframSyntaxKind {
    Root,

    // 基础 tokens
    Whitespace,
    Newline,

    // 标识符和字面量
    Identifier,
    Integer,
    Real,
    String,

    // 关键字
    If,
    Then,
    Else,
    While,
    For,
    Do,
    Function,
    Module,
    Block,
    With,
    Table,
    Map,
    Apply,
    Select,
    Cases,
    Rule,
    RuleDelayed,
    Set,
    SetDelayed,
    Unset,
    Clear,
    ClearAll,
    Return,
    Break,
    Continue,
    True,
    False,
    Null,
    Export,
    Import,

    // 运算符
    Plus,         // +
    Minus,        // -
    Times,        // *
    Divide,       // /
    Power,        // ^
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
    And,          // &&
    Or,           // ||
    Not,          // !

    // 赋值运算符
    Assign,       // =
    AddTo,        // +=
    SubtractFrom, // -=
    TimesBy,      // *=
    DivideBy,     // /=

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Semicolon,    // ;
    Colon,        // :
    Dot,          // .

    // 特殊符号
    Arrow,            // ->
    DoubleArrow,      // =>
    Question,         // ?
    Underscore,       // _
    DoubleUnderscore, // __
    TripleUnderscore, // ___
    Slot,             // #
    SlotSequence,     // ##

    // 注释
    Comment,

    // 文本
    Text,

    // 错误处理
    Error,

    // EOF
    Eof,
}

impl fmt::Display for WolframSyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TokenType for WolframSyntaxKind {
    const END_OF_STREAM: Self = WolframSyntaxKind::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        UniversalTokenRole::None
    }

    fn is_ignored(&self) -> bool {
        matches!(self, WolframSyntaxKind::Whitespace | WolframSyntaxKind::Newline | WolframSyntaxKind::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, WolframSyntaxKind::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, WolframSyntaxKind::Whitespace | WolframSyntaxKind::Newline)
    }
}

impl ElementType for WolframSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            WolframSyntaxKind::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
