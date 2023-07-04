use crate::lexer::token_type::VocTokenType;
use oak_core::{ElementType, UniversalElementRole};

/// VOC element types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum VocElementType {
    /// Root element.
    Root,
    /// VX document root.
    VxDocument,
    /// Template section.
    TemplateSection,
    /// Script section.
    ScriptSection,
    /// Style section.
    StyleSection,
    /// Template element.
    TemplateElement,
    /// Style rule.
    StyleRule,
    /// Style property.
    StyleProperty,
}

impl ElementType for VocElementType {
    type Role = UniversalElementRole;
    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::VxDocument => UniversalElementRole::Container,
            Self::TemplateSection => UniversalElementRole::Container,
            Self::ScriptSection => UniversalElementRole::Container,
            Self::StyleSection => UniversalElementRole::Container,
            Self::TemplateElement => UniversalElementRole::Definition,
            Self::StyleRule => UniversalElementRole::Statement,
            Self::StyleProperty => UniversalElementRole::Attribute,
        }
    }
}

impl From<VocTokenType> for VocElementType {
    fn from(token: VocTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
