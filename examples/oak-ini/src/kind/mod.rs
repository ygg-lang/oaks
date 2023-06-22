use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IniSyntaxKind {
    // Basic kinds
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,

    // Structures
    Root,
    Document,
    Table,
    ArrayOfTables,
    Array,
    InlineTable,
    KeyValue,
    Key,
    Value,

    // Tokens
    LeftBrace,          // {
    RightBrace,         // }
    LeftBracket,        // [
    RightBracket,       // ]
    DoubleLeftBracket,  // [[
    DoubleRightBracket, // ]]
    Comma,              // ,
    Dot,                // .
    Equal,              // =

    // Values
    Identifier,
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
}

impl IniSyntaxKind {
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

impl TokenType for IniSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::DoubleLeftBracket | Self::DoubleRightBracket | Self::Comma | Self::Dot | Self::Equal => UniversalTokenRole::Punctuation,
            Self::Identifier => UniversalTokenRole::Name,
            Self::String | Self::Integer | Self::Float | Self::Boolean | Self::DateTime => UniversalTokenRole::Literal,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for IniSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::Document => UniversalElementRole::Root,
            Self::Table | Self::ArrayOfTables | Self::Array | Self::InlineTable => UniversalElementRole::Container,
            Self::KeyValue => UniversalElementRole::Statement,
            Self::Key => UniversalElementRole::Name,
            Self::Value => UniversalElementRole::Value,
            Self::String | Self::Integer | Self::Float | Self::Boolean | Self::DateTime => UniversalElementRole::Value,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::Root | Self::Document)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}
