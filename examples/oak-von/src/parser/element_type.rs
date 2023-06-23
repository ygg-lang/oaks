use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VonElementType {
    Whitespace,
    Newline,
    Comment,
    Eof,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Eq,
    StringLiteral,
    NumberLiteral,
    BoolLiteral,
    NullLiteral,
    Identifier,
    Value,
    Object,
    Array,
    ObjectEntry,
    Enum,
    ErrorNode,
    Error,
    Root,
    ArrayElement,
}

impl ElementType for VonElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,

            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::VonTokenType> for VonElementType {
    fn from(token: crate::lexer::token_type::VonTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
