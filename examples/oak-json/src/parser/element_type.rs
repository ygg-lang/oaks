use oak_core::{ElementType, UniversalElementRole};

/// Element types for the JSON parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JsonElementType {
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
    /// An error element.
    Error,
}

impl ElementType for JsonElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,

            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::JsonTokenType> for JsonElementType {
    fn from(token: crate::lexer::token_type::JsonTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
