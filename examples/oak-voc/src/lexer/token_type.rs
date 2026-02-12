use oak_core::{TokenType, UniversalTokenRole};

/// VOC token types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum VocTokenType {
    /// End of file.
    Eof,
    /// Whitespace.
    Whitespace,
}

impl TokenType for VocTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Eof => UniversalTokenRole::Eof,
            Self::Whitespace => UniversalTokenRole::Whitespace,
        }
    }
}
