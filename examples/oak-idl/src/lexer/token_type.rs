use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IdlTokenType {
    // 空白和换行
    Whitespace = 0,
    Newline,
    Comment,

    // 字面量
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    CharLiteral,

    // 标识符
    Identifier,

    // 基本数据类型
    Void,
    Boolean,
    Byte,
    Octet,
    Short,
    UnsignedShort,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Float,
    Double,
    LongDouble,
    Char,
    WChar,
    String,
    WString,
    Any,
    Object,
    ValueBase,

    // 复合类型关键字
    Struct,
    Union,
    Enum,
    Interface,
    Module,
    Exception,
    Typedef,
    Sequence,
    Array,
    Fixed,

    // 修饰符
    Const,
    Readonly,
    Attribute,
    Oneway,
    In,
    Out,
    Inout,
    Raises,
    Context,
    Local,
    Abstract,
    Custom,
    Private,
    Public,
    Truncatable,
    Supports,
    ValueType,
    Native,
    Factory,

    // 预处理器指令
    Include,
    Pragma,
    Define,
    Ifdef,
    Ifndef,
    Endif,
    Else,
    Elif,
    Undef,
    Hash,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftAngle,
    RightAngle,
    Comma,
    Semicolon,
    Colon,
    DoubleColon,
    Dot,
    Arrow,

    // 操作符
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    LeftShift,
    RightShift,

    // 复合节点
    SourceFile,
    Error,
    Eof,
}

impl TokenType for IdlTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::StringLiteral | Self::NumberLiteral | Self::BooleanLiteral | Self::CharLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Void
            | Self::Boolean
            | Self::Byte
            | Self::Octet
            | Self::Short
            | Self::UnsignedShort
            | Self::Long
            | Self::UnsignedLong
            | Self::LongLong
            | Self::UnsignedLongLong
            | Self::Float
            | Self::Double
            | Self::LongDouble
            | Self::Char
            | Self::WChar
            | Self::String
            | Self::WString
            | Self::Any
            | Self::Object
            | Self::ValueBase
            | Self::Struct
            | Self::Union
            | Self::Enum
            | Self::Interface
            | Self::Module
            | Self::Exception
            | Self::Typedef
            | Self::Sequence
            | Self::Array
            | Self::Fixed
            | Self::Const
            | Self::Readonly
            | Self::Attribute
            | Self::Oneway
            | Self::In
            | Self::Out
            | Self::Inout
            | Self::Raises
            | Self::Context
            | Self::Local
            | Self::Abstract
            | Self::Custom
            | Self::Private
            | Self::Public
            | Self::Truncatable
            | Self::Supports
            | Self::ValueType
            | Self::Native
            | Self::Factory => UniversalTokenRole::Keyword,
            Self::Include | Self::Pragma | Self::Define | Self::Ifdef | Self::Ifndef | Self::Endif | Self::Else | Self::Elif | Self::Undef | Self::Hash => UniversalTokenRole::Keyword,
            Self::LeftParen
            | Self::RightParen
            | Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftAngle
            | Self::RightAngle
            | Self::Comma
            | Self::Semicolon
            | Self::Colon
            | Self::DoubleColon
            | Self::Dot
            | Self::Arrow => UniversalTokenRole::Punctuation,
            Self::Assign
            | Self::Plus
            | Self::Minus
            | Self::Multiply
            | Self::Divide
            | Self::Modulo
            | Self::BitwiseAnd
            | Self::BitwiseOr
            | Self::BitwiseXor
            | Self::BitwiseNot
            | Self::LogicalAnd
            | Self::LogicalOr
            | Self::LogicalNot
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::Equal
            | Self::NotEqual
            | Self::LeftShift
            | Self::RightShift => UniversalTokenRole::Operator,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

pub type IdlToken = Token<IdlTokenType>;
