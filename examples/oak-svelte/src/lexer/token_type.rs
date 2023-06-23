use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use crate::language::SvelteLanguage;

/// Represents a token in the Svelte language.
pub type SvelteToken = Token<SvelteTokenType>;

/// Token types for the Svelte language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SvelteTokenType {
    /// Whitespace characters.
    Whitespace,
    /// A Svelte or HTML comment.
    Comment,

    /// Opening brace `{`.
    OpenBrace,
    /// Closing brace `}`.
    CloseBrace,
    /// Hash symbol `#` used in control blocks like `{#if}`.
    Hash,
    /// Slash symbol `/` used in control blocks like `{/if}`.
    Slash,
    /// Colon symbol `:` used in control blocks like `{:else}`.
    Colon,
    /// At symbol `@` used in special tags like `{@html}`.
    At,

    /// `if` keyword.
    If,
    /// `else` keyword.
    Else,
    /// `each` keyword.
    Each,
    /// `as` keyword.
    As,
    /// `await` keyword.
    Await,
    /// `then` keyword.
    Then,
    /// `catch` keyword.
    Catch,
    /// `key` keyword.
    Key,
    /// `html` keyword.
    Html,
    /// `const` keyword.
    Const,
    /// `debug` keyword.
    Debug,

    /// Tag opening symbol `<`.
    TagOpen,
    /// Tag closing symbol `>`.
    TagClose,
    /// Self-closing tag symbol `/>`.
    TagSelfClose,
    /// End tag opening symbol `</`.
    TagEndOpen,
    /// Name of an attribute.
    AttributeName,
    /// Value of an attribute.
    AttributeValue,
    /// Plain text content.
    Text,

    /// An identifier.
    Identifier,
    /// A string literal.
    StringLiteral,
    /// A number literal.
    NumberLiteral,

    /// Equals symbol `=`.
    Eq,
    /// Dot symbol `.`.
    Dot,
    /// Comma symbol `,`.
    Comma,
    /// Semicolon symbol `;`.
    Semicolon,

    /// End of file marker.
    Eof,
    /// An error token.
    Error,
}

impl TokenType for SvelteTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::If | Self::Else | Self::Each | Self::Await | Self::Then | Self::Catch | Self::Key | Self::Const => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral | Self::NumberLiteral | Self::Text => UniversalTokenRole::Literal,
            Self::OpenBrace | Self::CloseBrace | Self::Hash | Self::Slash | Self::Colon | Self::At | Self::TagOpen | Self::TagClose | Self::TagSelfClose | Self::TagEndOpen | Self::Eq | Self::Dot | Self::Comma | Self::Semicolon => {
                UniversalTokenRole::Punctuation
            }
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            _ => UniversalTokenRole::None,
        }
    }
}
