use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::Serialize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, serde::Deserialize)]
pub enum ProtobufSyntaxKind {
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

impl TokenType for ProtobufSyntaxKind {
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

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for ProtobufSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            Self::Root => UniversalElementRole::Root,
            Self::MessageDef | Self::EnumDef | Self::ServiceDef | Self::PackageDef => UniversalElementRole::Detail,
            _ => UniversalElementRole::None,
        }
    }
}
