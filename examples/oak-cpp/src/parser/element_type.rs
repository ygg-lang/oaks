use crate::lexer::CppTokenType;
use oak_core::{ElementType, UniversalElementRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
/// Syntax element types for the C++ language.
pub enum CppElementType {
    /// A wrapper for tokens
    Token(CppTokenType),
    /// Root node of the source file
    SourceFile,
    /// A function definition
    FunctionDefinition,
    /// A class definition
    ClassDefinition,
    /// A namespace definition
    NamespaceDefinition,
    /// A declaration statement
    DeclarationStatement,
    /// An expression statement
    ExpressionStatement,
    /// A compound statement (block)
    CompoundStatement,
    /// An if statement
    IfStatement,
    /// A while statement
    WhileStatement,
    /// A for statement
    ForStatement,
    /// A return statement
    ReturnStatement,
    /// A function call
    FunctionCall,
    /// An error token
    Error,
}

impl From<CppTokenType> for CppElementType {
    fn from(token: CppTokenType) -> Self {
        Self::Token(token)
    }
}

impl ElementType for CppElementType {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::SourceFile)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::FunctionDefinition | Self::ClassDefinition | Self::NamespaceDefinition => UniversalElementRole::Definition,
            Self::DeclarationStatement | Self::ExpressionStatement | Self::CompoundStatement | Self::IfStatement | Self::WhileStatement | Self::ForStatement | Self::ReturnStatement => UniversalElementRole::Statement,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::Container,
        }
    }
}
