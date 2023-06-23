use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PascalElementType {
    Root,
    Program,
    Unit,
    Interface,
    Implementation,
    Initialization,
    Finalization,
    ConstSection,
    TypeSection,
    VarSection,
    Procedure,
    Function,
    Block,
    Statement,
    Expression,
    Error,
}

impl ElementType for PascalElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::PascalTokenType> for PascalElementType {
    fn from(token: crate::lexer::token_type::PascalTokenType) -> Self {
        use crate::lexer::token_type::PascalTokenType;
        match token {
            PascalTokenType::Root => PascalElementType::Root,
            PascalTokenType::ProgramBlock => PascalElementType::Program,
            PascalTokenType::VarSection => PascalElementType::VarSection,
            PascalTokenType::ConstSection => PascalElementType::ConstSection,
            PascalTokenType::TypeSection => PascalElementType::TypeSection,
            PascalTokenType::ProcedureDef => PascalElementType::Procedure,
            PascalTokenType::FunctionDef => PascalElementType::Function,
            PascalTokenType::CompoundStmt => PascalElementType::Block,
            PascalTokenType::Expression => PascalElementType::Expression,
            _ => PascalElementType::Error,
        }
    }
}
