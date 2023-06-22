#![doc = include_str!("../../readme.md")]

use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum VhdlSyntaxKind {
    // 特殊
    Root,
    SourceFile,

    // 基础文本
    Text,
    Whitespace,
    Newline,

    // 错误处理
    Error,

    // EOF
    Eof,

    // VHDL 关键字
    EntityKw,
    ArchitectureKw,
    BeginKw,
    EndKw,
    ProcessKw,
    SignalKw,
    VariableKw,
    ConstantKw,
    ComponentKw,
    PortKw,
    MapKw,
    GenericKw,
    LibraryKw,
    UseKw,
    PackageKw,
    FunctionKw,
    ProcedureKw,
    TypeKw,
    SubtypeKw,
    RecordKw,
    ArrayKw,
    IfKw,
    ThenKw,
    ElseKw,
    ElsifKw,
    CaseKw,
    WhenKw,
    LoopKw,
    ForKw,
    WhileKw,
    ExitKw,
    NextKw,
    ReturnKw,
    WaitKw,
    UntilKw,
    InKw,
    OutKw,
    InoutKw,
    BufferKw,
    LinkageKw,
    DowntoKw,
    ToKw,
    GenerateKw,
    WithKw,
    SelectKw,
    AllKw,
    OthersKw,
    NullKw,
    OpenKw,
    IsKw,
    OfKw,
    RangeKw,
    ReverseRangeKw,
    AttributeKw,
    AliasKw,
    FileKw,
    AccessKw,
    AfterKw,
    AssertKw,
    ReportKw,
    SeverityKw,

    // 标准类型
    BitKw,
    BitVectorKw,
    BooleanKw,
    CharacterKw,
    IntegerKw,
    NaturalKw,
    PositiveKw,
    RealKw,
    StringKw,
    TimeKw,
    StdLogicKw,
    StdLogicVectorKw,
    UnsignedKw,
    SignedKw,

    // 操作符
    And,
    Or,
    Nand,
    Nor,
    Xor,
    Xnor,
    Not,
    Sll,
    Srl,
    Sla,
    Sra,
    Rol,
    Ror,
    Mod,
    Rem,
    Abs,
    Pow,

    // 比较操作符
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // 算术操作符
    Plus,
    Minus,
    Star,
    Slash,
    Ampersand,

    // 赋值操作符
    Assign,
    Arrow,
    DoubleArrow,

    // 标点符号
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Semicolon,
    Colon,
    Comma,
    Dot,
    Apostrophe,
    Quote,
    Pipe,
    Underscore,

    // 字面量
    Identifier,
    StringLiteral,
    CharLiteral,
    IntegerLiteral,
    RealLiteral,
    BitStringLiteral,
    BasedLiteral,

    // 注释
    Comment,

    // 特殊符号
    Tick,
    DoubleTick,
    DoubleQuote,
    Hash,
    At,
    Question,
    Dollar,
    Percent,
    Caret,
    Tilde,
    Backslash,
}

impl TokenType for VhdlSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for VhdlSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
