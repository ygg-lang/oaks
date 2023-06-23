use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

pub type WolframToken = Token<WolframTokenType>;

impl fmt::Display for WolframTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TokenType for WolframTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        self.role()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WolframTokenType {
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

    // 函数式运算符
    At,                 // @
    SlashSlash,         // //
    MapOperator,        // /@
    ApplyOperator,      // @@
    ApplyLevelOperator, // @@@
    MapAllOperator,     // //@
    Ampersand,          // &
    AtStar,             // @*
    StarSlash,          // /*
    StringJoin,         // <>
    RuleDelayedOp,      // :>

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
    Factorial,        // ! (postfix)

    // 注释
    Comment,

    // 文本
    Text,

    // 错误处理
    Error,

    // EOF
    Eof,
}

impl WolframTokenType {
    pub fn role(&self) -> UniversalTokenRole {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace, // Map Newline to Whitespace as it's not in UniversalTokenRole
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name, // Use Name instead of Identifier
            Self::Integer | Self::Real => UniversalTokenRole::Literal,
            Self::String => UniversalTokenRole::Literal, // Map String to Literal as String is not in UniversalTokenRole
            Self::LeftParen | Self::LeftBracket | Self::LeftBrace => UniversalTokenRole::Punctuation,
            Self::RightParen | Self::RightBracket | Self::RightBrace => UniversalTokenRole::Punctuation,
            Self::Comma | Self::Semicolon | Self::Colon | Self::Dot => UniversalTokenRole::Punctuation,
            Self::Plus
            | Self::Minus
            | Self::Times
            | Self::Divide
            | Self::Power
            | Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::And
            | Self::Or
            | Self::Not
            | Self::At
            | Self::SlashSlash
            | Self::MapOperator
            | Self::ApplyOperator
            | Self::ApplyLevelOperator
            | Self::MapAllOperator
            | Self::Ampersand
            | Self::AtStar
            | Self::StarSlash
            | Self::StringJoin
            | Self::RuleDelayedOp
            | Self::Assign
            | Self::AddTo
            | Self::SubtractFrom
            | Self::TimesBy
            | Self::DivideBy
            | Self::Arrow
            | Self::DoubleArrow
            | Self::Question
            | Self::Underscore
            | Self::DoubleUnderscore
            | Self::TripleUnderscore
            | Self::Slot
            | Self::SlotSequence
            | Self::Factorial => UniversalTokenRole::Operator,
            Self::If
            | Self::Then
            | Self::Else
            | Self::While
            | Self::For
            | Self::Do
            | Self::Function
            | Self::Module
            | Self::Block
            | Self::With
            | Self::Table
            | Self::Map
            | Self::Apply
            | Self::Select
            | Self::Cases
            | Self::Rule
            | Self::RuleDelayed
            | Self::Set
            | Self::SetDelayed
            | Self::Unset
            | Self::Clear
            | Self::ClearAll
            | Self::Return
            | Self::Break
            | Self::Continue
            | Self::True
            | Self::False
            | Self::Null
            | Self::Export
            | Self::Import => UniversalTokenRole::Keyword,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}
