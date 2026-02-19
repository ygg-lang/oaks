//! Token types for the JASM language.
use oak_core::{Token, TokenType, UniversalTokenRole};

/// Type alias for a JASM token.
pub type JasmToken = Token<JasmTokenType>;

/// Token types for the JASM language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum JasmTokenType {
    /// Root node.
    Root,
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
    /// `super` keyword.
    SuperKw,
    /// `interface` keyword.
    InterfaceKw,
    /// `implements` keyword.
    ImplementsKw,
    /// `extends` keyword.
    ExtendsKw,
    /// `source` keyword.
    SourceKw,
    /// `catch` keyword.
    CatchKw,
    /// `attribute` keyword.
    AttributeKw,
    /// `stackmap` keyword.
    StackMapKw,

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

    /// `aconst_null` instruction.
    AConstNull,
    /// `iconst_m1` instruction.
    IConstM1,
    /// `iconst_0` instruction.
    IConst0,
    /// `iconst_1` instruction.
    IConst1,
    /// `iconst_2` instruction.
    IConst2,
    /// `iconst_3` instruction.
    IConst3,
    /// `iconst_4` instruction.
    IConst4,
    /// `iconst_5` instruction.
    IConst5,
    /// `lconst_0` instruction.
    LConst0,
    /// `lconst_1` instruction.
    LConst1,
    /// `fconst_0` instruction.
    FConst0,
    /// `fconst_1` instruction.
    FConst1,
    /// `fconst_2` instruction.
    FConst2,
    /// `dconst_0` instruction.
    DConst0,
    /// `dconst_1` instruction.
    DConst1,

    /// `aload` instruction.
    ALoad,
    /// `iload` instruction.
    ILoad,
    /// `lload` instruction.
    LLoad,
    /// `fload` instruction.
    FLoad,
    /// `dload` instruction.
    DLoad,

    /// `astore` instruction.
    AStore,
    /// `istore` instruction.
    IStore,
    /// `lstore` instruction.
    LStore,
    /// `fstore` instruction.
    FStore,
    /// `dstore` instruction.
    DStore,

    /// `baload` instruction.
    BALoad,
    /// `caload` instruction.
    CALoad,
    /// `saload` instruction.
    SALoad,
    /// `aaload` instruction.
    AALoad,
    /// `iaload` instruction.
    IALoad,
    /// `laload` instruction.
    LALoad,
    /// `faload` instruction.
    FALoad,
    /// `daload` instruction.
    DALoad,

    /// `bastore` instruction.
    BAStore,
    /// `castore` instruction.
    CAStore,
    /// `sastore` instruction.
    SAStore,
    /// `aastore` instruction.
    AAStore,
    /// `iastore` instruction.
    IAStore,
    /// `lastore` instruction.
    LAStore,
    /// `fastore` instruction.
    FAStore,
    /// `dastore` instruction.
    DAStore,

    /// `swap` instruction.
    Swap,
    /// `swap2` instruction.
    Swap2,
    /// `dup_x1` instruction.
    DupX1,
    /// `dup_x2` instruction.
    DupX2,
    /// `dup2` instruction.
    Dup2,
    /// `dup2_x1` instruction.
    Dup2X1,
    /// `dup2_x2` instruction.
    Dup2X2,

    /// `iadd` instruction.
    IAdd,
    /// `ladd` instruction.
    LAdd,
    /// `fadd` instruction.
    FAdd,
    /// `dadd` instruction.
    DAdd,
    /// `isub` instruction.
    ISub,
    /// `lsub` instruction.
    LSub,
    /// `fsub` instruction.
    FSub,
    /// `dsub` instruction.
    DSub,
    /// `imul` instruction.
    IMul,
    /// `lmul` instruction.
    LMul,
    /// `fmul` instruction.
    FMul,
    /// `dmul` instruction.
    DMul,
    /// `idiv` instruction.
    IDiv,
    /// `ldiv` instruction.
    LDiv,
    /// `fdiv` instruction.
    FDiv,
    /// `ddiv` instruction.
    DDiv,
    /// `irem` instruction.
    IRem,
    /// `lrem` instruction.
    LRem,
    /// `frem` instruction.
    FRem,
    /// `drem` instruction.
    DRem,

    /// `ineg` instruction.
    INeg,
    /// `lneg` instruction.
    LNeg,
    /// `fneg` instruction.
    FNeg,
    /// `dneg` instruction.
    DNeg,

    /// `ishl` instruction.
    IShl,
    /// `lshl` instruction.
    LShl,
    /// `ishr` instruction.
    IShr,
    /// `lshr` instruction.
    LShr,
    /// `iushr` instruction.
    IUShr,
    /// `lushr` instruction.
    LUShr,

    /// `iand` instruction.
    IAnd,
    /// `land` instruction.
    LAnd,
    /// `ior` instruction.
    IOr,
    /// `lor` instruction.
    LOr,
    /// `ixor` instruction.
    IXor,
    /// `lxor` instruction.
    LXor,

    /// `i2l` instruction.
    I2L,
    /// `i2f` instruction.
    I2F,
    /// `i2d` instruction.
    I2D,
    /// `l2i` instruction.
    L2I,
    /// `l2f` instruction.
    L2F,
    /// `l2d` instruction.
    L2D,
    /// `f2i` instruction.
    F2I,
    /// `f2l` instruction.
    F2L,
    /// `f2d` instruction.
    F2D,
    /// `d2i` instruction.
    D2I,
    /// `d2l` instruction.
    D2L,
    /// `d2f` instruction.
    D2F,

    /// `lcmp` instruction.
    LCmp,
    /// `fcmpl` instruction.
    FCmpL,
    /// `fcmpg` instruction.
    FCmpG,
    /// `dcmpl` instruction.
    DCmpL,
    /// `dcmpg` instruction.
    DCmpG,

    /// `ifeq` instruction.
    IfEq,
    /// `ifne` instruction.
    IfNe,
    /// `iflt` instruction.
    IfLt,
    /// `ifge` instruction.
    IfGe,
    /// `ifgt` instruction.
    IfGt,
    /// `ifle` instruction.
    IfLe,
    /// `if_icmpeq` instruction.
    IfICmpEq,
    /// `if_icmpne` instruction.
    IfICmpNe,
    /// `if_icmplt` instruction.
    IfICmpLt,
    /// `if_icmpge` instruction.
    IfICmpGe,
    /// `if_icmpgt` instruction.
    IfICmpGt,
    /// `if_icmple` instruction.
    IfICmpLe,
    /// `if_acmpeq` instruction.
    IfACmpEq,
    /// `if_acmpne` instruction.
    IfACmpNe,

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
    /// At symbol `@`.
    At,

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
}

impl TokenType for JasmTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
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
            Self::String | Self::Number => Literal,
            Self::Identifier => Name,
            Self::LeftBrace | Self::RightBrace | Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::Colon | Self::Semicolon | Self::Dot | Self::Comma | Self::Slash => Punctuation,
            Self::Whitespace | Self::Newline => Whitespace,
            Self::Comment => Comment,
            _ => None,
        }
    }
}
