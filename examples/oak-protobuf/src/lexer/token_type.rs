use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type ProtobufToken = Token<ProtobufTokenType>;

impl TokenType for ProtobufTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProtobufTokenType {
    // 空白符和换行
    Whitespace,
    Newline,

    // 注释
    Comment,

    // 字面量
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,

    // 标识符
    Identifier,

    // 关键字
    Syntax,
    Package,
    Import,
    Option,
    Message,
    Enum,
    Service,
    Rpc,
    Returns,
    Stream,
    Repeated,
    Optional,
    Required,
    Oneof,
    Map,
    Reserved,
    Extensions,
    Extend,
    Group,
    Public,
    Weak,

    // 数据类型
    Double,
    Float,
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Fixed32,
    Fixed64,
    Sfixed32,
    Sfixed64,
    Bool,
    String,
    Bytes,

    // 运算符
    Assign,
    Semicolon,
    Comma,
    Dot,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    LeftAngle,
    RightAngle,

    // 特殊
    Error,
    Eof,

    // Element types
    Root,
    SyntaxDef,
    PackageDef,
    ImportDef,
    OptionDef,
    MessageDef,
    EnumDef,
    ServiceDef,
    RpcDef,
    FieldDef,
    EnumFieldDef,
    OneofDef,
    MapFieldDef,
    ReservedDef,
    ExtensionsDef,
}
