use oak_core::{TokenType as _, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// APL 词法单元类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AplTokenType {
    Whitespace,
    Newline,
    Comment, // ⍝

    StringLiteral,
    NumberLiteral,
    Identifier,

    // Core Symbols
    LeftArrow,  // ←
    RightArrow, // →
    Diamond,    // ⋄
    Quad,       // ⎕
    QuoteQuad,  // ⍞
    Rho,        // ⍴
    Iota,       // ⍳
    Epsilon,    // ∊
    UpArrow,    // ↑
    DownArrow,  // ↓
    Del,        // ∇
    Delta,      // ∆
    Alpha,      // ⍺
    Omega,      // ⍵
    Zilde,      // ⍬

    // Operators/Functions
    Plus,         // +
    Minus,        // -
    Times,        // ×
    Divide,       // ÷
    Star,         // *
    Log,          // ⍟
    Circle,       // ○
    Or,           // ∨
    And,          // ∧
    Not,          // ∼
    Nor,          // ⍱
    Nand,         // ⍲
    Equal,        // =
    NotEqual,     // ≠
    LessThan,     // <
    LessEqual,    // ≤
    GreaterEqual, // ≥
    GreaterThan,  // >
    UpStile,      // ⌈
    DownStile,    // ⌊
    Bar,          // |
    Tilde,        // ∼
    Question,     // ?
    Factorial,    // !

    // Operators (Higher Order)
    Slash,        // /
    Backslash,    // \
    SlashBar,     // ⌿
    BackslashBar, // ⍀
    Dot,          // .
    Jot,          // ∘
    Diaeresis,    // ¨
    Power,        // ⍣
    Rank,         // ⍤
    Tally,        // ≢

    // Structural
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Semicolon,    // ;

    Eof,
    Error,
}

impl AplTokenType {
    /// 是否为标识符
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier | Self::Alpha | Self::Omega)
    }

    /// 是否为字面量
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::StringLiteral | Self::NumberLiteral | Self::Zilde)
    }
}

pub type TokenType = AplTokenType;

impl oak_core::TokenType for AplTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            Self::Identifier | Self::Alpha | Self::Omega => Name,
            Self::StringLiteral | Self::NumberLiteral | Self::Zilde => Literal,
            Self::Comment => Comment,
            Self::Whitespace | Self::Newline => Whitespace,
            Self::Error => Error,
            Self::Eof => Eof,

            // Core symbols and operators
            Self::LeftArrow
            | Self::RightArrow
            | Self::Plus
            | Self::Minus
            | Self::Times
            | Self::Divide
            | Self::Star
            | Self::Log
            | Self::Circle
            | Self::Or
            | Self::And
            | Self::Not
            | Self::Nor
            | Self::Nand
            | Self::Equal
            | Self::NotEqual
            | Self::LessThan
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::GreaterThan
            | Self::UpStile
            | Self::DownStile
            | Self::Bar
            | Self::Tilde
            | Self::Question
            | Self::Factorial
            | Self::Slash
            | Self::Backslash
            | Self::SlashBar
            | Self::BackslashBar
            | Self::Dot
            | Self::Jot
            | Self::Diaeresis
            | Self::Power
            | Self::Rank
            | Self::Tally
            | Self::Rho
            | Self::Iota
            | Self::Epsilon
            | Self::UpArrow
            | Self::DownArrow => Operator,

            Self::Diamond | Self::Quad | Self::QuoteQuad | Self::Del | Self::Delta | Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Semicolon => Punctuation,
        }
    }
}
