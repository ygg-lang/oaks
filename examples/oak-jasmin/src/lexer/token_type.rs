use oak_core::{Token, TokenType, UniversalTokenRole};

/// Jasmin token.
pub type JasminToken = Token<JasminTokenType>;

impl TokenType for JasminTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
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
            | Self::Bridge
            | Self::Enum
            | Self::Annotation
            | Self::Strictfp
            | Self::Interface
            | Self::Volatile
            | Self::Transient
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
            | Self::New => UniversalTokenRole::Keyword,
            Self::IdentifierToken => UniversalTokenRole::Name,
            Self::StringLiteral | Self::Number => UniversalTokenRole::Literal,
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::LeftBrace | Self::RightBrace | Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::Colon | Self::Semicolon | Self::Dot | Self::Comma => UniversalTokenRole::Punctuation,
            Self::Slash => UniversalTokenRole::Operator,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

/// Jasmin token type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum JasminTokenType {
    // Syntax Nodes
    /// Root node.
    Root,
    /// Class node.
    Class,
    /// Method node.
    Method,
    /// Field node.
    Field,
    /// Instruction node.
    Instruction,
    /// Identifier node.
    IdentifierNode,
    /// String node.
    StringNode,
    /// Number node.
    NumberNode,
    /// Error node.
    ErrorNode,

    // Lexical Kinds
    /// `.class` keyword.
    ClassKw,
    /// `.version` keyword.
    VersionKw,
    /// `.method` keyword.
    MethodKw,
    /// `.field` keyword.
    FieldKw,
    /// `.string` keyword.
    StringKw,
    /// `.source` keyword.
    SourceFileKw,
    /// `.stack` keyword.
    StackKw,
    /// `.locals` keyword.
    LocalsKw,
    /// `.end` keyword.
    EndKw,
    /// `compiled` keyword.
    CompiledKw,
    /// `from` keyword.
    FromKw,
    /// `inner` keyword.
    InnerClassKw,
    /// `nest` keyword.
    NestMembersKw,
    /// `bootstrap` keyword.
    BootstrapMethodKw,

    /// `public` modifier.
    Public,
    /// `private` modifier.
    Private,
    /// `protected` modifier.
    Protected,
    /// `static` modifier.
    Static,
    /// `super` modifier.
    Super,
    /// `final` modifier.
    Final,
    /// `abstract` modifier.
    Abstract,
    /// `synchronized` modifier.
    Synchronized,
    /// `native` modifier.
    Native,
    /// `synthetic` modifier.
    Synthetic,
    /// `deprecated` modifier.
    Deprecated,
    /// `varargs` modifier.
    Varargs,
    /// `bridge` modifier.
    Bridge,
    /// `enum` keyword.
    Enum,
    /// `annotation` keyword.
    Annotation,
    /// `strictfp` modifier.
    Strictfp,
    /// `interface` keyword.
    Interface,
    /// `volatile` modifier.
    Volatile,
    /// `transient` modifier.
    Transient,

    /// `aload_0` instruction.
    ALoad0,
    /// `aload_1` instruction.
    ALoad1,
    /// `aload_2` instruction.
    ALoad2,
    /// `aload_3` instruction.
    ALoad3,
    /// `iload_0` instruction.
    ILoad0,
    /// `iload_1` instruction.
    ILoad1,
    /// `iload_2` instruction.
    ILoad2,
    /// `iload_3` instruction.
    ILoad3,
    /// `ldc` instruction.
    Ldc,
    /// `ldc_w` instruction.
    LdcW,
    /// `ldc2_w` instruction.
    Ldc2W,
    /// `invokespecial` instruction.
    InvokeSpecial,
    /// `invokevirtual` instruction.
    InvokeVirtual,
    /// `invokestatic` instruction.
    InvokeStatic,
    /// `invokeinterface` instruction.
    InvokeInterface,
    /// `invokedynamic` instruction.
    InvokeDynamic,
    /// `getstatic` instruction.
    GetStatic,
    /// `putstatic` instruction.
    PutStatic,
    /// `getfield` instruction.
    GetField,
    /// `putfield` instruction.
    PutField,
    /// `return` instruction.
    Return,
    /// `ireturn` instruction.
    IReturn,
    /// `areturn` instruction.
    AReturn,
    /// `lreturn` instruction.
    LReturn,
    /// `freturn` instruction.
    FReturn,
    /// `dreturn` instruction.
    DReturn,
    /// `nop` instruction.
    Nop,
    /// `dup` instruction.
    Dup,
    /// `pop` instruction.
    Pop,
    /// `new` instruction.
    New,

    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Colon `:`.
    Colon,
    /// Semicolon `;`.
    Semicolon,
    /// Dot `.`.
    Dot,
    /// Comma `,`.
    Comma,
    /// Slash `/`.
    Slash,

    /// String literal.
    StringLiteral,
    /// Number literal.
    Number,
    /// Type descriptor.
    TypeDescriptor,
    /// Identifier token.
    IdentifierToken,
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    /// End of file.
    Eof,
    /// Error.
    Error,
}
