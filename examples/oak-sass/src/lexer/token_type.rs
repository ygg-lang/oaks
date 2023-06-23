use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type SassToken = Token<SassTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum SassTokenType {
    // 基础
    Whitespace,
    LineComment,
    BlockComment,
    Eof,
    Error,

    // 字面量
    StringLiteral,
    CharLiteral,
    NumberLiteral,
    FloatLiteral,
    ColorLiteral,
    Variable,
    Identifier,

    // 关键字
    Import,
    Include,
    Extend,
    Mixin,
    Function,
    Return,
    If,
    Else,
    ElseIf,
    For,
    Each,
    While,
    Default,
    Important,
    Optional,
    Global,
    And,
    Or,
    Not,

    // 运算符
    EqEq,
    Ne,
    Le,
    Ge,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    Lt,
    Gt,

    // 标点符号
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Colon,
    Comma,
    Dot,
    Hash,
    Dollar,
    At,
    Ampersand,
    Exclamation,
    Question,
    Tilde,
}

impl TokenType for SassTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::LineComment | Self::BlockComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral | Self::CharLiteral | Self::NumberLiteral | Self::FloatLiteral | Self::ColorLiteral => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            _ => UniversalTokenRole::None,
        }
    }
}

impl SassTokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Import
                | Self::Include
                | Self::Extend
                | Self::Mixin
                | Self::Function
                | Self::Return
                | Self::If
                | Self::Else
                | Self::ElseIf
                | Self::For
                | Self::Each
                | Self::While
                | Self::Default
                | Self::Important
                | Self::Optional
                | Self::Global
                | Self::And
                | Self::Or
                | Self::Not
        )
    }
}
