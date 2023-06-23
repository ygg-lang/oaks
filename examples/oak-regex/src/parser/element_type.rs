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
        unsafe { std::mem::transmute(token) }
    }
}
