use oak_core::SyntaxKind;
use serde::{Deserialize, Serialize};

/// Go 语法节点类型
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GoLangSyntaxKind {
    // 字面
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    RuneLiteral,
    BoolLiteral,

    // 标识
    Identifier,

    // 关键
    Break,
    Case,
    Chan,
    Const,
    Continue,
    Default,
    Defer,
    Else,
    Fallthrough,
    For,
    Func,
    Go,
    Goto,
    If,
    Import,
    Interface,
    Map,
    Package,
    Range,
    Return,
    Select,
    Struct,
    Switch,
    Type,
    Var,

    // 内置类型
    Bool,
    Byte,
    Complex64,
    Complex128,
    ErrorType,
    Float32,
    Float64,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Rune,
    String,
    Uint,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uintptr,

    // 特殊字面
    NilLiteral,
    NumberLiteral,
    CharLiteral,

    // 操作
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    Ampersand,      // &
    Pipe,           // |
    Caret,          // ^
    LeftShift,      // <<
    RightShift,     // >>
    AmpersandCaret, // &^

    PlusAssign,           // +=
    MinusAssign,          // -=
    StarAssign,           // *=
    SlashAssign,          // /=
    PercentAssign,        // %=
    AmpersandAssign,      // &=
    PipeAssign,           // |=
    CaretAssign,          // ^=
    XorAssign,            // ^= (别名)
    LeftShiftAssign,      // <<=
    RightShiftAssign,     // >>=
    AmpersandCaretAssign, // &^=
    AndAssign,            // &=
    OrAssign,             // |=
    AndNotAssign,         // &^=
    AndNot,               // &^

    LogicalAnd, // &&
    LogicalOr,  // ||
    And,        // && (别名)
    Or,         // || (别名)
    Arrow,      // <-
    LeftArrow,  // <- (别名)
    Increment,  // ++
    Decrement,  // --

    Equal,      // ==
    Less,       // <
    Greater,    // >
    Assign,     // =
    LogicalNot, // !
    Not,        // ! (别名)

    NotEqual,     // !=
    LessEqual,    // <=
    GreaterEqual, // >=
    ColonAssign,  // :=
    Define,       // := (别名)
    Ellipsis,     // ...

    // 分隔
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Period,       // .
    Dot,          // . (别名)
    Semicolon,    // ;
    Colon,        // :

    // 空白和注
    Whitespace,
    Comment,

    // 特殊
    Eof,
    Error,
}

impl SyntaxKind for GoLangSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        true
    }

    fn is_element_type(&self) -> bool {
        false
    }
}

use core::fmt;

impl fmt::Debug for GoLangSyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntLiteral => write!(f, "IntLiteral"),
            Self::FloatLiteral => write!(f, "FloatLiteral"),
            Self::StringLiteral => write!(f, "StringLiteral"),
            Self::RuneLiteral => write!(f, "RuneLiteral"),
            Self::BoolLiteral => write!(f, "BoolLiteral"),
            Self::NilLiteral => write!(f, "NilLiteral"),
            Self::Identifier => write!(f, "Identifier"),
            Self::Package => write!(f, "Package"),
            Self::Import => write!(f, "Import"),
            Self::Func => write!(f, "Func"),
            Self::Var => write!(f, "Var"),
            Self::Const => write!(f, "Const"),
            Self::Type => write!(f, "Type"),
            Self::Struct => write!(f, "Struct"),
            Self::Interface => write!(f, "Interface"),
            Self::Map => write!(f, "Map"),
            Self::Chan => write!(f, "Chan"),
            Self::If => write!(f, "If"),
            Self::Else => write!(f, "Else"),
            Self::For => write!(f, "For"),
            Self::Range => write!(f, "Range"),
            Self::Switch => write!(f, "Switch"),
            Self::Case => write!(f, "Case"),
            Self::Default => write!(f, "Default"),
            Self::Break => write!(f, "Break"),
            Self::Continue => write!(f, "Continue"),
            Self::Return => write!(f, "Return"),
            Self::Go => write!(f, "Go"),
            Self::Defer => write!(f, "Defer"),
            Self::Select => write!(f, "Select"),
            Self::Fallthrough => write!(f, "Fallthrough"),
            Self::Goto => write!(f, "Goto"),
            Self::LeftParen => write!(f, "LeftParen"),
            Self::RightParen => write!(f, "RightParen"),
            Self::LeftBrace => write!(f, "LeftBrace"),
            Self::RightBrace => write!(f, "RightBrace"),
            Self::LeftBracket => write!(f, "LeftBracket"),
            Self::RightBracket => write!(f, "RightBracket"),
            Self::Plus => write!(f, "Plus"),
            Self::Minus => write!(f, "Minus"),
            Self::Star => write!(f, "Star"),
            Self::Slash => write!(f, "Slash"),
            Self::Percent => write!(f, "Percent"),
            Self::Ampersand => write!(f, "Ampersand"),
            Self::Pipe => write!(f, "Pipe"),
            Self::Caret => write!(f, "Caret"),
            Self::LeftShift => write!(f, "LeftShift"),
            Self::RightShift => write!(f, "RightShift"),
            Self::AndNot => write!(f, "AndNot"),
            Self::PlusAssign => write!(f, "PlusAssign"),
            Self::MinusAssign => write!(f, "MinusAssign"),
            Self::StarAssign => write!(f, "StarAssign"),
            Self::SlashAssign => write!(f, "SlashAssign"),
            Self::PercentAssign => write!(f, "PercentAssign"),
            Self::AmpersandAssign => write!(f, "AmpersandAssign"),
            Self::PipeAssign => write!(f, "PipeAssign"),
            Self::CaretAssign => write!(f, "CaretAssign"),
            Self::LeftShiftAssign => write!(f, "LeftShiftAssign"),
            Self::RightShiftAssign => write!(f, "RightShiftAssign"),
            Self::XorAssign => write!(f, "XorAssign"),
            Self::AndAssign => write!(f, "AndAssign"),
            Self::OrAssign => write!(f, "OrAssign"),
            Self::AndNotAssign => write!(f, "AndNotAssign"),
            Self::LogicalAnd => write!(f, "LogicalAnd"),
            Self::LogicalOr => write!(f, "LogicalOr"),
            Self::And => write!(f, "And"),
            Self::Or => write!(f, "Or"),
            Self::Arrow => write!(f, "Arrow"),
            Self::LeftArrow => write!(f, "LeftArrow"),
            Self::Increment => write!(f, "Increment"),
            Self::Decrement => write!(f, "Decrement"),
            Self::Equal => write!(f, "Equal"),
            Self::Less => write!(f, "Less"),
            Self::Greater => write!(f, "Greater"),
            Self::Assign => write!(f, "Assign"),
            Self::LogicalNot => write!(f, "LogicalNot"),
            Self::Not => write!(f, "Not"),
            Self::NotEqual => write!(f, "NotEqual"),
            Self::LessEqual => write!(f, "LessEqual"),
            Self::GreaterEqual => write!(f, "GreaterEqual"),
            Self::ColonAssign => write!(f, "ColonAssign"),
            Self::Define => write!(f, "Define"),
            Self::Comma => write!(f, "Comma"),
            Self::Period => write!(f, "Period"),
            Self::Dot => write!(f, "Dot"),
            Self::Semicolon => write!(f, "Semicolon"),
            Self::Colon => write!(f, "Colon"),
            Self::Ellipsis => write!(f, "Ellipsis"),
            Self::AmpersandCaret => write!(f, "AmpersandCaret"),
            Self::AmpersandCaretAssign => write!(f, "AmpersandCaretAssign"),
            Self::Bool => write!(f, "Bool"),
            Self::Byte => write!(f, "Byte"),
            Self::Complex64 => write!(f, "Complex64"),
            Self::Complex128 => write!(f, "Complex128"),
            Self::ErrorType => write!(f, "ErrorType"),
            Self::Float32 => write!(f, "Float32"),
            Self::Float64 => write!(f, "Float64"),
            Self::Int => write!(f, "Int"),
            Self::Int8 => write!(f, "Int8"),
            Self::Int16 => write!(f, "Int16"),
            Self::Int32 => write!(f, "Int32"),
            Self::Int64 => write!(f, "Int64"),
            Self::Rune => write!(f, "Rune"),
            Self::String => write!(f, "String"),
            Self::Uint => write!(f, "Uint"),
            Self::Uint8 => write!(f, "Uint8"),
            Self::Uint16 => write!(f, "Uint16"),
            Self::Uint32 => write!(f, "Uint32"),
            Self::Uint64 => write!(f, "Uint64"),
            Self::Uintptr => write!(f, "Uintptr"),
            Self::NumberLiteral => write!(f, "NumberLiteral"),
            Self::CharLiteral => write!(f, "CharLiteral"),
            Self::Whitespace => write!(f, "Whitespace"),
            Self::Comment => write!(f, "Comment"),
            Self::Eof => write!(f, "Eof"),
            Self::Error => write!(f, "Error"),
        }
    }
}
