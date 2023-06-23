use crate::lexer::token_type::TomlTokenKind;
use oak_core::{ElementType, UniversalElementRole};

pub type TomlElementType = TomlTokenKind;

impl ElementType for TomlTokenKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Eof => UniversalElementRole::None,
            _ => UniversalElementRole::Value,
        }
    }
}
