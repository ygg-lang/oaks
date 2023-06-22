use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use std::fmt::Display;

pub type RegexToken = Token<RegexSyntaxKind>;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, serde::Serialize, serde::Deserialize)]
#[repr(u16)]
pub enum RegexSyntaxKind {
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

    // Characters
    Character,
    Digit,
    Whitespace,

    // Error and comments
    Error,
    Comment,
}

impl Display for RegexSyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TokenType for RegexSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            RegexSyntaxKind::Whitespace => UniversalTokenRole::Whitespace,
            RegexSyntaxKind::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for RegexSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
