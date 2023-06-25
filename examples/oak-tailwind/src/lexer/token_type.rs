//! Token types for the Tailwind DSL.
use oak_core::{Token, TokenType, UniversalTokenRole};

/// Type alias for a Tailwind token.
pub type TailwindToken = Token<TailwindTokenType>;

/// Token types for the Tailwind DSL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TailwindTokenType {
    /// The root node of the parse tree.
    Root,
    /// A utility class (e.g., `bg-red-500`, `p-4`).
    Utility,
    /// A modifier (e.g., `hover:`, `md:`, `dark:`).
    Modifier,
    /// An arbitrary value (e.g., `[100px]`, `[#000]`).
    ArbitraryValue,
    /// The important flag (`!`).
    Important,
    /// A directive (e.g., `@tailwind`, `@apply`, `@layer`).
    Directive,
    /// A CSS property name or identifier.
    Identifier,
    /// A CSS value or literal.
    Value,

    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Colon `:`.
    Colon,
    /// Semicolon `;`.
    Semicolon,
    /// At sign `@`.
    At,
    /// Exclamation mark `!`.
    Bang,
    /// Dash `-`.
    Dash,
    /// Slash `/`.
    Slash,
    /// Dot `.`.
    Dot,
    /// Hash `#`.
    Hash,
    /// Comma `,`.
    Comma,
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,

    /// Whitespace.
    Whitespace,
    /// Comment.
    Comment,
    /// Unknown or error token.
    Unknown,
    /// End of stream.
    Eof,
}

impl core::fmt::Display for TailwindTokenType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Root => f.write_str("Root"),
            Self::Utility => f.write_str("Utility"),
            Self::Modifier => f.write_str("Modifier"),
            Self::ArbitraryValue => f.write_str("ArbitraryValue"),
            Self::Important => f.write_str("Important"),
            Self::Directive => f.write_str("Directive"),
            Self::Identifier => f.write_str("Identifier"),
            Self::Value => f.write_str("Value"),
            Self::LeftBracket => f.write_str("["),
            Self::RightBracket => f.write_str("]"),
            Self::Colon => f.write_str(":"),
            Self::Semicolon => f.write_str(";"),
            Self::At => f.write_str("@"),
            Self::Bang => f.write_str("!"),
            Self::Dash => f.write_str("-"),
            Self::Slash => f.write_str("/"),
            Self::Dot => f.write_str("."),
            Self::Hash => f.write_str("#"),
            Self::Comma => f.write_str(","),
            Self::LeftParen => f.write_str("("),
            Self::RightParen => f.write_str(")"),
            Self::Whitespace => f.write_str("Whitespace"),
            Self::Comment => f.write_str("Comment"),
            Self::Unknown => f.write_str("Unknown"),
            Self::Eof => f.write_str("Eof"),
        }
    }
}

impl TokenType for TailwindTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalTokenRole::None,
            Self::Utility => UniversalTokenRole::Keyword,
            Self::Modifier => UniversalTokenRole::Keyword,
            Self::ArbitraryValue => UniversalTokenRole::Literal,
            Self::Important => UniversalTokenRole::Operator,
            Self::Directive => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Value => UniversalTokenRole::Literal,
            Self::LeftBracket | Self::RightBracket | Self::LeftParen | Self::RightParen => UniversalTokenRole::Punctuation,
            Self::Colon | Self::Semicolon | Self::At | Self::Bang | Self::Dash | Self::Slash | Self::Dot | Self::Hash | Self::Comma => UniversalTokenRole::Punctuation,
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Unknown => UniversalTokenRole::Error,
        }
    }
}
