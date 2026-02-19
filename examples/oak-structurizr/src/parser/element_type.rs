use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Structurizr DSL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum StructurizrElementType {
    /// The root of a Structurizr document.
    Root,
    /// A workspace definition.
    Workspace,
    /// A model definition.
    Model,
    /// A person in the C4 model.
    Person,
    /// A software system in the C4 model.
    SoftwareSystem,
    /// A container in the C4 model.
    Container,
    /// A component in the C4 model.
    Component,
    /// Error or unknown element.
    Error,
}

impl ElementType for StructurizrElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::StructurizrTokenType> for StructurizrElementType {
    fn from(token: crate::lexer::token_type::StructurizrTokenType) -> Self {
        match token {
            crate::lexer::token_type::StructurizrTokenType::Workspace => Self::Workspace,
            crate::lexer::token_type::StructurizrTokenType::Model => Self::Model,
            crate::lexer::token_type::StructurizrTokenType::Person => Self::Person,
            crate::lexer::token_type::StructurizrTokenType::SoftwareSystem => Self::SoftwareSystem,
            crate::lexer::token_type::StructurizrTokenType::Container => Self::Container,
            crate::lexer::token_type::StructurizrTokenType::Component => Self::Component,
            _ => Self::Error,
        }
    }
}
