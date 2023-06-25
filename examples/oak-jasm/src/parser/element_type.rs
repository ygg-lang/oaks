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
        unsafe { std::mem::transmute(token) }
    }
}
