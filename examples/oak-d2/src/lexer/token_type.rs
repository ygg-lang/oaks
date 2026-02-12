use oak_core::{TokenType, UniversalTokenRole};

/// Token types for the D2 language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum D2TokenType {
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,

    /// Identifier.
    Id,
    /// Label.
    Label,
    /// Colon `:`.
    Colon,
    /// Arrow `->`.
    Arrow,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,

    /// Lexing error.
    Error,
}

impl TokenType for D2TokenType {
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
