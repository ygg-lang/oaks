use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum JasmElementType {
    // --- Tokens (mirrored from JasmTokenType) ---
    // These MUST be in the same order as JasmTokenType
    RootToken, // renamed to avoid conflict if needed, but JasmTokenType has Root at 0
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
    StringLiteral,
    Number,
    IdentifierToken,
    TypeDescriptor,
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
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,

    // --- Composite Elements ---
    Root,
    Class,
    Method,
    Field,
    Annotation,
    Instruction,
}

impl ElementType for JasmElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        use UniversalElementRole::*;
        match self {
            Self::Root => Root,
            Self::Class => Definition,
            Self::Method => Definition,
            Self::Field => Definition,
            _ => None,
        }
    }
}

impl From<crate::lexer::token_type::JasmTokenType> for JasmElementType {
    fn from(token: crate::lexer::token_type::JasmTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
