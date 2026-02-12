use oak_core::{TokenType, UniversalTokenRole};

/// Token types for PlantUML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum PlantUmlTokenType {
    /// Whitespace characters.
    Whitespace,
    /// Newline characters.
    Newline,
    /// Comment lines or blocks.
    Comment,

    /// '@startuml' marker.
    StartUml,
    /// '@enduml' marker.
    EndUml,
    /// 'class' keyword.
    Class,
    /// 'interface' keyword.
    Interface,
    /// An identifier.
    Id,
    /// A label or description.
    Label,

    /// Error or unknown token.
    Error,
}

impl TokenType for PlantUmlTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}
