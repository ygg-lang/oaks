use oak_core::{TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
/// Represents all possible token kinds in the Windows Batch (BAT) scripting language.
pub enum BatTokenType {
    /// Whitespace characters
    Whitespace,
    /// Newline characters
    Newline,
    /// Comments (starting with REM or ::)
    Comment,
    /// String literals
    StringLiteral,
    /// Variable references (e.g., %VAR%, !VAR!)
    Variable,
    /// Numeric literals
    NumberLiteral,
    /// Identifiers
    Identifier,
    /// Batch keywords (IF, FOR, SET, etc.)
    Keyword,
    /// Operators (==, EQU, NEQ, etc.)
    Operator,
    /// Delimiters
    Delimiter,
    /// Command names
    Command,
    /// Labels (starting with :)
    Label,
    /// Plain text content
    Text,
    /// Error token
    Error,
    /// End of file marker
    Eof,
}

impl oak_core::TokenType for BatTokenType {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Keyword => UniversalTokenRole::Keyword,
            Self::Identifier | Self::Variable | Self::Command | Self::Label => UniversalTokenRole::Name,
            Self::StringLiteral | Self::NumberLiteral => UniversalTokenRole::Literal,
            Self::Operator => UniversalTokenRole::Operator,
            Self::Delimiter => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
