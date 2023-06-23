use oak_core::{TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
/// Represents all possible token kinds in the Bash shell scripting language.
pub enum BashTokenType {
    /// Whitespace characters (spaces, tabs)
    Whitespace,
    /// Newline characters
    Newline,
    /// Comments (starting with #)
    Comment,
    /// String literals enclosed in quotes
    StringLiteral,
    /// Variable references (e.g., $VAR)
    Variable,
    /// Numeric literals
    NumberLiteral,
    /// Identifiers (variable names, function names, etc.)
    Identifier,
    /// Bash keywords (if, then, else, etc.)
    Keyword,
    /// Operators (&&, ||, >, <, etc.)
    Operator,
    /// Delimiters (, (, ), {, }, etc.)
    Delimiter,
    /// Command names
    Command,
    /// File system paths
    Path,
    /// Here documents
    Heredoc,
    /// Glob patterns (*, ?, [])
    GlobPattern,
    /// Special characters with specific meaning
    SpecialChar,
    /// Plain text content
    Text,
    /// Error token
    Error,
    /// End of file marker
    Eof,
}

impl TokenType for BashTokenType {
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
            Self::Identifier | Self::Variable | Self::Command => UniversalTokenRole::Name,
            Self::StringLiteral | Self::NumberLiteral => UniversalTokenRole::Literal,
            Self::Operator => UniversalTokenRole::Operator,
            Self::Delimiter | Self::SpecialChar => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
