use oak_core::ElementType;

/// Vampire element types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VampireElementType {
    /// Root element.
    Root,
}

impl ElementType for VampireElementType {
    type Role = oak_core::UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            VampireElementType::Root => oak_core::UniversalElementRole::Root,
        }
    }
}

impl From<crate::lexer::token_type::VampireTokenType> for VampireElementType {
    fn from(_: crate::lexer::token_type::VampireTokenType) -> Self {
        VampireElementType::Root
    }
}

impl From<VampireElementType> for u16 {
    fn from(t: VampireElementType) -> u16 {
        t as u16
    }
}

impl From<u16> for VampireElementType {
    fn from(i: u16) -> Self {
        match i {
            0 => VampireElementType::Root,
            _ => VampireElementType::Root, // Fallback
        }
    }
}
