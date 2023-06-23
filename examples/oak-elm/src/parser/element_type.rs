use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ElmElementType {
    Root,
    Module,
    Import,
    TypeDeclaration,
    TypeAlias,
    FunctionDeclaration,
    Expression,
    Literal,
    Identifier,
    BinaryExpression,
    UnaryExpression,
    IfExpression,
    CaseExpression,
    LetExpression,
    TupleExpression,
    ListExpression,
    RecordExpression,
    FieldExpression,
    LambdaExpression,
}

impl ElementType for ElmElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Module => UniversalElementRole::Definition,
            Self::Import => UniversalElementRole::Statement,
            Self::FunctionDeclaration => UniversalElementRole::Definition,
            Self::Expression => UniversalElementRole::Expression,
            Self::Identifier => UniversalElementRole::Name,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ElmTokenType> for ElmElementType {
    fn from(token: crate::lexer::token_type::ElmTokenType) -> Self {
        match token {
            crate::lexer::token_type::ElmTokenType::Root => Self::Root,
            crate::lexer::token_type::ElmTokenType::Identifier => Self::Identifier,
            _ => unsafe { std::mem::transmute(token) },
        }
    }
}
