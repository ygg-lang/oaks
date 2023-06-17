use oak_core::SyntaxKind;

/// 统一JASMIN 语法种类（节点与词法
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

impl SyntaxKind for JasminSyntaxKind {
    fn is_trivia(&self) -> bool {
        todo!()
    }

    fn is_comment(&self) -> bool {
        todo!()
    }

    fn is_whitespace(&self) -> bool {
        todo!()
    }

    fn is_token_type(&self) -> bool {
        todo!()
    }

    fn is_element_type(&self) -> bool {
        todo!()
    }
}
