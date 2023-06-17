/// 统一SQL 语法种类（包含节点与词法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SqlSyntaxKind {
    // 节点种类
    Root,
    Statement,
    SelectStatement,
    InsertStatement,
    UpdateStatement,
    DeleteStatement,
    CreateStatement,
    DropStatement,
    AlterStatement,
    Expression,
    Identifier,
    TableName,
    ColumnName,
    ErrorNode,

    // 空白字符和换
    Whitespace,
    Newline,

    // 注释
    LineComment,
    BlockComment,

    // 字面
    NumberLiteral,
    StringLiteral,
    BooleanLiteral,
    NullLiteral,

    // 标识
    Identifier_,

    // SQL 关键
    Select,
    From,
    Where,
    Insert,
    Into,
    Values,
    Update,
    Set,
    Delete,
    Create,
    Table,
    Drop,
    Alter,
    Add,
    Column,
    Primary,
    Key,
    Foreign,
    References,
    Index,
    Unique,
    Not,
    Null,
    Default,
    Auto_Increment,
    And,
    Or,
    In,
    Like,
    Between,
    Is,
    As,
    Join,
    Inner,
    Left,
    Right,
    Full,
    Outer,
    On,
    Group,
    By,
    Having,
    Order,
    Asc,
    Desc,
    Limit,
    Offset,
    Union,
    All,
    Distinct,
    Count,
    Sum,
    Avg,
    Min,
    Max,

    // 数据类型
    Int,
    Integer,
    Varchar,
    Char,
    Text,
    Date,
    Time,
    Timestamp,
    Decimal,
    Float,
    Double,
    Boolean,

    // 操作
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Assign,

    // 分隔
    LeftParen,
    RightParen,
    Comma,
    Semicolon,
    Dot,

    // 错误和结
    Error,
    Eof,
}

impl oak_core::SyntaxKind for SqlSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(
            self,
            SqlSyntaxKind::Whitespace | SqlSyntaxKind::Newline | SqlSyntaxKind::LineComment | SqlSyntaxKind::BlockComment
        )
    }

    fn is_comment(&self) -> bool {
        matches!(self, SqlSyntaxKind::LineComment | SqlSyntaxKind::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, SqlSyntaxKind::Whitespace | SqlSyntaxKind::Newline)
    }

    fn is_token_type(&self) -> bool {
        !self.is_element_type()
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            SqlSyntaxKind::Root
                | SqlSyntaxKind::Statement
                | SqlSyntaxKind::SelectStatement
                | SqlSyntaxKind::InsertStatement
                | SqlSyntaxKind::UpdateStatement
                | SqlSyntaxKind::DeleteStatement
                | SqlSyntaxKind::CreateStatement
                | SqlSyntaxKind::DropStatement
                | SqlSyntaxKind::AlterStatement
                | SqlSyntaxKind::Expression
                | SqlSyntaxKind::Identifier
                | SqlSyntaxKind::TableName
                | SqlSyntaxKind::ColumnName
                | SqlSyntaxKind::ErrorNode
        )
    }
}
