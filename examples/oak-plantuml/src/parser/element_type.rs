use oak_core::{ElementType, UniversalElementRole};

/// Element types for the PlantUML language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum PlantUmlElementType {
    /// The root of a PlantUML document.
    Root,
    /// A class definition.
    Class,
    /// An interface definition.
    Interface,
    /// A relationship between elements.
    Relation,
    /// Error or unknown element.
    Error,
}

impl ElementType for PlantUmlElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}
