use oak_core::{ElementType, UniversalElementRole};

/// INI element types used in the syntax tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IniElementType {
    /// Whitespace characters.
    Whitespace,
    /// Newline character.
    Newline,
    /// Comment.
    Comment,
    /// Error element.
    Error,
    /// End of file.
    Eof,

    /// `{` symbol.
    LeftBrace,
    /// `}` symbol.
    RightBrace,
    /// `[` symbol.
    LeftBracket,
    /// `]` symbol.
    RightBracket,
    /// `[[` symbol.
    DoubleLeftBracket,
    /// `]]` symbol.
    DoubleRightBracket,
    /// `,` symbol.
    Comma,
    /// `.` symbol.
    Dot,
    /// `=` operator.
    Equal,

    /// Identifier.
    Identifier,
    /// String literal.
    String,
    /// Integer literal.
    Integer,
    /// Floating-point literal.
    Float,
    /// Boolean literal.
    Boolean,
    /// Date and time literal.
    DateTime,

    /// Root of the INI document.
    Root,
    /// The entire INI document.
    Document,
    /// A section in the INI file.
    Section,
    /// A table definition.
    Table,
    /// An array of tables definition.
    ArrayOfTables,
    /// A key-value pair.
    KeyValue,
    /// The key part of a key-value pair.
    Key,
    /// The value part of a key-value pair.
    Value,
    /// An array value.
    Array,
    /// An inline table value.
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
