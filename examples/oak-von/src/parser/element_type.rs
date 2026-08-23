use oak_core::{ElementType, UniversalElementRole};

/// Element types for the VON (Value-Oriented Notation) parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VonElementType {
    /// Whitespace characters.
    Whitespace,
    /// Line breaks.
    Newline,
    /// Comments.
    Comment,
    /// End of file.
    Eof,
    /// An opening brace (`{`).
    LeftBrace,
    /// A closing brace (`}`).
    RightBrace,
    /// An opening bracket (`[`).
    LeftBracket,
    /// A closing bracket (`]`).
    RightBracket,
    /// A comma (`,`).
    Comma,
    /// A colon (`:`).
    Colon,
    /// An equal sign (`=`).
    Eq,
    /// A string literal.
    StringLiteral,
    /// A numeric literal.
    NumberLiteral,
    /// A boolean literal.
    BoolLiteral,
    /// A null literal.
    NullLiteral,
    /// An identifier.
    Identifier,
    /// A value element.
    Value,
    /// An object element.
    Object,
    /// An array element.
    Array,
    /// An entry in an object.
    ObjectEntry,
    /// An enum element.
    Enum,
    /// An error node in the parse tree.
    ErrorNode,
    /// An error element.
    Error,
    /// The root of the parse tree.
    Root,
    /// An element in an array.
    ArrayElement,
}

impl ElementType for VonElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,

            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::VonTokenType> for VonElementType {
    fn from(token: crate::lexer::token_type::VonTokenType) -> Self {
                match token {
            crate::lexer::token_type::VonTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::VonTokenType::Newline => Self::Newline,
            crate::lexer::token_type::VonTokenType::Comment => Self::Comment,
            crate::lexer::token_type::VonTokenType::Eof => Self::Eof,
            crate::lexer::token_type::VonTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::VonTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::VonTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::VonTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::VonTokenType::Comma => Self::Comma,
            crate::lexer::token_type::VonTokenType::Colon => Self::Colon,
            crate::lexer::token_type::VonTokenType::Eq => Self::Eq,
            crate::lexer::token_type::VonTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::VonTokenType::NumberLiteral => Self::NumberLiteral,
            crate::lexer::token_type::VonTokenType::BoolLiteral => Self::BoolLiteral,
            crate::lexer::token_type::VonTokenType::NullLiteral => Self::NullLiteral,
            crate::lexer::token_type::VonTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::VonTokenType::Value => Self::Value,
            crate::lexer::token_type::VonTokenType::Object => Self::Object,
            crate::lexer::token_type::VonTokenType::Array => Self::Array,
            crate::lexer::token_type::VonTokenType::ObjectEntry => Self::ObjectEntry,
            crate::lexer::token_type::VonTokenType::Enum => Self::Enum,
            crate::lexer::token_type::VonTokenType::ErrorNode => Self::ErrorNode,
            crate::lexer::token_type::VonTokenType::Error => Self::Error,
            crate::lexer::token_type::VonTokenType::Root => Self::Root,
            crate::lexer::token_type::VonTokenType::ArrayElement => Self::ArrayElement,
            _ => Self::Error,
        }
    }
}
