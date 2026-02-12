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
