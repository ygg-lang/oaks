/// 统一Stylus 语法种类（包含节点与词法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StylusSyntaxKind {
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

impl oak_core::SyntaxKind for StylusSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, StylusSyntaxKind::Whitespace | StylusSyntaxKind::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, StylusSyntaxKind::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, StylusSyntaxKind::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        matches!(
            self,
            StylusSyntaxKind::LeftBrace
                | StylusSyntaxKind::RightBrace
                | StylusSyntaxKind::LeftBracket
                | StylusSyntaxKind::RightBracket
                | StylusSyntaxKind::DoubleLeftBracket
                | StylusSyntaxKind::DoubleRightBracket
                | StylusSyntaxKind::Comma
                | StylusSyntaxKind::Dot
                | StylusSyntaxKind::Equal
                | StylusSyntaxKind::BasicString
                | StylusSyntaxKind::LiteralString
                | StylusSyntaxKind::MultilineBasicString
                | StylusSyntaxKind::MultilineLiteralString
                | StylusSyntaxKind::Integer
                | StylusSyntaxKind::Float
                | StylusSyntaxKind::Boolean
                | StylusSyntaxKind::OffsetDateTime
                | StylusSyntaxKind::LocalDateTime
                | StylusSyntaxKind::LocalDate
                | StylusSyntaxKind::LocalTime
                | StylusSyntaxKind::BareKey
                | StylusSyntaxKind::Whitespace
                | StylusSyntaxKind::Comment
                | StylusSyntaxKind::Eof
                | StylusSyntaxKind::Error
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            StylusSyntaxKind::Root
                | StylusSyntaxKind::Document
                | StylusSyntaxKind::Table
                | StylusSyntaxKind::ArrayOfTables
                | StylusSyntaxKind::KeyValue
                | StylusSyntaxKind::Key
                | StylusSyntaxKind::QuotedKey
                | StylusSyntaxKind::Value
                | StylusSyntaxKind::Array
                | StylusSyntaxKind::InlineTable
                | StylusSyntaxKind::ErrorNode
        )
    }
}

impl StylusSyntaxKind {
    /// 判断是否为值类
    pub fn is_value(self) -> bool {
        matches!(
            self,
            StylusSyntaxKind::BasicString
                | StylusSyntaxKind::LiteralString
                | StylusSyntaxKind::MultilineBasicString
                | StylusSyntaxKind::MultilineLiteralString
                | StylusSyntaxKind::Integer
                | StylusSyntaxKind::Float
                | StylusSyntaxKind::Boolean
                | StylusSyntaxKind::OffsetDateTime
                | StylusSyntaxKind::LocalDateTime
                | StylusSyntaxKind::LocalDate
                | StylusSyntaxKind::LocalTime
                | StylusSyntaxKind::Array
                | StylusSyntaxKind::InlineTable
        )
    }

    /// 判断是否为字面量
    pub fn is_literal(self) -> bool {
        matches!(
            self,
            StylusSyntaxKind::BasicString
                | StylusSyntaxKind::LiteralString
                | StylusSyntaxKind::MultilineBasicString
                | StylusSyntaxKind::MultilineLiteralString
                | StylusSyntaxKind::Integer
                | StylusSyntaxKind::Float
                | StylusSyntaxKind::Boolean
                | StylusSyntaxKind::OffsetDateTime
                | StylusSyntaxKind::LocalDateTime
                | StylusSyntaxKind::LocalDate
                | StylusSyntaxKind::LocalTime
        )
    }
}

impl core::fmt::Display for StylusSyntaxKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StylusSyntaxKind::Root => f.write_str("Root"),
            StylusSyntaxKind::Document => f.write_str("Document"),
            StylusSyntaxKind::Table => f.write_str("Table"),
            StylusSyntaxKind::ArrayOfTables => f.write_str("ArrayOfTables"),
            StylusSyntaxKind::KeyValue => f.write_str("KeyValue"),
            StylusSyntaxKind::Key => f.write_str("Key"),
            StylusSyntaxKind::Value => f.write_str("Value"),
            StylusSyntaxKind::Array => f.write_str("Array"),
            StylusSyntaxKind::InlineTable => f.write_str("InlineTable"),
            StylusSyntaxKind::BasicString => f.write_str("BasicString"),
            StylusSyntaxKind::LiteralString => f.write_str("LiteralString"),
            StylusSyntaxKind::MultilineBasicString => f.write_str("MultilineBasicString"),
            StylusSyntaxKind::MultilineLiteralString => f.write_str("MultilineLiteralString"),
            StylusSyntaxKind::Integer => f.write_str("Integer"),
            StylusSyntaxKind::Float => f.write_str("Float"),
            StylusSyntaxKind::Boolean => f.write_str("Boolean"),
            StylusSyntaxKind::OffsetDateTime => f.write_str("OffsetDateTime"),
            StylusSyntaxKind::LocalDateTime => f.write_str("LocalDateTime"),
            StylusSyntaxKind::LocalDate => f.write_str("LocalDate"),
            StylusSyntaxKind::LocalTime => f.write_str("LocalTime"),
            StylusSyntaxKind::BareKey => f.write_str("BareKey"),
            StylusSyntaxKind::QuotedKey => f.write_str("QuotedKey"),
            StylusSyntaxKind::ErrorNode => f.write_str("ErrorNode"),
            StylusSyntaxKind::LeftBrace => f.write_str("{"),
            StylusSyntaxKind::RightBrace => f.write_str("}"),
            StylusSyntaxKind::LeftBracket => f.write_str("["),
            StylusSyntaxKind::RightBracket => f.write_str("]"),
            StylusSyntaxKind::DoubleLeftBracket => f.write_str("[["),
            StylusSyntaxKind::DoubleRightBracket => f.write_str("]]"),
            StylusSyntaxKind::Comma => f.write_str(","),
            StylusSyntaxKind::Dot => f.write_str("."),
            StylusSyntaxKind::Equal => f.write_str("="),
            StylusSyntaxKind::Whitespace => f.write_str("Whitespace"),
            StylusSyntaxKind::Comment => f.write_str("Comment"),
            StylusSyntaxKind::Eof => f.write_str("EOF"),
            StylusSyntaxKind::Error => f.write_str("Error"),
        }
    }
}
