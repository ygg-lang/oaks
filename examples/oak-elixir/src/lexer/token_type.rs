use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ElixirTokenType {
    Root,
    Whitespace,
    Newline,
    Comment,
    Identifier,
    Atom,
    Variable,
    Number,
    Float,
    String,
    Character,
    Sigil,

    After,
    And,
    Case,
    Catch,
    Cond,
    Def,
    Defp,
    Defmodule,
    Defstruct,
    Defprotocol,
    Defimpl,
    Defmacro,
    Defmacrop,
    Do,
    Else,
    Elsif,
    End,
    False,
    Fn,
    If,
    In,
    Not,
    Or,
    Receive,
    Rescue,
    True,
    Try,
    Unless,
    When,
    With,

    // Operators
    Plus,
    Minus,
    Mul,
    Div,
    Dot,
    Comma,
    Semicolon,
    Colon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Arrow,
    Pipe,

    // More operators
    Eq,
    EqEq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    AndAnd,
    OrOr,
    Bang,
    Concat,
    PlusPlus,
    MinusMinus,
    Pipeline,
    LeftDoubleBracket,
    RightDoubleBracket,
    At,
    Percent,

    Error,
}

pub type ElixirToken = Token<ElixirTokenType>;

impl ElixirTokenType {
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::After
                | Self::And
                | Self::Case
                | Self::Catch
                | Self::Cond
                | Self::Def
                | Self::Defp
                | Self::Defmodule
                | Self::Defstruct
                | Self::Defprotocol
                | Self::Defimpl
                | Self::Defmacro
                | Self::Defmacrop
                | Self::Do
                | Self::Else
                | Self::Elsif
                | Self::End
                | Self::False
                | Self::Fn
                | Self::If
                | Self::In
                | Self::Not
                | Self::Or
                | Self::Receive
                | Self::Rescue
                | Self::True
                | Self::Try
                | Self::Unless
                | Self::When
                | Self::With
        )
    }
}

impl TokenType for ElixirTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Newline)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number | Self::Float => UniversalTokenRole::Literal,
            Self::String | Self::Character => UniversalTokenRole::Literal,
            Self::Plus
            | Self::Minus
            | Self::Mul
            | Self::Div
            | Self::Dot
            | Self::Eq
            | Self::EqEq
            | Self::Ne
            | Self::Lt
            | Self::Le
            | Self::Gt
            | Self::Ge
            | Self::AndAnd
            | Self::OrOr
            | Self::Bang
            | Self::Concat
            | Self::PlusPlus
            | Self::MinusMinus
            | Self::Pipeline
            | Self::Arrow
            | Self::Pipe => UniversalTokenRole::Operator,
            Self::Comma | Self::Semicolon | Self::Colon | Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::LeftDoubleBracket | Self::RightDoubleBracket => {
                UniversalTokenRole::Punctuation
            }
            Self::Error => UniversalTokenRole::Error,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            _ => UniversalTokenRole::None,
        }
    }
}
