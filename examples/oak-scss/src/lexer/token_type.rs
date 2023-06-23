use oak_core::{TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum ScssTokenType {
    // Keywords
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
    True,
    False,
    Null,

    // Operators
    EqEq,
    Ne,
    Le,
    Ge,
    AndAnd,
    OrOr,
    Eq,
    Lt,
    Gt,
    And,
    Or,
    Xor,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,

    // Punctuation
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
    At,
    Dollar,

    // Literals and Identifiers
    Identifier,
    IntegerLiteral,
    StringLiteral,

    // Others
    Whitespace,
    Newline,
    Comment,
    Eof,
    Error,
}

impl TokenType for ScssTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Newline)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Import | Self::Include | Self::Mixin | Self::Function | Self::Return | Self::If | Self::Else | Self::For | Self::While | Self::Each | Self::In | Self::True | Self::False | Self::Null => UniversalTokenRole::Keyword,

            Self::EqEq | Self::Ne | Self::Le | Self::Ge | Self::AndAnd | Self::OrOr | Self::Eq | Self::Lt | Self::Gt | Self::And | Self::Or | Self::Xor | Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Percent | Self::Bang => {
                UniversalTokenRole::Operator
            }

            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Colon | Self::Comma | Self::Dot | Self::Hash | Self::At | Self::Dollar => {
                UniversalTokenRole::Punctuation
            }

            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::StringLiteral => UniversalTokenRole::Literal,

            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
        }
    }
}
