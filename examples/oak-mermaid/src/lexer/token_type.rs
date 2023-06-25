use oak_core::{TokenType, UniversalTokenRole};

/// Token types for the Mermaid language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
