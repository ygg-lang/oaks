//! Svelte token types.

use oak_core::{Token, TokenType, UniversalTokenRole};

/// Svelte token.
pub type SvelteToken = Token<SvelteTokenType>;

/// Svelte token types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SvelteTokenType {
    // --- Lexical Tokens ---
    /// Whitespace.
    Whitespace,
    /// Comment.
    Comment,

    // Keywords
    /// `import` keyword.
    Import,
    /// `export` keyword.
    Export,
    /// `default` keyword.
    Default,
    /// `const` keyword.
    Const,
    /// `let` keyword.
    Let,
    /// `var` keyword.
    Var,
    /// `function` keyword.
    Function,
    /// `if` keyword.
    If,
    /// `else` keyword.
    Else,
    /// `while` keyword.
    While,
    /// `for` keyword.
    For,
    /// `return` keyword.
    Return,
    /// `true` keyword.
    True,
    /// `false` keyword.
    False,
    /// `null` keyword.
    Null,

    // Literals & Identifiers
    /// Identifier.
    Identifier,
    /// String literal.
    StringLiteral,
    /// Number literal.
    NumberLiteral,
    /// Plain text in template.
    Text,

    // Operators & Punctuation
    /// Plus `+`.
    Plus,
    /// Minus `-`.
    Minus,
    /// Star `*`.
    Star,
    /// Slash `/`.
    Slash,
    /// Equals `=`.
    Eq,
    /// Dot `.`.
    Dot,
    /// Colon `:`.
    Colon,
    /// Comma `,`.
    Comma,
    /// Semicolon `;`.
    Semicolon,
    /// Hash `#`.
    Hash,
    /// At `@`.
    At,

    // Delimiters
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBraceBracket, // Fix: naming conflict

    // Svelte Blocks
    /// `{#` start of a block.
    HashBrace,
    /// `{/` end of a block.
    SlashBrace,
    /// `{:else` or `{:then`.
    ColonBrace,

    // HTML-like tokens
    /// `<` start tag.
    Lt,
    /// `>` end tag.
    Gt,
    /// `/>` self-closing.
    SlashGt,
    /// `</` closing tag.
    LtSlash,

    /// End of file.
    Eof,
    /// Error.
    Error,

    // --- Structural Elements ---
    /// Root node.
    Root,
    /// Element.
    Element,
    /// Attribute.
    Attribute,
    /// Expression.
    Expression,
    /// Block (if, each, await).
    Block,
}

impl TokenType for SvelteTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Import | Self::Export | Self::Default | Self::Const | Self::Let | Self::Var | Self::Function | Self::If | Self::Else | Self::While | Self::For | Self::Return | Self::True | Self::False | Self::Null => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral => UniversalTokenRole::Literal,
            Self::NumberLiteral => UniversalTokenRole::Literal,
            Self::Text => UniversalTokenRole::Literal,
            Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Eq | Self::Dot | Self::Colon | Self::Comma | Self::Semicolon | Self::Hash | Self::At => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBraceBracket => UniversalTokenRole::Punctuation,
            Self::HashBrace | Self::SlashBrace | Self::ColonBrace | Self::Lt | Self::Gt | Self::SlashGt | Self::LtSlash => UniversalTokenRole::Punctuation,
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
