/// Fluent token types.
use oak_core::{TokenRole, UniversalTokenRole};

/// Fluent token kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FluentTokenKind {
    /// End of file.
    Eof,
    /// Whitespace.
    Whitespace,
    /// Comment.
    Comment,
    /// Identifier.
    Identifier,
    /// String literal.
    StringLiteral,
    /// Number literal.
    NumberLiteral,
    /// Equals sign.
    Equals,
    /// Left bracket.
    LeftBracket,
    /// Right bracket.
    RightBracket,
    /// Left brace.
    LeftBrace,
    /// Right brace.
    RightBrace,
    /// Comma.
    Comma,
    /// Period.
    Period,
    /// Colon.
    Colon,
    /// Hyphen.
    Hyphen,
    /// Underscore.
    Underscore,
    /// At symbol.
    At,
    /// Hash symbol.
    Hash,
    /// Dollar sign.
    Dollar,
    /// Pipe symbol.
    Pipe,
    /// Asterisk.
    Asterisk,
    /// Error token.
    Error,
}

impl oak_core::TokenType for FluentTokenKind {
    /// The associated role type.
    type Role = UniversalTokenRole;

    /// End of stream token.
    const END_OF_STREAM: Self = Self::Eof;

    /// Returns the role of the token.
    fn role(&self) -> Self::Role {
        match self {
            Self::Eof => UniversalTokenRole::Eof,
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral => UniversalTokenRole::Literal,
            Self::NumberLiteral => UniversalTokenRole::Literal,
            Self::Equals | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Comma | Self::Period | Self::Colon | Self::Hyphen | Self::Underscore | Self::At | Self::Hash | Self::Dollar | Self::Pipe | Self::Asterisk => {
                UniversalTokenRole::Punctuation
            }
            Self::Error => UniversalTokenRole::Error,
        }
    }
}
