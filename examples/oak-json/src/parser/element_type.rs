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
                match token {
            crate::lexer::token_type::JsonTokenType::Root => Self::Root,
            crate::lexer::token_type::JsonTokenType::Value => Self::Value,
            crate::lexer::token_type::JsonTokenType::Object => Self::Object,
            crate::lexer::token_type::JsonTokenType::Array => Self::Array,
            crate::lexer::token_type::JsonTokenType::String => Self::String,
            crate::lexer::token_type::JsonTokenType::Number => Self::Number,
            crate::lexer::token_type::JsonTokenType::Boolean => Self::Boolean,
            crate::lexer::token_type::JsonTokenType::Null => Self::Null,
            crate::lexer::token_type::JsonTokenType::ObjectEntry => Self::ObjectEntry,
            crate::lexer::token_type::JsonTokenType::ArrayElement => Self::ArrayElement,
            crate::lexer::token_type::JsonTokenType::ErrorNode => Self::ErrorNode,
            crate::lexer::token_type::JsonTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::JsonTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::JsonTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::JsonTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::JsonTokenType::Comma => Self::Comma,
            crate::lexer::token_type::JsonTokenType::Colon => Self::Colon,
            crate::lexer::token_type::JsonTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::JsonTokenType::NumberLiteral => Self::NumberLiteral,
            crate::lexer::token_type::JsonTokenType::BooleanLiteral => Self::BooleanLiteral,
            crate::lexer::token_type::JsonTokenType::NullLiteral => Self::NullLiteral,
            crate::lexer::token_type::JsonTokenType::BareKey => Self::BareKey,
            crate::lexer::token_type::JsonTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::JsonTokenType::Comment => Self::Comment,
            crate::lexer::token_type::JsonTokenType::Eof => Self::Eof,
            crate::lexer::token_type::JsonTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
