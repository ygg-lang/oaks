use oak_core::{ElementType, UniversalElementRole};

/// Smalltalk element types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SmalltalkElementType {
    /// Root element.
    Root,
    /// Method definition.
    MethodDefinition,
    /// Message send.
    MessageSend,
    /// Error.
    Error,
}

impl ElementType for SmalltalkElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::MethodDefinition => UniversalElementRole::Definition,
            Self::MessageSend => UniversalElementRole::Call,
            Self::Error => UniversalElementRole::Error,
        }
    }
}
