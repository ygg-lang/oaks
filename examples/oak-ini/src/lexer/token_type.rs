//! INI token types and roles.

use oak_core::{Token, TokenType, UniversalTokenRole};

/// A token in the INI language.
pub type IniToken = Token<IniTokenType>;

/// INI token types.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IniTokenType {
    /// Whitespace characters.
    Whitespace,
    /// Newline character.
    Newline,
    /// Comment.
    Comment,
    /// Error token.
    Error,
    /// End of file.
    Eof,

    /// `{` symbol.
    LeftBrace,
    /// `}` symbol.
    RightBrace,
    /// `[` symbol.
    LeftBracket,
    /// `]` symbol.
    RightBracket,
    /// `[[` symbol.
    DoubleLeftBracket,
    /// `]]` symbol.
    DoubleRightBracket,
    /// `,` symbol.
    Comma,
    /// `.` symbol.
    Dot,
    /// `=` operator.
    Equal,

    /// Identifier.
    Identifier,
    /// String literal.
    String,
    /// Integer literal.
    Integer,
    /// Floating-point literal.
    Float,
    /// Boolean literal.
    Boolean,
    /// Date and time literal.
    DateTime,
}

impl TokenType for IniTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::DoubleLeftBracket | Self::DoubleRightBracket | Self::Comma | Self::Dot => UniversalTokenRole::Punctuation,
            Self::Equal => UniversalTokenRole::Operator,
            Self::Identifier => UniversalTokenRole::Name,
            Self::String | Self::Integer | Self::Float | Self::Boolean | Self::DateTime => UniversalTokenRole::Literal,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::Eof,
        }
    }
}
