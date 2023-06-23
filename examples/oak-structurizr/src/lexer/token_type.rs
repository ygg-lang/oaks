use oak_core::{TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum StructurizrTokenType {
    Whitespace,
    Newline,
    Comment,

    Workspace,
    Model,
    Person,
    SoftwareSystem,
    Container,
    Component,
    Id,
    Label,
    LeftBrace,
    RightBrace,

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
