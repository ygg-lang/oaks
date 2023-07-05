use oak_core::{ElementType, UniversalElementRole};

/// Solidity element types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SolidityElementType {
    /// Root element.
    Root,
    /// Pragma directive.
    PragmaDirective,
    /// Contract definition.
    ContractDefinition,
    /// Error element.
    Error,
}

impl ElementType for SolidityElementType {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::PragmaDirective | Self::ContractDefinition => UniversalElementRole::Definition,
            Self::Error => UniversalElementRole::Error,
        }
    }
}
