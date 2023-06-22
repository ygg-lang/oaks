use core::fmt;
use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// Ruby 令牌种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum RubySyntaxKind {
    // 基础标识符和字面量
    Identifier,
    GlobalVariable,
    InstanceVariable,
    ClassVariable,
    Constant,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    Literal,
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
    Equal,
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

    // 节点种类
    Root,
    BinaryExpression,
    UnaryExpression,
    LiteralExpression,
    ParenExpression,
    ParenthesizedExpression,
    MethodDefinition,
    ClassDefinition,
    ModuleDefinition,
    IfStatement,
    WhileStatement,
    ReturnStatement,
    IfExpression,
    CallExpression,
    MemberAccess,
    ParameterList,
    ArgumentList,
    Error,
}

impl RubySyntaxKind {
    pub fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::If
                | Self::Unless
                | Self::Elsif
                | Self::Else
                | Self::Case
                | Self::When
                | Self::Then
                | Self::For
                | Self::While
                | Self::Until
                | Self::Break
                | Self::Next
                | Self::Redo
                | Self::Retry
                | Self::Return
                | Self::Yield
                | Self::Def
                | Self::Class
                | Self::Module
                | Self::End
                | Self::Lambda
                | Self::Proc
                | Self::Begin
                | Self::Rescue
                | Self::Ensure
                | Self::Raise
                | Self::Require
                | Self::Load
                | Self::Include
                | Self::Extend
                | Self::Prepend
                | Self::And
                | Self::Or
                | Self::Not
                | Self::In
                | Self::True
                | Self::False
                | Self::Nil
                | Self::Super
                | Self::Self_
                | Self::Alias
                | Self::Undef
                | Self::Defined
                | Self::Do
        )
    }
}

impl TokenType for RubySyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier | Self::GlobalVariable | Self::InstanceVariable | Self::ClassVariable | Self::Constant => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::Literal | Self::Symbol | Self::RegexLiteral => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Multiply
            | Self::Divide
            | Self::Modulo
            | Self::Power
            | Self::EqualEqual
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::EqualEqualEqual
            | Self::Spaceship
            | Self::Assign
            | Self::PlusAssign
            | Self::MinusAssign
            | Self::MultiplyAssign
            | Self::DivideAssign
            | Self::ModuloAssign
            | Self::PowerAssign
            | Self::BitAnd
            | Self::BitOr
            | Self::Xor
            | Self::LogicalNot
            | Self::Tilde
            | Self::LeftShift
            | Self::RightShift
            | Self::AndAssign
            | Self::OrAssign
            | Self::XorAssign
            | Self::LeftShiftAssign
            | Self::RightShiftAssign
            | Self::AndAnd
            | Self::OrOr
            | Self::OrOrAssign
            | Self::AndAndAssign
            | Self::Question
            | Self::DotDot
            | Self::DotDotDot
            | Self::Match
            | Self::NotMatch => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Comma | Self::Colon | Self::Semicolon | Self::Dot | Self::DoubleColon | Self::At | Self::Dollar => {
                UniversalTokenRole::Punctuation
            }
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for RubySyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::BinaryExpression | Self::UnaryExpression | Self::LiteralExpression | Self::ParenExpression => UniversalElementRole::Expression,
            _ => UniversalElementRole::None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Invalid)
    }
}

impl fmt::Display for RubySyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Identifier => "Identifier",
            Self::GlobalVariable => "GlobalVariable",
            Self::InstanceVariable => "InstanceVariable",
            Self::ClassVariable => "ClassVariable",
            Self::Constant => "Constant",
            Self::IntegerLiteral => "IntegerLiteral",
            Self::FloatLiteral => "FloatLiteral",
            Self::StringLiteral => "StringLiteral",
            Self::Literal => "Literal",
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
            Self::Root => "Root",
            Self::BinaryExpression => "BinaryExpression",
            Self::UnaryExpression => "UnaryExpression",
            Self::LiteralExpression => "LiteralExpression",
            Self::ParenExpression => "ParenExpression",
            Self::ParenthesizedExpression => "ParenthesizedExpression",
            Self::MethodDefinition => "MethodDefinition",
            Self::ClassDefinition => "ClassDefinition",
            Self::ModuleDefinition => "ModuleDefinition",
            Self::IfStatement => "IfStatement",
            Self::WhileStatement => "WhileStatement",
            Self::ReturnStatement => "ReturnStatement",
            Self::IfExpression => "IfExpression",
            Self::CallExpression => "CallExpression",
            Self::MemberAccess => "MemberAccess",
            Self::ParameterList => "ParameterList",
            Self::ArgumentList => "ArgumentList",
            Self::Error => "Error",
            Self::Equal => "Equal",
        };
        write!(f, "{}", name)
    }
}
