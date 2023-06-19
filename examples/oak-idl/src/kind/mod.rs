use oak_core::{SyntaxKind, Token};
use serde::{Deserialize, Serialize};

pub type IdlToken = Token<IdlSyntaxKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IdlSyntaxKind {
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

    // 运算符
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Assign,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    LeftAngle,
    RightAngle,
    Semicolon,
    Comma,
    Colon,
    DoubleColon,
    Dot,
    Arrow,
    Hash,

    // 复合节点
    SourceFile,
    Error,
    Eof,
}

impl SyntaxKind for IdlSyntaxKind {
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
        matches!(
            self,
            Self::Whitespace
                | Self::Newline
                | Self::Comment
                | Self::StringLiteral
                | Self::NumberLiteral
                | Self::BooleanLiteral
                | Self::CharLiteral
                | Self::Identifier
                | Self::Void
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
                | Self::Factory
                | Self::Include
                | Self::Pragma
                | Self::Define
                | Self::Ifdef
                | Self::Ifndef
                | Self::Endif
                | Self::Else
                | Self::Elif
                | Self::Undef
                | Self::Plus
                | Self::Minus
                | Self::Multiply
                | Self::Divide
                | Self::Modulo
                | Self::BitwiseAnd
                | Self::BitwiseOr
                | Self::BitwiseXor
                | Self::BitwiseNot
                | Self::LeftShift
                | Self::RightShift
                | Self::LogicalAnd
                | Self::LogicalOr
                | Self::LogicalNot
                | Self::Equal
                | Self::NotEqual
                | Self::Less
                | Self::Greater
                | Self::LessEqual
                | Self::GreaterEqual
                | Self::Assign
                | Self::LeftParen
                | Self::RightParen
                | Self::LeftBracket
                | Self::RightBracket
                | Self::LeftBrace
                | Self::RightBrace
                | Self::LeftAngle
                | Self::RightAngle
                | Self::Semicolon
                | Self::Comma
                | Self::Colon
                | Self::DoubleColon
                | Self::Dot
                | Self::Arrow
                | Self::Hash
                | Self::Eof
                | Self::Error
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::SourceFile)
    }
}
