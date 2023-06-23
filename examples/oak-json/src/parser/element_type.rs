use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JsonElementType {
    Root,
    Value,
    Object,
    Array,
    String,
    Number,
    Boolean,
    Null,
    ObjectEntry,
    ArrayElement,
    ErrorNode,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    NullLiteral,
    BareKey,
    Whitespace,
    Comment,
    Eof,
    Error,
}

impl ElementType for JsonElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,

            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::JsonTokenType> for JsonElementType {
    fn from(token: crate::lexer::token_type::JsonTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
