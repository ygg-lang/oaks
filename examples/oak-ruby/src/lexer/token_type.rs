use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

pub type RubyToken = Token<RubyTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum RubyTokenType {
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

    Whitespace,
    Newline,
    Comment,
    Eof,
    Invalid,
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
    Equal,
}

impl RubyTokenType {
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

impl fmt::Display for RubyTokenType {
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

impl TokenType for RubyTokenType {
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
