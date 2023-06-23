use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum JavadocElementType {
    Root,
    Whitespace,
    Newline,
    Comment,
    Tag,
    Error,
}

impl ElementType for JavadocElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Comment => UniversalElementRole::Documentation,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::JavadocTokenType> for JavadocElementType {
    fn from(token: crate::lexer::token_type::JavadocTokenType) -> Self {
        use crate::lexer::token_type::JavadocTokenType as T;
        match token {
            T::Root => Self::Root,
            T::Whitespace => Self::Whitespace,
            T::Newline => Self::Newline,
            T::Error => Self::Error,
            _ => Self::Tag,
        }
    }
}
