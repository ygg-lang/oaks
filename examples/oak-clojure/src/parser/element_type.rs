use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ClojureElementType {
    Token,
    List,
    Vector,
    Map,
    Set,
    AnonFn,
    Root,
    SourceFile,
    Error,
}

impl ElementType for ClojureElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ClojureTokenType> for ClojureElementType {
    fn from(token: crate::lexer::token_type::ClojureTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
