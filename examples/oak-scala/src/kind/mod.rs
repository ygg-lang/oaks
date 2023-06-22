use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// 统一Scala 语法种类（包含节点与词法单元）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScalaSyntaxKind {
    // 节点种类
    SourceFile,

    // 基础词法种类
    Whitespace,
    Newline,
    Error,
    Eof,
    ErrorNode,

    // 标识符和字面量
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BooleanLiteral,

    // 关键字
    Abstract,
    Case,
    Catch,
    Class,
    Def,
    Do,
    Else,
    Extends,
    False,
    Final,
    Finally,
    For,
    ForSome,
    If,
    Implicit,
    Import,
    Lazy,
    Match,
    New,
    Null,
    Object,
    Override,
    Package,
    Private,
    Protected,
    Return,
    Sealed,
    Super,
    This,
    Throw,
    Trait,
    Try,
    True,
    Type,
    Val,
    Var,
    While,
    With,
    Yield,

    // 操作符
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %
    Eq,           // =
    EqEq,         // ==
    Ne,           // !=
    Lt,           // <
    Le,           // <=
    Gt,           // >
    Ge,           // >=
    LessEqual,    // <=
    GreaterEqual, // >=
    EqualEqual,   // ==
    NotEqual,     // !=
    And,          // &
    Or,           // |
    Xor,          // ^
    AndAnd,       // &&
    OrOr,         // ||
    Not,          // !
    Tilde,        // ~
    LShift,       // <<
    RShift,       // >>
    URShift,      // >>>
    PlusEq,       // +=
    MinusEq,      // -=
    StarEq,       // *=
    SlashEq,      // /=
    PercentEq,    // %=
    AndEq,        // &=
    OrEq,         // |=
    XorEq,        // ^=
    LShiftEq,     // <<=
    RShiftEq,     // >>=
    URShiftEq,    // >>>=
    Arrow,        // =>
    LeftArrow,    // <-
    Colon,        // :
    ColonColon,   // ::
    Semicolon,    // ;
    Dot,          // .
    Comma,        // ,
    Question,     // ?
    At,           // @
    Hash,         // #

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }

    // 注释
    LineComment,
    BlockComment,
    DocComment,
}

impl TokenType for ScalaSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment | Self::DocComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for ScalaSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error | Self::ErrorNode => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
