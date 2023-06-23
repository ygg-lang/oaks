use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type RToken = Token<RTokenType>;

impl RTokenType {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

impl TokenType for RTokenType {
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

use crate::language::RLanguage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum RTokenType {
    // Whitespace and newlines
    Whitespace,
    Newline,

    // Comments
    Comment,

    // Literals
    StringLiteral,
    IntegerLiteral,
    FloatLiteral,
    BooleanLiteral,
    NullLiteral,
    Inf,
    NaN,
    NA,
    NaInteger,
    NaReal,
    NaComplex,
    NaCharacter,

    // Identifiers
    Identifier,

    // Keywords
    If,
    Else,
    For,
    In,
    While,
    Repeat,
    Next,
    Break,
    Function,
    Return,
    True,
    False,
    Null,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    AndAnd,
    OrOr,
    Tilde,
    LeftArrow,
    RightArrow,
    DoubleLeftArrow,
    DoubleRightArrow,
    Pipe,
    Operator,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    DoubleColon,
    TripleColon,
    Dot,
    Dollar,
    At,
    Question,

    // Root node
    Root,

    // Expressions
    Assignment,
    BinaryExpression,
    UnaryExpression,
    LiteralExpression,
    IdentifierExpression,
    CallExpression,
    GroupingExpression,
    BlockExpression,
    IfExpression,
    WhileExpression,
    ForExpression,
    RepeatExpression,
    FunctionDefinition,
    IndexExpression,
    MemberExpression,
    ArgumentList,
    ParameterList,

    // 错误和结束
    Error,
    Eof,
}

pub type RNode<'a> = oak_core::tree::RedNode<'a, RLanguage>;
