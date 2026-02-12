use oak_core::{Token, TokenType, UniversalTokenRole};

/// Alias for `Token<JsonTokenType>`.
pub type JsonToken = Token<JsonTokenType>;

/// Token types for the JSON lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JsonTokenType {
    /// Root node of the JSON document.
    Root,
    /// A JSON value.
    Value,
    /// A JSON object.
    Object,
    /// A JSON array.
    Array,
    /// A JSON string.
    String,
    /// A JSON number.
    Number,
    /// A JSON boolean.
    Boolean,
    /// A JSON null value.
    Null,
    /// An entry in a JSON object.
    ObjectEntry,
    /// An element in a JSON array.
    ArrayElement,
    /// An error node in the parse tree.
    ErrorNode,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Comma `,`.
    Comma,
    /// Colon `:`.
    Colon,
    /// A string literal.
    StringLiteral,
    /// A number literal.
    NumberLiteral,
    /// A boolean literal (`true` or `false`).
    BooleanLiteral,
    /// A null literal (`null`).
    NullLiteral,
    /// A bare key (used in some JSON variants).
    BareKey,
    /// Whitespace (spaces, tabs, newlines).
    Whitespace,
    /// A comment.
    Comment,
    /// End of stream.
    Eof,
    /// An error token.
    Error,
}

impl TokenType for JsonTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
