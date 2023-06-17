/// 统一TOML 语法种类（包含节点与词法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TomlSyntaxKind {
    // 节点种类
    Root,
    Document,
    Table,
    ArrayOfTables,
    KeyValue,
    Key,
    BareKey,
    QuotedKey,
    Value,
    Array,
    InlineTable,

    // 细分字面量类
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
    ErrorNode,

    // 词法种类
    LeftBrace,          // {
    RightBrace,         // }
    LeftBracket,        // [
    RightBracket,       // ]
    DoubleLeftBracket,  // [[
    DoubleRightBracket, // ]]
    Comma,              // ,
    Dot,                // .
    Equal,              // =
    Whitespace,
    Comment,
    Eof,
    Error,
}

impl oak_core::SyntaxKind for TomlSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, TomlSyntaxKind::Whitespace | TomlSyntaxKind::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, TomlSyntaxKind::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, TomlSyntaxKind::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        matches!(
            self,
            TomlSyntaxKind::LeftBrace
                | TomlSyntaxKind::RightBrace
                | TomlSyntaxKind::LeftBracket
                | TomlSyntaxKind::RightBracket
                | TomlSyntaxKind::DoubleLeftBracket
                | TomlSyntaxKind::DoubleRightBracket
                | TomlSyntaxKind::Comma
                | TomlSyntaxKind::Dot
                | TomlSyntaxKind::Equal
                | TomlSyntaxKind::BasicString
                | TomlSyntaxKind::LiteralString
                | TomlSyntaxKind::MultilineBasicString
                | TomlSyntaxKind::MultilineLiteralString
                | TomlSyntaxKind::Integer
                | TomlSyntaxKind::Float
                | TomlSyntaxKind::Boolean
                | TomlSyntaxKind::OffsetDateTime
                | TomlSyntaxKind::LocalDateTime
                | TomlSyntaxKind::LocalDate
                | TomlSyntaxKind::LocalTime
                | TomlSyntaxKind::BareKey
                | TomlSyntaxKind::Whitespace
                | TomlSyntaxKind::Comment
                | TomlSyntaxKind::Eof
                | TomlSyntaxKind::Error
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            TomlSyntaxKind::Root
                | TomlSyntaxKind::Document
                | TomlSyntaxKind::Table
                | TomlSyntaxKind::ArrayOfTables
                | TomlSyntaxKind::KeyValue
                | TomlSyntaxKind::Key
                | TomlSyntaxKind::QuotedKey
                | TomlSyntaxKind::Value
                | TomlSyntaxKind::Array
                | TomlSyntaxKind::InlineTable
                | TomlSyntaxKind::ErrorNode
        )
    }
}

impl TomlSyntaxKind {
    /// 判断是否为值类
    pub fn is_value(self) -> bool {
        matches!(
            self,
            TomlSyntaxKind::BasicString
                | TomlSyntaxKind::LiteralString
                | TomlSyntaxKind::MultilineBasicString
                | TomlSyntaxKind::MultilineLiteralString
                | TomlSyntaxKind::Integer
                | TomlSyntaxKind::Float
                | TomlSyntaxKind::Boolean
                | TomlSyntaxKind::OffsetDateTime
                | TomlSyntaxKind::LocalDateTime
                | TomlSyntaxKind::LocalDate
                | TomlSyntaxKind::LocalTime
                | TomlSyntaxKind::Array
                | TomlSyntaxKind::InlineTable
        )
    }

    /// 判断是否为字面量
    pub fn is_literal(self) -> bool {
        matches!(
            self,
            TomlSyntaxKind::BasicString
                | TomlSyntaxKind::LiteralString
                | TomlSyntaxKind::MultilineBasicString
                | TomlSyntaxKind::MultilineLiteralString
                | TomlSyntaxKind::Integer
                | TomlSyntaxKind::Float
                | TomlSyntaxKind::Boolean
                | TomlSyntaxKind::OffsetDateTime
                | TomlSyntaxKind::LocalDateTime
                | TomlSyntaxKind::LocalDate
                | TomlSyntaxKind::LocalTime
        )
    }
}

impl core::fmt::Display for TomlSyntaxKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TomlSyntaxKind::Root => f.write_str("Root"),
            TomlSyntaxKind::Document => f.write_str("Document"),
            TomlSyntaxKind::Table => f.write_str("Table"),
            TomlSyntaxKind::ArrayOfTables => f.write_str("ArrayOfTables"),
            TomlSyntaxKind::KeyValue => f.write_str("KeyValue"),
            TomlSyntaxKind::Key => f.write_str("Key"),
            TomlSyntaxKind::Value => f.write_str("Value"),
            TomlSyntaxKind::Array => f.write_str("Array"),
            TomlSyntaxKind::InlineTable => f.write_str("InlineTable"),
            TomlSyntaxKind::BasicString => f.write_str("BasicString"),
            TomlSyntaxKind::LiteralString => f.write_str("LiteralString"),
            TomlSyntaxKind::MultilineBasicString => f.write_str("MultilineBasicString"),
            TomlSyntaxKind::MultilineLiteralString => f.write_str("MultilineLiteralString"),
            TomlSyntaxKind::Integer => f.write_str("Integer"),
            TomlSyntaxKind::Float => f.write_str("Float"),
            TomlSyntaxKind::Boolean => f.write_str("Boolean"),
            TomlSyntaxKind::OffsetDateTime => f.write_str("OffsetDateTime"),
            TomlSyntaxKind::LocalDateTime => f.write_str("LocalDateTime"),
            TomlSyntaxKind::LocalDate => f.write_str("LocalDate"),
            TomlSyntaxKind::LocalTime => f.write_str("LocalTime"),
            TomlSyntaxKind::BareKey => f.write_str("BareKey"),
            TomlSyntaxKind::QuotedKey => f.write_str("QuotedKey"),
            TomlSyntaxKind::ErrorNode => f.write_str("ErrorNode"),
            TomlSyntaxKind::LeftBrace => f.write_str("{"),
            TomlSyntaxKind::RightBrace => f.write_str("}"),
            TomlSyntaxKind::LeftBracket => f.write_str("["),
            TomlSyntaxKind::RightBracket => f.write_str("]"),
            TomlSyntaxKind::DoubleLeftBracket => f.write_str("[["),
            TomlSyntaxKind::DoubleRightBracket => f.write_str("]]"),
            TomlSyntaxKind::Comma => f.write_str(","),
            TomlSyntaxKind::Dot => f.write_str("."),
            TomlSyntaxKind::Equal => f.write_str("="),
            TomlSyntaxKind::Whitespace => f.write_str("Whitespace"),
            TomlSyntaxKind::Comment => f.write_str("Comment"),
            TomlSyntaxKind::Eof => f.write_str("EOF"),
            TomlSyntaxKind::Error => f.write_str("Error"),
        }
    }
}
