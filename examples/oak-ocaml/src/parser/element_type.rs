use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OCamlElementType {
    Root,
    ModuleDef,
    LetBinding,
    MatchExpr,
    FunctionDef,
    TypeDefinition,
    Expression,
    Error,
}

impl ElementType for OCamlElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::OCamlTokenType> for OCamlElementType {
    fn from(token: crate::lexer::token_type::OCamlTokenType) -> Self {
        use crate::lexer::token_type::OCamlTokenType;
        match token {
            OCamlTokenType::Root => OCamlElementType::Root,
            OCamlTokenType::ModuleDef => OCamlElementType::ModuleDef,
            OCamlTokenType::LetBinding => OCamlElementType::LetBinding,
            OCamlTokenType::MatchExpr => OCamlElementType::MatchExpr,
            OCamlTokenType::FunctionDef => OCamlElementType::FunctionDef,
            OCamlTokenType::TypeDefinition => OCamlElementType::TypeDefinition,
            OCamlTokenType::Expression => OCamlElementType::Expression,
            _ => OCamlElementType::Error,
        }
    }
}
