use oak_core::{TokenType, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum CobolTokenType {
    // 基础 tokens
    Whitespace = 0,
    Newline,
    Comment,

    // 标识符和字面量
    Identifier,
    StringLiteral,
    NumberLiteral,
    PictureLiteral,

    // COBOL 关键字 - 数据定义
    Accept,
    Add,
    Call,
    Cancel,
    Close,
    Compute,
    Continue,
    Delete,
    Display,
    Divide,
    Evaluate,
    Exit,
    GoTo,
    If,
    Initialize,
    Inspect,
    Move,
    Multiply,
    Open,
    Perform,
    Read,
    Return,
    Rewrite,
    Search,
    Set,
    Sort,
    Start,
    Stop,
    String,
    Subtract,
    Unstring,
    Write,

    // 数据部门关键字
    Data,
    Division,
    Section,
    WorkingStorage,
    FileSection,
    LinkageSection,
    LocalStorageSection,
    LowValue,
    HighValue,
    Zero,
    Space,
    QuoteLiteral,

    // 程序结构关键字
    Identification,
    Program,
    Environment,
    Configuration,
    InputOutput,
    File,
    Procedure,

    // 数据类型和级别
    Pic,
    Picture,
    Value,
    Occurs,
    Redefines,
    Usage,
    Comp,
    Comp1,
    Comp2,
    Comp3,
    Comp4,
    Comp5,
    Binary,
    Packed,
    Display_,

    // 文件操作关键字
    Select,
    Assign,
    Organization,
    Access,
    Record,
    Key,
    Status,
    Sequential,
    Random,
    Dynamic,
    Indexed,
    Relative,

    // 条件和控制流
    When,
    Other,
    Also,
    Through,
    Thru,
    Until,
    Varying,
    From,
    By,
    After,
    Before,

    // 逻辑操作
    And,
    Or,
    Not,
    Equal,
    Greater,
    Less,

    // 算术操作符
    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /
    Power, // **

    // 比较操作符
    EqualSign,    // =
    GreaterThan,  // >
    LessThan,     // <
    GreaterEqual, // >=
    LessEqual,    // <=
    NotEqual,     // <>

    // 分隔符
    LeftParen,  // (
    RightParen, // )
    Comma,      // ,
    Period,     // .
    Semicolon,  // ;
    Colon,      // :
    Quote,      // "
    Apostrophe, // '

    // 特殊字符
    At,        // @
    Hash,      // #
    Dollar,    // $
    Ampersand, // &

    Text,
    Error,
    Eof,
}

impl TokenType for CobolTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::StringLiteral | Self::NumberLiteral | Self::PictureLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Power
            | Self::EqualSign
            | Self::GreaterThan
            | Self::LessThan
            | Self::GreaterEqual
            | Self::LessEqual
            | Self::NotEqual
            | Self::And
            | Self::Or
            | Self::Not
            | Self::Equal
            | Self::Greater
            | Self::Less => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::Comma | Self::Period | Self::Semicolon | Self::Colon | Self::Quote | Self::Apostrophe | Self::At | Self::Hash | Self::Dollar | Self::Ampersand => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::None,
            _ => UniversalTokenRole::Keyword,
        }
    }
}
