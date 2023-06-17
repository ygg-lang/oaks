use oak_core::SyntaxKind;

/// 统一Ini 语法种类（包含节点与词法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IniSyntaxKind {
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
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
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
    Equals,             // = (别名)
    Number,             // 数字
    Identifier,
    Whitespace,
    Newline,
    Comment,
    Eof,
    Error,
}

impl IniSyntaxKind {
    /// 判断是否trivia（空白或注释
    pub fn is_trivia(&self) -> bool {
        matches!(self, IniSyntaxKind::Whitespace | IniSyntaxKind::Newline | IniSyntaxKind::Comment)
    }

    /// 判断是否为值类
    pub fn is_value(self) -> bool {
        matches!(
            self,
            IniSyntaxKind::String
                | IniSyntaxKind::Integer
                | IniSyntaxKind::Float
                | IniSyntaxKind::Boolean
                | IniSyntaxKind::DateTime
                | IniSyntaxKind::Array
                | IniSyntaxKind::InlineTable
        )
    }

    /// 判断是否为字面量
    pub fn is_literal(self) -> bool {
        matches!(
            self,
            IniSyntaxKind::String
                | IniSyntaxKind::Integer
                | IniSyntaxKind::Float
                | IniSyntaxKind::Boolean
                | IniSyntaxKind::DateTime
        )
    }
}

impl core::fmt::Display for IniSyntaxKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IniSyntaxKind::Root => write!(f, "Root"),
            IniSyntaxKind::Document => write!(f, "Document"),
            IniSyntaxKind::Table => write!(f, "Table"),
            IniSyntaxKind::ArrayOfTables => write!(f, "ArrayOfTables"),
            IniSyntaxKind::KeyValue => write!(f, "KeyValue"),
            IniSyntaxKind::Key => write!(f, "Key"),
            IniSyntaxKind::Value => write!(f, "Value"),
            IniSyntaxKind::Array => write!(f, "Array"),
            IniSyntaxKind::InlineTable => write!(f, "InlineTable"),
            IniSyntaxKind::String => write!(f, "String"),
            IniSyntaxKind::Integer => write!(f, "Integer"),
            IniSyntaxKind::Float => write!(f, "Float"),
            IniSyntaxKind::Boolean => write!(f, "Boolean"),
            IniSyntaxKind::DateTime => write!(f, "DateTime"),
            IniSyntaxKind::BareKey => write!(f, "BareKey"),
            IniSyntaxKind::QuotedKey => write!(f, "QuotedKey"),
            IniSyntaxKind::Equals => write!(f, "Equals"),
            IniSyntaxKind::Number => write!(f, "Number"),
            IniSyntaxKind::ErrorNode => write!(f, "ErrorNode"),
            IniSyntaxKind::LeftBrace => write!(f, "{{"),
            IniSyntaxKind::RightBrace => write!(f, "}}"),
            IniSyntaxKind::LeftBracket => write!(f, "["),
            IniSyntaxKind::RightBracket => write!(f, "]"),
            IniSyntaxKind::DoubleLeftBracket => write!(f, "[["),
            IniSyntaxKind::DoubleRightBracket => write!(f, "]]"),
            IniSyntaxKind::Comma => write!(f, ","),
            IniSyntaxKind::Dot => write!(f, "."),
            IniSyntaxKind::Equal => write!(f, "="),
            IniSyntaxKind::Identifier => write!(f, "Identifier"),
            IniSyntaxKind::Whitespace => write!(f, "Whitespace"),
            IniSyntaxKind::Newline => write!(f, "Newline"),
            IniSyntaxKind::Comment => write!(f, "Comment"),
            IniSyntaxKind::Eof => write!(f, "EOF"),
            IniSyntaxKind::Error => write!(f, "Error"),
        }
    }
}

impl SyntaxKind for IniSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(
            self,
            Self::Root
                | Self::Document
                | Self::Table
                | Self::ArrayOfTables
                | Self::KeyValue
                | Self::Key
                | Self::BareKey
                | Self::QuotedKey
                | Self::Value
                | Self::Array
                | Self::InlineTable
                | Self::String
                | Self::Integer
                | Self::Float
                | Self::Boolean
                | Self::DateTime
                | Self::ErrorNode
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            Self::Root
                | Self::Document
                | Self::Table
                | Self::ArrayOfTables
                | Self::KeyValue
                | Self::Key
                | Self::BareKey
                | Self::QuotedKey
                | Self::Value
                | Self::Array
                | Self::InlineTable
                | Self::String
                | Self::Integer
                | Self::Float
                | Self::Boolean
                | Self::DateTime
                | Self::ErrorNode
        )
    }
}
