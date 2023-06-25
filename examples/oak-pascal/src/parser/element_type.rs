use oak_core::{ElementType, Parser, UniversalElementRole};

/// Element types for Pascal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PascalElementType {
    /// The root node of the AST.
    Root,
    /// A program node.
    Program,
    /// A unit node.
    Unit,
    /// An interface section.
    Interface,
    /// An implementation section.
    Implementation,
    /// An initialization section.
    Initialization,
    /// A finalization section.
    Finalization,
    /// A constant declaration section.
    ConstSection,
    /// A type declaration section.
    TypeSection,
    /// A variable declaration section.
    VarSection,
    /// A procedure declaration.
    Procedure,
    /// A function declaration.
    Function,
    /// A code block.
    Block,
    /// A statement.
    Statement,
    /// An expression.
    Expression,
    /// An error element.
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
