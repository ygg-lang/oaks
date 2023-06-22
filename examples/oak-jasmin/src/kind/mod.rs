use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// 统一JASMIN 语法种类（节点与词法）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum JasminSyntaxKind {
    // 语法节点
    Root,
    Class,
    Method,
    Field,
    Instruction,
    IdentifierNode,
    StringNode,
    NumberNode,
    ErrorNode,

    // 词法 kind
    ClassKw,
    VersionKw,
    MethodKw,
    FieldKw,
    StringKw,
    SourceFileKw,
    StackKw,
    LocalsKw,
    EndKw,
    CompiledKw,
    FromKw,
    InnerClassKw,
    NestMembersKw,
    BootstrapMethodKw,

    Public,
    Private,
    Protected,
    Static,
    Super,
    Final,
    Abstract,
    Synchronized,
    Native,
    Synthetic,
    Deprecated,
    Varargs,

    ALoad0,
    ALoad1,
    ALoad2,
    ALoad3,
    ILoad0,
    ILoad1,
    ILoad2,
    ILoad3,
    Ldc,
    LdcW,
    Ldc2W,
    InvokeSpecial,
    InvokeVirtual,
    InvokeStatic,
    InvokeInterface,
    InvokeDynamic,
    GetStatic,
    PutStatic,
    GetField,
    PutField,
    Return,
    IReturn,
    AReturn,
    LReturn,
    FReturn,
    DReturn,
    Nop,
    Dup,
    Pop,
    New,

    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Colon,
    Semicolon,
    Dot,
    Comma,
    Slash,

    StringLiteral,
    Number,
    TypeDescriptor,
    IdentifierToken,
    Whitespace,
    Comment,
    Eof,
    Error,
}

impl TokenType for JasminSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for JasminSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
