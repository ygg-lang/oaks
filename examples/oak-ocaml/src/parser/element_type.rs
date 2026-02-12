use oak_core::{ElementType, UniversalElementRole};

/// Element types for the OCaml parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OCamlElementType {
    /// Root node of the OCaml AST.
    Root,
    /// A module definition.
    ModuleDef,
    /// A let binding.
    LetBinding,
    /// A match expression.
    MatchExpr,
    /// A function definition.
    FunctionDef,
    /// A type definition.
    TypeDefinition,
    /// An expression.
    Expression,
    /// A binary expression.
    BinaryExpr,
    /// A unary expression.
    UnaryExpr,
    /// A function call.
    CallExpr,
    /// A literal expression.
    LiteralExpr,
    /// An identifier expression.
    IdentifierExpr,
    /// An error element.
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
            OCamlTokenType::BinaryExpr => OCamlElementType::BinaryExpr,
            OCamlTokenType::UnaryExpr => OCamlElementType::UnaryExpr,
            OCamlTokenType::CallExpr => OCamlElementType::CallExpr,
            OCamlTokenType::LiteralExpr => OCamlElementType::LiteralExpr,
            OCamlTokenType::IdentifierExpr => OCamlElementType::IdentifierExpr,
            _ => OCamlElementType::Error,
        }
    }
}
