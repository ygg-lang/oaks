use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};

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

impl TokenType for TomlTokenKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            TomlTokenKind::Whitespace => UniversalTokenRole::Whitespace,
            TomlTokenKind::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
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

impl ElementType for TomlTokenKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            TomlTokenKind::Invalid | TomlTokenKind::ErrorNode => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
