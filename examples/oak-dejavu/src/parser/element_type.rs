use oak_core::{ElementType, UniversalElementRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DejavuElementType {
    Root,
    Eof,
    Whitespace,
    Error,
}

impl ElementType for DejavuElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::DejavuTokenType> for DejavuElementType {
    fn from(token: crate::lexer::token_type::DejavuTokenType) -> Self {
        match token {
            crate::lexer::token_type::DejavuTokenType::Eof => Self::Eof,
            crate::lexer::token_type::DejavuTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::DejavuTokenType::Error => Self::Error,
        }
    }
}
