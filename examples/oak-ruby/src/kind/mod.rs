use core::fmt;
use oak_core::{SyntaxKind, Token};

/// Ruby 令牌
pub type RubyToken = Token<RubySyntaxKind>;

/// Ruby 令牌种类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RubySyntaxKind {
    // 基础标识符和字面量
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    Symbol,
    RegexLiteral,

    // 关键字
    If,
    Unless,
    Elsif,
    Else,
    Case,
    When,
    Then,
    For,
    While,
    Until,
    Break,
    Next,
    Redo,
    Retry,
    Return,
    Yield,
    Def,
    Class,
    Module,
    End,
    Lambda,
    Proc,
    Begin,
    Rescue,
    Ensure,
    Raise,
    Require,
    Load,
    Include,
    Extend,
    Prepend,
    And,
    Or,
    Not,
    In,
    True,
    False,
    Nil,
    Super,
    Self_,
    Alias,
    Undef,
    Defined,
    Do,

    // 操作符
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    EqualEqualEqual,
    Spaceship,
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    PowerAssign,
    BitAnd,
    BitOr,
    Xor,
    LogicalNot,
    Tilde,
    LeftShift,
    RightShift,
    AndAssign,
    OrAssign,
    XorAssign,
    LeftShiftAssign,
    RightShiftAssign,
    AndAnd,
    OrOr,
    OrOrAssign,
    AndAndAssign,
    Question,
    DotDot,
    DotDotDot,
    Match,
    NotMatch,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Semicolon,
    Dot,
    DoubleColon,
    At,
    Dollar,

    // 空白和注释
    Whitespace,
    Newline,
    Comment,

    // 特殊
    Eof,
    Invalid,
}

impl fmt::Display for RubySyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Identifier => "Identifier",
            Self::IntegerLiteral => "IntegerLiteral",
            Self::FloatLiteral => "FloatLiteral",
            Self::StringLiteral => "StringLiteral",
            Self::Symbol => "Symbol",
            Self::RegexLiteral => "RegexLiteral",

            Self::If => "If",
            Self::Unless => "Unless",
            Self::Elsif => "Elsif",
            Self::Else => "Else",
            Self::Case => "Case",
            Self::When => "When",
            Self::Then => "Then",
            Self::For => "For",
            Self::While => "While",
            Self::Until => "Until",
            Self::Break => "Break",
            Self::Next => "Next",
            Self::Redo => "Redo",
            Self::Retry => "Retry",
            Self::Return => "Return",
            Self::Yield => "Yield",
            Self::Def => "Def",
            Self::Class => "Class",
            Self::Module => "Module",
            Self::End => "End",
            Self::Lambda => "Lambda",
            Self::Proc => "Proc",
            Self::Begin => "Begin",
            Self::Rescue => "Rescue",
            Self::Ensure => "Ensure",
            Self::Raise => "Raise",
            Self::Require => "Require",
            Self::Load => "Load",
            Self::Include => "Include",
            Self::Extend => "Extend",
            Self::Prepend => "Prepend",
            Self::And => "And",
            Self::Or => "Or",
            Self::Not => "Not",
            Self::In => "In",
            Self::True => "True",
            Self::False => "False",
            Self::Nil => "Nil",
            Self::Super => "Super",
            Self::Self_ => "Self",
            Self::Alias => "Alias",
            Self::Undef => "Undef",
            Self::Defined => "Defined",
            Self::Do => "Do",

            Self::Plus => "Plus",
            Self::Minus => "Minus",
            Self::Multiply => "Multiply",
            Self::Divide => "Divide",
            Self::Modulo => "Modulo",
            Self::Power => "Power",
            Self::EqualEqual => "EqualEqual",
            Self::NotEqual => "NotEqual",
            Self::Less => "Less",
            Self::Greater => "Greater",
            Self::LessEqual => "LessEqual",
            Self::GreaterEqual => "GreaterEqual",
            Self::EqualEqualEqual => "EqualEqualEqual",
            Self::Spaceship => "Spaceship",
            Self::Assign => "Assign",
            Self::PlusAssign => "PlusAssign",
            Self::MinusAssign => "MinusAssign",
            Self::MultiplyAssign => "MultiplyAssign",
            Self::DivideAssign => "DivideAssign",
            Self::ModuloAssign => "ModuloAssign",
            Self::PowerAssign => "PowerAssign",
            Self::BitAnd => "BitAnd",
            Self::BitOr => "BitOr",
            Self::Xor => "Xor",
            Self::LogicalNot => "LogicalNot",
            Self::Tilde => "Tilde",
            Self::LeftShift => "LeftShift",
            Self::RightShift => "RightShift",
            Self::AndAssign => "AndAssign",
            Self::OrAssign => "OrAssign",
            Self::XorAssign => "XorAssign",
            Self::LeftShiftAssign => "LeftShiftAssign",
            Self::RightShiftAssign => "RightShiftAssign",
            Self::AndAnd => "AndAnd",
            Self::OrOr => "OrOr",
            Self::OrOrAssign => "OrOrAssign",
            Self::AndAndAssign => "AndAndAssign",
            Self::Question => "Question",
            Self::DotDot => "DotDot",
            Self::DotDotDot => "DotDotDot",
            Self::Match => "Match",
            Self::NotMatch => "NotMatch",

            Self::LeftParen => "LeftParen",
            Self::RightParen => "RightParen",
            Self::LeftBracket => "LeftBracket",
            Self::RightBracket => "RightBracket",
            Self::LeftBrace => "LeftBrace",
            Self::RightBrace => "RightBrace",
            Self::Comma => "Comma",
            Self::Colon => "Colon",
            Self::Semicolon => "Semicolon",
            Self::Dot => "Dot",
            Self::DoubleColon => "DoubleColon",
            Self::At => "At",
            Self::Dollar => "Dollar",

            Self::Whitespace => "Whitespace",
            Self::Newline => "Newline",
            Self::Comment => "Comment",
            Self::Eof => "Eof",
            Self::Invalid => "Invalid",
        };
        write!(f, "{}", name)
    }
}

impl SyntaxKind for RubySyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        true // Ruby 中所有类型都是 token 类型
    }

    fn is_element_type(&self) -> bool {
        false // Ruby 中没有元素类型
    }
}
