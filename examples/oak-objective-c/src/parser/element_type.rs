use oak_core::{ElementType, UniversalElementRole};

/// Objective-C element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectiveCElementType {
    /// Root node.
    Root,
    /// Interface declaration.
    InterfaceDeclaration,
    /// Implementation declaration.
    ImplementationDeclaration,
    /// Protocol declaration.
    ProtocolDeclaration,
    /// Property declaration.
    PropertyDeclaration,
    /// Method declaration.
    MethodDeclaration,
    /// Category declaration.
    CategoryDeclaration,
    /// Class extension.
    ClassExtension,
    /// Error node.
    Error,
}

impl ElementType for ObjectiveCElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::InterfaceDeclaration | Self::ProtocolDeclaration | Self::ImplementationDeclaration | Self::CategoryDeclaration | Self::ClassExtension => UniversalElementRole::Definition, // Best fit for declarations
            Self::PropertyDeclaration | Self::MethodDeclaration => UniversalElementRole::Definition,                                                                                         // Best fit for members
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
