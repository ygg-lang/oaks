use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ObjectiveCElementType {
    Root,
    InterfaceDeclaration,
    ImplementationDeclaration,
    ProtocolDeclaration,
    PropertyDeclaration,
    MethodDeclaration,
    CategoryDeclaration,
    ClassExtension,
    Error,
}

impl ElementType for ObjectiveCElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ObjectiveCTokenType> for ObjectiveCElementType {
    fn from(token: crate::lexer::token_type::ObjectiveCTokenType) -> Self {
        use crate::lexer::token_type::ObjectiveCTokenType;
        match token {
            ObjectiveCTokenType::Root => Self::Root,
            ObjectiveCTokenType::InterfaceDeclaration => Self::InterfaceDeclaration,
            ObjectiveCTokenType::ImplementationDeclaration => Self::ImplementationDeclaration,
            ObjectiveCTokenType::ProtocolDeclaration => Self::ProtocolDeclaration,
            ObjectiveCTokenType::PropertyDeclaration => Self::PropertyDeclaration,
            ObjectiveCTokenType::MethodDeclaration => Self::MethodDeclaration,
            ObjectiveCTokenType::CategoryDeclaration => Self::CategoryDeclaration,
            ObjectiveCTokenType::ClassExtension => Self::ClassExtension,
            _ => Self::Error,
        }
    }
}
