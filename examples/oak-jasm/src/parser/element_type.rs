//! Element types for the JASM language.
use oak_core::{ElementType, UniversalElementRole};

/// Element types for the JASM AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum JasmElementType {
    // --- Tokens (mirrored from JasmTokenType) ---
    // These MUST be in the same order as JasmTokenType
    /// Root token.
    RootToken,
    /// `class` keyword.
    ClassKw,
    /// `version` keyword.
    VersionKw,
    /// `method` keyword.
    MethodKw,
    /// `field` keyword.
    FieldKw,
    /// `string` keyword.
    StringKw,
    /// `source_file` keyword.
    SourceFileKw,
    /// `stack` keyword.
    StackKw,
    /// `locals` keyword.
    LocalsKw,
    /// `end` keyword.
    EndKw,
    /// `compiled` keyword.
    CompiledKw,
    /// `from` keyword.
    FromKw,
    /// `inner_class` keyword.
    InnerClassKw,
    /// `nest_members` keyword.
    NestMembersKw,
    /// `bootstrap_method` keyword.
    BootstrapMethodKw,
    /// `public` access modifier.
    Public,
    /// `private` access modifier.
    Private,
    /// `protected` access modifier.
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
    /// `new` instruction.
    New,
    /// `checkcast` instruction.
    CheckCast,
    /// `instanceof` instruction.
    InstanceOf,
    /// `newarray` instruction.
    NewArray,
    /// `anewarray` instruction.
    ANewArray,
    /// `arraylength` instruction.
    ArrayLength,
    /// `athrow` instruction.
    AThrow,
    /// `monitorenter` instruction.
    MonitorEnter,
    /// `monitorexit` instruction.
    MonitorExit,
    /// `multianewarray` instruction.
    MultiANewArray,
    /// `ifnull` instruction.
    IfNull,
    /// `ifnonnull` instruction.
    IfNonNull,
    /// `goto` instruction.
    Goto,
    /// `goto_w` instruction.
    GotoW,
    /// `jsr` instruction.
    Jsr,
    /// `jsr_w` instruction.
    JsrW,
    /// `ret` instruction.
    Ret,
    /// `tableswitch` instruction.
    TableSwitch,
    /// `lookupswitch` instruction.
    LookupSwitch,
    /// `ireturn` instruction.
    IReturn,
    /// `lreturn` instruction.
    LReturn,
    /// `freturn` instruction.
    FReturn,
    /// `dreturn` instruction.
    DReturn,
    /// `areturn` instruction.
    AReturn,
    /// `return` instruction.
    Return,
    /// `bipush` instruction.
    BiPush,
    /// `sipush` instruction.
    SiPush,
    /// `iinc` instruction.
    IInc,
    /// `wide` instruction.
    Wide,
    /// `breakpoint` instruction.
    BreakPoint,
    /// `impdep1` instruction.
    ImpDep1,
    /// `impdep2` instruction.
    ImpDep2,
    /// `nop` instruction.
    Nop,
    /// `dup` instruction.
    Dup,
    /// `pop` instruction.
    Pop,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Comma `,`.
    Comma,
    /// Colon `:`.
    Colon,
    /// Semicolon `;`.
    Semicolon,
    /// Equals `=`.
    Eq,
    /// Dot `.`.
    Dot,
    /// Slash `/`.
    Slash,
    /// Identifier.
    Identifier,
    /// String literal.
    String,
    /// Number literal.
    Number,
    /// Comment.
    Comment,
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// End of file.
    Eof,

    // --- High-level AST Elements ---
    /// Root node of the AST.
    Root,
    /// Class definition.
    Class,
    /// Method definition.
    Method,
    /// Field definition.
    Field,
    /// Constant pool entry.
    Constant,
    /// Attribute definition.
    Attribute,
    /// Instruction.
    Instruction,
    /// Exception handler.
    ExceptionHandler,
    /// Stack map frame.
    StackMapFrame,
    /// Inner class definition.
    InnerClass,
    /// Annotation definition.
    Annotation,
    /// Annotation parameter.
    AnnotationParam,
    /// Annotation array parameter.
    AnnotationArray,
    /// Error node.
    Error,
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
                match token {
            crate::lexer::token_type::JasmTokenType::Root => Self::Root,
            crate::lexer::token_type::JasmTokenType::ClassKw => Self::ClassKw,
            crate::lexer::token_type::JasmTokenType::VersionKw => Self::VersionKw,
            crate::lexer::token_type::JasmTokenType::MethodKw => Self::MethodKw,
            crate::lexer::token_type::JasmTokenType::FieldKw => Self::FieldKw,
            crate::lexer::token_type::JasmTokenType::StringKw => Self::StringKw,
            crate::lexer::token_type::JasmTokenType::SourceFileKw => Self::SourceFileKw,
            crate::lexer::token_type::JasmTokenType::StackKw => Self::StackKw,
            crate::lexer::token_type::JasmTokenType::LocalsKw => Self::LocalsKw,
            crate::lexer::token_type::JasmTokenType::EndKw => Self::EndKw,
            crate::lexer::token_type::JasmTokenType::CompiledKw => Self::CompiledKw,
            crate::lexer::token_type::JasmTokenType::FromKw => Self::FromKw,
            crate::lexer::token_type::JasmTokenType::InnerClassKw => Self::InnerClassKw,
            crate::lexer::token_type::JasmTokenType::NestMembersKw => Self::NestMembersKw,
            crate::lexer::token_type::JasmTokenType::BootstrapMethodKw => Self::BootstrapMethodKw,
            crate::lexer::token_type::JasmTokenType::Public => Self::Public,
            crate::lexer::token_type::JasmTokenType::Private => Self::Private,
            crate::lexer::token_type::JasmTokenType::Protected => Self::Protected,
            crate::lexer::token_type::JasmTokenType::Static => Self::Static,
            crate::lexer::token_type::JasmTokenType::Super => Self::Super,
            crate::lexer::token_type::JasmTokenType::Final => Self::Final,
            crate::lexer::token_type::JasmTokenType::Abstract => Self::Abstract,
            crate::lexer::token_type::JasmTokenType::Synchronized => Self::Synchronized,
            crate::lexer::token_type::JasmTokenType::Native => Self::Native,
            crate::lexer::token_type::JasmTokenType::Synthetic => Self::Synthetic,
            crate::lexer::token_type::JasmTokenType::Deprecated => Self::Deprecated,
            crate::lexer::token_type::JasmTokenType::Varargs => Self::Varargs,
            crate::lexer::token_type::JasmTokenType::ALoad0 => Self::ALoad0,
            crate::lexer::token_type::JasmTokenType::ALoad1 => Self::ALoad1,
            crate::lexer::token_type::JasmTokenType::ALoad2 => Self::ALoad2,
            crate::lexer::token_type::JasmTokenType::ALoad3 => Self::ALoad3,
            crate::lexer::token_type::JasmTokenType::ILoad0 => Self::ILoad0,
            crate::lexer::token_type::JasmTokenType::ILoad1 => Self::ILoad1,
            crate::lexer::token_type::JasmTokenType::ILoad2 => Self::ILoad2,
            crate::lexer::token_type::JasmTokenType::ILoad3 => Self::ILoad3,
            crate::lexer::token_type::JasmTokenType::Ldc => Self::Ldc,
            crate::lexer::token_type::JasmTokenType::LdcW => Self::LdcW,
            crate::lexer::token_type::JasmTokenType::Ldc2W => Self::Ldc2W,
            crate::lexer::token_type::JasmTokenType::InvokeSpecial => Self::InvokeSpecial,
            crate::lexer::token_type::JasmTokenType::InvokeVirtual => Self::InvokeVirtual,
            crate::lexer::token_type::JasmTokenType::InvokeStatic => Self::InvokeStatic,
            crate::lexer::token_type::JasmTokenType::InvokeInterface => Self::InvokeInterface,
            crate::lexer::token_type::JasmTokenType::InvokeDynamic => Self::InvokeDynamic,
            crate::lexer::token_type::JasmTokenType::GetStatic => Self::GetStatic,
            crate::lexer::token_type::JasmTokenType::PutStatic => Self::PutStatic,
            crate::lexer::token_type::JasmTokenType::GetField => Self::GetField,
            crate::lexer::token_type::JasmTokenType::PutField => Self::PutField,
            crate::lexer::token_type::JasmTokenType::Return => Self::Return,
            crate::lexer::token_type::JasmTokenType::IReturn => Self::IReturn,
            crate::lexer::token_type::JasmTokenType::AReturn => Self::AReturn,
            crate::lexer::token_type::JasmTokenType::LReturn => Self::LReturn,
            crate::lexer::token_type::JasmTokenType::FReturn => Self::FReturn,
            crate::lexer::token_type::JasmTokenType::DReturn => Self::DReturn,
            crate::lexer::token_type::JasmTokenType::Nop => Self::Nop,
            crate::lexer::token_type::JasmTokenType::Dup => Self::Dup,
            crate::lexer::token_type::JasmTokenType::Pop => Self::Pop,
            crate::lexer::token_type::JasmTokenType::New => Self::New,
            crate::lexer::token_type::JasmTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::JasmTokenType::Number => Self::Number,
            crate::lexer::token_type::JasmTokenType::IdentifierToken => Self::IdentifierToken,
            crate::lexer::token_type::JasmTokenType::TypeDescriptor => Self::TypeDescriptor,
            crate::lexer::token_type::JasmTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::JasmTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::JasmTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::JasmTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::JasmTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::JasmTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::JasmTokenType::Colon => Self::Colon,
            crate::lexer::token_type::JasmTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::JasmTokenType::Dot => Self::Dot,
            crate::lexer::token_type::JasmTokenType::Comma => Self::Comma,
            crate::lexer::token_type::JasmTokenType::Slash => Self::Slash,
            crate::lexer::token_type::JasmTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::JasmTokenType::Newline => Self::Newline,
            crate::lexer::token_type::JasmTokenType::Comment => Self::Comment,
            crate::lexer::token_type::JasmTokenType::Error => Self::Error,
            crate::lexer::token_type::JasmTokenType::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}
