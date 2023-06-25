use oak_core::{ElementType, UniversalElementRole};

/// VOC element types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum VocElementType {
    /// Root element.
    Root,
}

impl ElementType for VocElementType {
    type Role = UniversalElementRole;
    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
        }
    }
}
