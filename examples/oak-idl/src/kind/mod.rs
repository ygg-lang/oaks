use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
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

impl TokenType for IdlSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for IdlSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::SourceFile)
    }
}
