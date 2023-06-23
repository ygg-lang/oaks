use oak_core::{Token, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type SqlToken = Token<SqlTokenType>;

impl oak_core::TokenType for SqlTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, SqlTokenType::Whitespace | SqlTokenType::Newline | SqlTokenType::Comment | SqlTokenType::LineComment | SqlTokenType::BlockComment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, SqlTokenType::Comment | SqlTokenType::LineComment | SqlTokenType::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, SqlTokenType::Whitespace | SqlTokenType::Newline)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            Self::Whitespace | Self::Newline => Whitespace,
            Self::Comment | Self::LineComment | Self::BlockComment => Comment,
            Self::NumberLiteral | Self::FloatLiteral | Self::StringLiteral | Self::BooleanLiteral | Self::NullLiteral => Literal,
            Self::Identifier_ => Name,
            Self::Select
            | Self::From
            | Self::Where
            | Self::Insert
            | Self::Into
            | Self::Values
            | Self::Update
            | Self::Set
            | Self::Delete
            | Self::Create
            | Self::Table
            | Self::Drop
            | Self::Alter
            | Self::Add
            | Self::Column
            | Self::Primary
            | Self::Key
            | Self::Foreign
            | Self::References
            | Self::Index
            | Self::Unique
            | Self::Not
            | Self::Null
            | Self::Default
            | Self::AutoIncrement
            | Self::And
            | Self::Or
            | Self::In
            | Self::Like
            | Self::Between
            | Self::Is
            | Self::As
            | Self::Join
            | Self::Inner
            | Self::Left
            | Self::Right
            | Self::Full
            | Self::Outer
            | Self::On
            | Self::Group
            | Self::By
            | Self::Having
            | Self::Order
            | Self::Asc
            | Self::Desc
            | Self::Limit
            | Self::Offset
            | Self::Union
            | Self::All
            | Self::Distinct
            | Self::Count
            | Self::Sum
            | Self::Avg
            | Self::Min
            | Self::Max
            | Self::View
            | Self::Database
            | Self::Schema
            | Self::True
            | Self::False
            | Self::Exists
            | Self::Case
            | Self::When
            | Self::Then
            | Self::Else
            | Self::End
            | Self::If
            | Self::Begin
            | Self::Commit
            | Self::Rollback
            | Self::Transaction => Keyword,
            Self::Int | Self::Integer | Self::Varchar | Self::Char | Self::Text | Self::Date | Self::Time | Self::Timestamp | Self::Decimal | Self::Float | Self::Double | Self::Boolean => Keyword, // Types are often keywords
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::Assign
            | Self::Eq
            | Self::Ne
            | Self::Lt
            | Self::Le
            | Self::Gt
            | Self::Ge
            | Self::Concat => Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Comma | Self::Semicolon | Self::Dot | Self::Colon | Self::Question => Punctuation,
            Self::Error => Error,
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SqlTokenType {
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
    JoinClause,
    GroupByClause,
    HavingClause,
    OrderByClause,
    LimitClause,
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
