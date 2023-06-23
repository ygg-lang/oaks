use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IniElementType {
    // Basic kinds (mirrored from tokens for convenience)
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,

    // Tokens (mirrored)
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    DoubleLeftBracket,
    DoubleRightBracket,
    Comma,
    Dot,
    Equal,

    // Values (mirrored)
    Identifier,
    String,
    Integer,
    Float,
    Boolean,
    DateTime,

    // Structures (Nodes)
    Root,
    Document,
    Section,
    Table,
    ArrayOfTables,
    KeyValue,
    Key,
    Value,
    Array,
    InlineTable,
}

impl ElementType for IniElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::Document => UniversalElementRole::Root,
            Self::Section | Self::Table | Self::ArrayOfTables | Self::Array | Self::InlineTable => UniversalElementRole::Container,
            Self::KeyValue => UniversalElementRole::Attribute,
            Self::Key => UniversalElementRole::AttributeKey,
            Self::Value => UniversalElementRole::Value,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::IniTokenType> for IniElementType {
    fn from(token: crate::lexer::token_type::IniTokenType) -> Self {
        match token {
            crate::lexer::token_type::IniTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::IniTokenType::Newline => Self::Newline,
            crate::lexer::token_type::IniTokenType::Comment => Self::Comment,
            crate::lexer::token_type::IniTokenType::Error => Self::Error,
            crate::lexer::token_type::IniTokenType::Eof => Self::Eof,
            crate::lexer::token_type::IniTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::IniTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::IniTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::IniTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::IniTokenType::DoubleLeftBracket => Self::DoubleLeftBracket,
            crate::lexer::token_type::IniTokenType::DoubleRightBracket => Self::DoubleRightBracket,
            crate::lexer::token_type::IniTokenType::Comma => Self::Comma,
            crate::lexer::token_type::IniTokenType::Dot => Self::Dot,
            crate::lexer::token_type::IniTokenType::Equal => Self::Equal,
            crate::lexer::token_type::IniTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::IniTokenType::String => Self::String,
            crate::lexer::token_type::IniTokenType::Integer => Self::Integer,
            crate::lexer::token_type::IniTokenType::Float => Self::Float,
            crate::lexer::token_type::IniTokenType::Boolean => Self::Boolean,
            crate::lexer::token_type::IniTokenType::DateTime => Self::DateTime,
        }
    }
}
