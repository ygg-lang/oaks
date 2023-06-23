use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SassElementType {
    SourceFile,
    Root,
    Selector,
    Variable,
    Mixin,
    Function,
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
    Whitespace,
    Comment,
    Eof,
    Error,
}

impl ElementType for SassElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile | Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::SassTokenType> for SassElementType {
    fn from(token: crate::lexer::token_type::SassTokenType) -> Self {
        use crate::lexer::token_type::SassTokenType as T;
        match token {
            T::Whitespace => Self::Whitespace,
            T::LineComment | T::BlockComment => Self::Comment,
            T::Eof => Self::Eof,
            T::Error => Self::Error,
            T::LeftBrace => Self::LeftBrace,
            T::RightBrace => Self::RightBrace,
            T::LeftBracket => Self::LeftBracket,
            T::RightBracket => Self::RightBracket,
            T::Comma => Self::Comma,
            T::Colon => Self::Colon,
            _ => Self::Error, // Fallback for now
        }
    }
}
