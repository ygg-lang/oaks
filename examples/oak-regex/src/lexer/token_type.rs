use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub type RegexToken = Token<RegexTokenType>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum RegexTokenType {
    // Special Kinds
    TOMBSTONE,
    Eof,

    // Regex pattern
    RegexPattern,

    // Alternation
    Pipe,

    // Quantifiers
    Question,
    Star,
    Plus,
    LBrace,
    RBrace,
    Comma,

    // Groups
    LParen,
    RParen,

    // Character classes
    LBrack,
    RBrack,
    Hat,
    Dash,

    // Assertions
    Dollar,

    // Special characters
    Dot,

    // Escape character
    Backslash,

    // Literals and others
    Literal,
    Character,
    Digit,
    Whitespace,
    Comment,
    Error,
}

impl Display for RegexTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TokenType for RegexTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}
