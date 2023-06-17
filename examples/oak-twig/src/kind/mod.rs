use oak_core::Token;

/// TOML tokens
pub type TomlToken = Token<TomlTokenKind>;

/// TOML tokens 种类（亦作为语法节点种类
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TomlTokenKind {
    // 语法节点（red/green 根）
    Document,

    // TOML 值类
    String,
    Integer,
    Float,
    Boolean,
    DateTime,

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

    // 标识符和
    Identifier,

    // 空白和注
    Whitespace,
    Comment,

    // 特殊
    Eof,
    Invalid,
}

impl core::fmt::Display for TomlTokenKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TomlTokenKind::Document => write!(f, "Document"),
            TomlTokenKind::String => write!(f, "String"),
            TomlTokenKind::Integer => write!(f, "Integer"),
            TomlTokenKind::Float => write!(f, "Float"),
            TomlTokenKind::Boolean => write!(f, "Boolean"),
            TomlTokenKind::DateTime => write!(f, "DateTime"),
            TomlTokenKind::LeftBrace => write!(f, "{{"),
            TomlTokenKind::RightBrace => write!(f, "}}"),
            TomlTokenKind::LeftBracket => write!(f, "["),
            TomlTokenKind::RightBracket => write!(f, "]"),
            TomlTokenKind::Comma => write!(f, ","),
            TomlTokenKind::Dot => write!(f, "."),
            TomlTokenKind::Equal => write!(f, "="),
            TomlTokenKind::DoubleLeftBracket => write!(f, "[["),
            TomlTokenKind::DoubleRightBracket => write!(f, "]]"),
            TomlTokenKind::Identifier => write!(f, "Identifier"),
            TomlTokenKind::Whitespace => write!(f, "Whitespace"),
            TomlTokenKind::Comment => write!(f, "Comment"),
            TomlTokenKind::Eof => write!(f, "EOF"),
            TomlTokenKind::Invalid => write!(f, "Invalid"),
        }
    }
}
