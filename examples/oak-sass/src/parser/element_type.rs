use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Sass language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SassElementType {
    /// A source file.
    SourceFile,
    /// The root node.
    Root,
    /// A selector.
    Selector,
    /// A variable declaration.
    Variable,
    /// A mixin declaration.
    Mixin,
    /// A function declaration.
    Function,
    /// A value.
    Value,
    /// An object.
    Object,
    /// An array.
    Array,
    /// A string literal.
    String,
    /// A numeric literal.
    Number,
    /// A boolean literal.
    Boolean,
    /// A null literal.
    Null,
    /// An entry in an object.
    ObjectEntry,
    /// An element in an array.
    ArrayElement,
    /// An error node.
    ErrorNode,
    /// A left brace `{`.
    LeftBrace,
    /// A right brace `}`.
    RightBrace,
    /// A left bracket `[`.
    LeftBracket,
    /// A right bracket `]`.
    RightBracket,
    /// A comma `,`.
    Comma,
    /// A colon `:`.
    Colon,
    /// Whitespace.
    Whitespace,
    /// A comment.
    Comment,
    /// End of file marker.
    Eof,
    /// Syntax error.
    Error,
}

impl ElementType for SassElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile | Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::SassTokenType> for SassElementType {
    fn from(token: crate::lexer::token_type::SassTokenType) -> Self {
        use crate::lexer::token_type::SassTokenType as T;
        match token {
            T::Whitespace => Self::Whitespace,
            T::LineComment | T::BlockComment => Self::Comment,
            T::Eof => Self::Eof,
            T::Error => Self::Error,
            T::LeftBrace => Self::LeftBrace,
            T::RightBrace => Self::RightBrace,
            T::LeftBracket => Self::LeftBracket,
            T::RightBracket => Self::RightBracket,
            T::Comma => Self::Comma,
            T::Colon => Self::Colon,
            _ => Self::Error, // Fallback for now
        }
    }
}
