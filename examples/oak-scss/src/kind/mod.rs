use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// 统一 SCSS 语法种类（包含节点与词法）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScssSyntaxKind {
    // 节点种类
    SourceFile,
    ErrorNode,
    Selector,
    Property,
    Variable,
    Ruleset,
    Declaration,

    // 基础词法种类
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,

    // 标识符和字面量
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BooleanLiteral,

    // SCSS 关键字
    Import,
    Include,
    Mixin,
    Function,
    Return,
    If,
    Else,
    For,
    While,
    Each,
    In,
    From,
    To,
    Through,
    Default,
    Important,
    Optional,
    Global,
    Null,
    True,
    False,

    // 临时保留的 Scala 关键字（为了兼容现有 lexer）
    Abstract,
    Case,
    Catch,
    Class,
    Def,
    Do,
    Extends,
    Final,
    Finally,
    ForSome,
    Implicit,
    Lazy,
    Match,
    New,
    Object,
    Override,
    Package,
    Private,
    Protected,
    Sealed,
    Super,
    This,
    Throw,
    Trait,
    Try,
    Type,
    Val,
    Var,
    With,
    Yield,

    // 操作符
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Eq,         // =
    EqEq,       // ==
    Ne,         // !=
    Lt,         // <
    Le,         // <=
    Gt,         // >
    Ge,         // >=
    And,        // &
    Or,         // |
    Xor,        // ^
    AndAnd,     // &&
    OrOr,       // ||
    Not,        // !
    Bang,       // ! (alternative name)
    Tilde,      // ~
    LShift,     // <<
    RShift,     // >>
    URShift,    // >>>
    PlusEq,     // +=
    MinusEq,    // -=
    StarEq,     // *=
    SlashEq,    // /=
    PercentEq,  // %=
    AndEq,      // &=
    OrEq,       // |=
    XorEq,      // ^=
    LShiftEq,   // <<=
    RShiftEq,   // >>=
    URShiftEq,  // >>>=
    Arrow,      // =>
    LeftArrow,  // <-
    Colon,      // :
    ColonColon, // ::
    Semicolon,  // ;
    Dot,        // .
    Comma,      // ,
    Question,   // ?
    At,         // @
    Hash,       // #
    Dollar,     // $

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

impl TokenType for ScssSyntaxKind {
    type Role = UniversalTokenRole;

    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment | Self::LineComment | Self::BlockComment | Self::DocComment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::None,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::BooleanLiteral | Self::True | Self::False | Self::Null => UniversalTokenRole::Literal,
            Self::Import
            | Self::Include
            | Self::Mixin
            | Self::Function
            | Self::Return
            | Self::If
            | Self::Else
            | Self::For
            | Self::While
            | Self::Each
            | Self::In
            | Self::From
            | Self::To
            | Self::Through
            | Self::Default
            | Self::Important
            | Self::Optional
            | Self::Global => UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Eq
            | Self::EqEq
            | Self::Ne
            | Self::Lt
            | Self::Le
            | Self::Gt
            | Self::Ge
            | Self::And
            | Self::Or
            | Self::Xor
            | Self::AndAnd
            | Self::OrOr
            | Self::Not
            | Self::Bang
            | Self::Tilde
            | Self::LShift
            | Self::RShift
            | Self::URShift
            | Self::PlusEq
            | Self::MinusEq
            | Self::StarEq
            | Self::SlashEq
            | Self::PercentEq
            | Self::AndEq
            | Self::OrEq
            | Self::XorEq
            | Self::LShiftEq
            | Self::RShiftEq
            | Self::URShiftEq
            | Self::Arrow
            | Self::LeftArrow => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for ScssSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::ErrorNode => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
