use oak_core::{SyntaxKind, Token};

/// TOML tokens
pub type TomlToken = Token<TomlTokenKind>;

/// TOML kind kind (alias for TomlTokenKind)
pub type TomlSyntaxKind = TomlTokenKind;

/// TOML tokens 种类（亦作为语法节点种类）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum TomlTokenKind {
    // 语法节点（red/green 根）
    Document,
    Root,

    // TOML 值类型
    BasicString,
    LiteralString,
    MultilineBasicString,
    MultilineLiteralString,
    Integer,
    Float,
    Boolean,
    OffsetDateTime,
    LocalDateTime,
    LocalDate,
    LocalTime,

    // TOML 结构符号
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Dot,          // .
    Equal,        // =

    // TOML 特殊符号
    DoubleLeftBracket,  // [[
    DoubleRightBracket, // ]]

    // 标识符和键
    BareKey,
    QuotedKey,

    // 语法节点
    Key,
    Value,
    KeyValue,
    Table,
    ArrayOfTables,
    Array,
    InlineTable,
    ErrorNode,

    // 空白和注释
    Whitespace,
    Comment,

    // 特殊
    Eof,
    Invalid,
}

impl SyntaxKind for TomlTokenKind {
    fn is_trivia(&self) -> bool {
        matches!(self, TomlTokenKind::Whitespace | TomlTokenKind::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, TomlTokenKind::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, TomlTokenKind::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        matches!(
            self,
            TomlTokenKind::BasicString
                | TomlTokenKind::LiteralString
                | TomlTokenKind::MultilineBasicString
                | TomlTokenKind::MultilineLiteralString
                | TomlTokenKind::Integer
                | TomlTokenKind::Float
                | TomlTokenKind::Boolean
                | TomlTokenKind::OffsetDateTime
                | TomlTokenKind::LocalDateTime
                | TomlTokenKind::LocalDate
                | TomlTokenKind::LocalTime
                | TomlTokenKind::LeftBrace
                | TomlTokenKind::RightBrace
                | TomlTokenKind::LeftBracket
                | TomlTokenKind::RightBracket
                | TomlTokenKind::Comma
                | TomlTokenKind::Dot
                | TomlTokenKind::Equal
                | TomlTokenKind::DoubleLeftBracket
                | TomlTokenKind::DoubleRightBracket
                | TomlTokenKind::BareKey
                | TomlTokenKind::QuotedKey
                | TomlTokenKind::Whitespace
                | TomlTokenKind::Comment
                | TomlTokenKind::Eof
                | TomlTokenKind::Invalid
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            TomlTokenKind::Document
                | TomlTokenKind::Root
                | TomlTokenKind::Key
                | TomlTokenKind::Value
                | TomlTokenKind::KeyValue
                | TomlTokenKind::Table
                | TomlTokenKind::ArrayOfTables
                | TomlTokenKind::Array
                | TomlTokenKind::InlineTable
                | TomlTokenKind::ErrorNode
        )
    }
}

impl core::fmt::Display for TomlTokenKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TomlTokenKind::Document => write!(f, "Document"),
            TomlTokenKind::Root => write!(f, "Root"),
            TomlTokenKind::BasicString => write!(f, "BasicString"),
            TomlTokenKind::LiteralString => write!(f, "LiteralString"),
            TomlTokenKind::MultilineBasicString => write!(f, "MultilineBasicString"),
            TomlTokenKind::MultilineLiteralString => write!(f, "MultilineLiteralString"),
            TomlTokenKind::Integer => write!(f, "Integer"),
            TomlTokenKind::Float => write!(f, "Float"),
            TomlTokenKind::Boolean => write!(f, "Boolean"),
            TomlTokenKind::OffsetDateTime => write!(f, "OffsetDateTime"),
            TomlTokenKind::LocalDateTime => write!(f, "LocalDateTime"),
            TomlTokenKind::LocalDate => write!(f, "LocalDate"),
            TomlTokenKind::LocalTime => write!(f, "LocalTime"),
            TomlTokenKind::LeftBrace => write!(f, "{{"),
            TomlTokenKind::RightBrace => write!(f, "}}"),
            TomlTokenKind::LeftBracket => write!(f, "["),
            TomlTokenKind::RightBracket => write!(f, "]"),
            TomlTokenKind::Comma => write!(f, ","),
            TomlTokenKind::Dot => write!(f, "."),
            TomlTokenKind::Equal => write!(f, "="),
            TomlTokenKind::DoubleLeftBracket => write!(f, "[["),
            TomlTokenKind::DoubleRightBracket => write!(f, "]]"),
            TomlTokenKind::BareKey => write!(f, "BareKey"),
            TomlTokenKind::QuotedKey => write!(f, "QuotedKey"),
            TomlTokenKind::Key => write!(f, "Key"),
            TomlTokenKind::Value => write!(f, "Value"),
            TomlTokenKind::KeyValue => write!(f, "KeyValue"),
            TomlTokenKind::Table => write!(f, "Table"),
            TomlTokenKind::ArrayOfTables => write!(f, "ArrayOfTables"),
            TomlTokenKind::Array => write!(f, "Array"),
            TomlTokenKind::InlineTable => write!(f, "InlineTable"),
            TomlTokenKind::ErrorNode => write!(f, "ErrorNode"),
            TomlTokenKind::Whitespace => write!(f, "Whitespace"),
            TomlTokenKind::Comment => write!(f, "Comment"),
            TomlTokenKind::Eof => write!(f, "EOF"),
            TomlTokenKind::Invalid => write!(f, "Invalid"),
        }
    }
}
