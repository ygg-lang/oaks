/// 统一SQL 语法种类（包含节点与词法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
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
    Comment,
    LineComment,
    BlockComment,

    // 字面
    NumberLiteral,
    FloatLiteral,
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
    AutoIncrement,
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
    View,
    Database,
    Schema,
    True,
    False,
    Exists,
    Case,
    When,
    Then,
    Else,
    End,
    If,
    Begin,
    Commit,
    Rollback,
    Transaction,

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

    // 操作符
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
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Concat,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Dot,
    Colon,
    Question,

    // 错误和结
    Error,
    Eof,
}

impl oak_core::SyntaxKind for SqlSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(
            self,
            SqlSyntaxKind::Whitespace
                | SqlSyntaxKind::Newline
                | SqlSyntaxKind::Comment
                | SqlSyntaxKind::LineComment
                | SqlSyntaxKind::BlockComment
        )
    }

    fn is_comment(&self) -> bool {
        matches!(self, SqlSyntaxKind::Comment | SqlSyntaxKind::LineComment | SqlSyntaxKind::BlockComment)
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
