use oak_core::SyntaxKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
}

impl SyntaxKind for ProtobufSyntaxKind {
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
        true // Protobuf doesn't have element types in this simple implementation
    }

    fn is_element_type(&self) -> bool {
        false // Protobuf doesn't have element types in this simple implementation
    }
}
