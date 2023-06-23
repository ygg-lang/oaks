use oak_core::Token;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type JasmToken = Token<JasmTokenType>;

/// 统一JASM 语法种类（节点与词法）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JasmTokenType {
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
    Newline,
    Comment,
    Eof,
    Error,
}

impl oak_core::TokenType for JasmTokenType {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = oak_core::UniversalTokenRole;

    fn role(&self) -> Self::Role {
        use oak_core::UniversalTokenRole::*;
        match self {
            Self::ClassKw
            | Self::VersionKw
            | Self::MethodKw
            | Self::FieldKw
            | Self::StringKw
            | Self::SourceFileKw
            | Self::StackKw
            | Self::LocalsKw
            | Self::EndKw
            | Self::CompiledKw
            | Self::FromKw
            | Self::InnerClassKw
            | Self::NestMembersKw
            | Self::BootstrapMethodKw
            | Self::Public
            | Self::Private
            | Self::Protected
            | Self::Static
            | Self::Super
            | Self::Final
            | Self::Abstract
            | Self::Synchronized
            | Self::Native
            | Self::Synthetic
            | Self::Deprecated
            | Self::Varargs
            | Self::ALoad0
            | Self::ALoad1
            | Self::ALoad2
            | Self::ALoad3
            | Self::ILoad0
            | Self::ILoad1
            | Self::ILoad2
            | Self::ILoad3
            | Self::Ldc
            | Self::LdcW
            | Self::Ldc2W
            | Self::InvokeSpecial
            | Self::InvokeVirtual
            | Self::InvokeStatic
            | Self::InvokeInterface
            | Self::InvokeDynamic
            | Self::GetStatic
            | Self::PutStatic
            | Self::GetField
            | Self::PutField
            | Self::Return
            | Self::IReturn
            | Self::AReturn
            | Self::LReturn
            | Self::FReturn
            | Self::DReturn
            | Self::Nop
            | Self::Dup
            | Self::Pop
            | Self::New => Keyword,
            Self::StringLiteral | Self::Number => Literal,
            Self::IdentifierToken | Self::TypeDescriptor => Name,
            Self::LeftBrace | Self::RightBrace | Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::Colon | Self::Semicolon | Self::Dot | Self::Comma | Self::Slash => Punctuation,
            Self::Whitespace | Self::Newline => Whitespace,
            Self::Comment => Comment,
            _ => None,
        }
    }

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl oak_core::ElementType for JasmTokenType {
    type Role = oak_core::UniversalElementRole;

    fn role(&self) -> Self::Role {
        use oak_core::UniversalElementRole::*;
        match self {
            Self::Root => Root,
            Self::Class | Self::Method | Self::Field => Definition,
            _ => None,
        }
    }
}
