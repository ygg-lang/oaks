use oak_core::{TokenType, UniversalTokenRole};

/// Token types for the Structurizr DSL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum StructurizrTokenType {
    /// Whitespace characters.
    Whitespace,
    /// Newline characters.
    Newline,
    /// Comment lines or blocks.
    Comment,
    /// The 'workspace' keyword.
    Workspace,
    /// The 'model' keyword.
    Model,
    /// The 'person' keyword.
    Person,
    /// The 'softwareSystem' keyword.
    SoftwareSystem,
    /// The 'container' keyword.
    Container,
    /// The 'component' keyword.
    Component,
    /// An identifier.
    Id,
    /// A label or description.
    Label,
    /// Left brace '{'.
    LeftBrace,
    /// Right brace '}'.
    RightBrace,
    /// Error or unknown token.
    Error,
}

impl TokenType for StructurizrTokenType {
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
