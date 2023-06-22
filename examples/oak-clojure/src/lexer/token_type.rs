use oak_core::{TokenType, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// Represents all possible token kinds in the Clojure programming language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ClojureTokenType {
    // Trivia
    /// Whitespace characters (spaces, tabs)
    Whitespace,
    /// Newline characters
    Newline,
    /// Comments (both single-line and multi-line)
    Comment,

    // Literals
    /// String literals (e.g., "hello")
    StringLiteral,
    /// Character literals (e.g., \a)
    CharacterLiteral,
    /// Number literals (integer and floating-point)
    NumberLiteral,
    /// Boolean literals (true, false)
    BooleanLiteral,
    /// Nil literal
    NilLiteral,
    /// Keyword literals (e.g., :keyword)
    KeywordLiteral,

    // Identifiers and symbols
    /// Symbol identifiers (e.g., variable names, function names)
    Symbol,
    /// Keyword identifiers (e.g., :keyword)
    Keyword,

    // Collections
    /// List start delimiter: `(`
    ListStart,
    /// List end delimiter: `)`
    ListEnd,
    /// Vector start delimiter: `[`
    VectorStart,
    /// Vector end delimiter: `]`
    VectorEnd,
    /// Map start delimiter: `{`
    MapStart,
    /// Map end delimiter: `}`
    MapEnd,
    /// Set start delimiter: `#{`
    SetStart,

    // Special forms
    /// Quote form (e.g., 'expr)
    Quote,
    /// Unquote form (e.g., ~expr)
    Unquote,
    /// Unquote-splicing form (e.g., ~@expr)
    UnquoteSplice,
    /// Deref form (e.g., @expr)
    Deref,
    /// Metadata form (e.g., ^metadata expr)
    Meta,
    /// Dispatch macro (e.g., #)
    Dispatch,

    // Reader macros
    /// Reader macro form (e.g., #tag expr)
    ReaderMacro,

    // Regex
    /// Regular expression literals (e.g., #"pattern")
    RegexLiteral,

    // Anonymous function
    /// Anonymous function start delimiter: #(
    AnonFnStart,
    /// Anonymous function argument (e.g., %1, %2, etc.)
    AnonFnArg,

    // Error handling
    /// Error token
    Error,
    /// End of file marker
    Eof,
}

impl TokenType for ClojureTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        self.is_whitespace() || self.is_comment()
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
            Self::StringLiteral | Self::CharacterLiteral | Self::NumberLiteral | Self::BooleanLiteral | Self::NilLiteral | Self::KeywordLiteral | Self::RegexLiteral => UniversalTokenRole::Literal,
            Self::Symbol | Self::Keyword | Self::AnonFnArg => UniversalTokenRole::Name,
            Self::ListStart
            | Self::ListEnd
            | Self::VectorStart
            | Self::VectorEnd
            | Self::MapStart
            | Self::MapEnd
            | Self::SetStart
            | Self::Quote
            | Self::Unquote
            | Self::UnquoteSplice
            | Self::Deref
            | Self::Meta
            | Self::Dispatch
            | Self::ReaderMacro
            | Self::AnonFnStart => UniversalTokenRole::Punctuation,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::None,
        }
    }
}
