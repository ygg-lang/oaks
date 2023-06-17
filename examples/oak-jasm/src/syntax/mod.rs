/// 统一JASM 语法种类（节点与词法
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JasmSyntaxKind {
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

impl oak_core::SyntaxKind for JasmSyntaxKind {
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
        !matches!(
            self,
            Self::Root
                | Self::Class
                | Self::Method
                | Self::Field
                | Self::Instruction
                | Self::IdentifierNode
                | Self::StringNode
                | Self::NumberNode
                | Self::ErrorNode
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            Self::Root
                | Self::Class
                | Self::Method
                | Self::Field
                | Self::Instruction
                | Self::IdentifierNode
                | Self::StringNode
                | Self::NumberNode
                | Self::ErrorNode
        )
    }
}
