use oak_core::{TokenType, UniversalTokenRole};

/// Token types for the Mermaid language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MermaidTokenType {
    /// Whitespace characters.
    Whitespace,
    /// Newline character.
    Newline,
    /// Comment.
    Comment,

    /// `graph` keyword.
    Graph,
    /// Direction (e.g., LR, TD).
    Direction,
    /// Identifier.
    Id,
    /// Node label.
    Label,
    /// Connection arrow (e.g., `-->`).
    Arrow,

    /// Lexing error.
    Error,
}

#[cfg(feature = "serde")]
impl serde::Serialize for MermaidTokenType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MermaidTokenType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Ok(unsafe { std::mem::transmute(value) })
    }
}

impl TokenType for MermaidTokenType {
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
