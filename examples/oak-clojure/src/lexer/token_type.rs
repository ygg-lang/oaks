/// Token type definitions for Clojure lexer.
use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

/// Type alias for a Clojure token.
pub type ClojureToken = Token<ClojureTokenType>;

/// Token types for Clojure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClojureTokenType {
    /// Generic token.
    Token,
    /// List node.
    List,
    /// Vector node.
    Vector,
    /// Map node.
    Map,
    /// Set node.
    Set,
    /// Anonymous function node.
    AnonFn,
    /// Root node.
    Root,
    /// Source file node.
    SourceFile,
    /// Error token.
    Error,
    // Lexer tokens
    /// `(`
    ListStart,
    /// `)`
    ListEnd,
    /// `[`
    VectorStart,
    /// `]`
    VectorEnd,
    /// `{`
    MapStart,
    /// `}`
    MapEnd,
    /// `#{`
    SetStart,
    /// `#(`
    AnonFnStart,
    /// `'`
    Quote,
    /// `~`
    Unquote,
    /// `~@`
    UnquoteSplice,
    /// `^`
    Meta,
    /// Whitespace.
    Whitespace,
    /// Comment.
    Comment,
    /// String literal.
    StringLiteral,
    /// Character literal.
    CharacterLiteral,
    /// Number literal.
    NumberLiteral,
    /// Keyword literal.
    KeywordLiteral,
    /// `#`
    Dispatch,
    /// Regex literal.
    RegexLiteral,
    /// Symbol.
    Symbol,
}

impl TokenType for ClojureTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
