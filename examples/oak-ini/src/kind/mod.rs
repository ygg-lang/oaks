use oak_core::Token;

/// Ini tokens
pub type IniToken = Token<IniTokenKind>;

/// Ini tokens 种类（亦作为语法节点种类
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum IniTokenKind {
    // 语法节点（red/green 根）
    Document,

    // Ini 值类
    String,
    Integer,
    Float,
    Boolean,
    DateTime,

    // Ini 结构符号
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Dot,          // .
    Equal,        // =

    // Ini 特殊符号
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

impl core::fmt::Display for IniTokenKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IniTokenKind::Document => write!(f, "Document"),
            IniTokenKind::String => write!(f, "String"),
            IniTokenKind::Integer => write!(f, "Integer"),
            IniTokenKind::Float => write!(f, "Float"),
            IniTokenKind::Boolean => write!(f, "Boolean"),
            IniTokenKind::DateTime => write!(f, "DateTime"),
            IniTokenKind::LeftBrace => write!(f, "{{"),
            IniTokenKind::RightBrace => write!(f, "}}"),
            IniTokenKind::LeftBracket => write!(f, "["),
            IniTokenKind::RightBracket => write!(f, "]"),
            IniTokenKind::Comma => write!(f, ","),
            IniTokenKind::Dot => write!(f, "."),
            IniTokenKind::Equal => write!(f, "="),
            IniTokenKind::DoubleLeftBracket => write!(f, "[["),
            IniTokenKind::DoubleRightBracket => write!(f, "]]"),
            IniTokenKind::Identifier => write!(f, "Identifier"),
            IniTokenKind::Whitespace => write!(f, "Whitespace"),
            IniTokenKind::Comment => write!(f, "Comment"),
            IniTokenKind::Eof => write!(f, "EOF"),
            IniTokenKind::Invalid => write!(f, "Invalid"),
        }
    }
}
