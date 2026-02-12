use oak_core::{ElementType, UniversalElementRole};

/// Valkyrie element types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValkyrieElementType {
    /// Root element.
    Root,
}

impl ElementType for ValkyrieElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
        }
    }
}
