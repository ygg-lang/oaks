use oak_core::language::{TokenType, UniversalTokenRole};

/// Token types for glob pattern syntax.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobTokenType {
    /// Comment token.
    Comment,
    /// Rule token.
    Rule,
    /// Whitespace token.
    Whitespace,
    /// End of file token.
    Eof,
}

impl TokenType for GlobTokenType {
    type Role = UniversalTokenRole;

    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Comment => UniversalTokenRole::Comment,
            Self::Rule => UniversalTokenRole::Literal,
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Eof => UniversalTokenRole::Eof,
        }
    }
}
