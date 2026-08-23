use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum RegexElementType {
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

    // Parser specific nodes
    Alternation,
    Concatenation,
    Quantifier,
    Group,
    CharacterClass,
    Assertion,
    Escape,
}

impl Display for RegexElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ElementType for RegexElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::RegexTokenType> for RegexElementType {
    fn from(token: crate::lexer::token_type::RegexTokenType) -> Self {
                match token {
            crate::lexer::token_type::RegexTokenType::TOMBSTONE => Self::TOMBSTONE,
            crate::lexer::token_type::RegexTokenType::Eof => Self::Eof,
            crate::lexer::token_type::RegexTokenType::RegexPattern => Self::RegexPattern,
            crate::lexer::token_type::RegexTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::RegexTokenType::Question => Self::Question,
            crate::lexer::token_type::RegexTokenType::Star => Self::Star,
            crate::lexer::token_type::RegexTokenType::Plus => Self::Plus,
            crate::lexer::token_type::RegexTokenType::LBrace => Self::LBrace,
            crate::lexer::token_type::RegexTokenType::RBrace => Self::RBrace,
            crate::lexer::token_type::RegexTokenType::Comma => Self::Comma,
            crate::lexer::token_type::RegexTokenType::LParen => Self::LParen,
            crate::lexer::token_type::RegexTokenType::RParen => Self::RParen,
            crate::lexer::token_type::RegexTokenType::LBrack => Self::LBrack,
            crate::lexer::token_type::RegexTokenType::RBrack => Self::RBrack,
            crate::lexer::token_type::RegexTokenType::Hat => Self::Hat,
            crate::lexer::token_type::RegexTokenType::Dash => Self::Dash,
            crate::lexer::token_type::RegexTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::RegexTokenType::Dot => Self::Dot,
            crate::lexer::token_type::RegexTokenType::Backslash => Self::Backslash,
            crate::lexer::token_type::RegexTokenType::Literal => Self::Literal,
            crate::lexer::token_type::RegexTokenType::Character => Self::Character,
            crate::lexer::token_type::RegexTokenType::Digit => Self::Digit,
            crate::lexer::token_type::RegexTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::RegexTokenType::Comment => Self::Comment,
            crate::lexer::token_type::RegexTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
