use oak_core::{ElementType, UniversalElementRole};

/// COBOL element types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CobolElementType {
    /// Root element.
    Root,
    /// Division.
    Division,
    /// Paragraph.
    Paragraph,
}

impl ElementType for CobolElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}
