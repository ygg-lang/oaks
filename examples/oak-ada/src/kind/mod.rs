use oak_core::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
#[repr(u16)]
pub enum AdaSyntaxKind {
    // Trivia
    Whitespace,
    Newline,
    Comment,

    // Literals
    StringLiteral,
    CharacterLiteral,
    NumberLiteral,

    // Identifiers
    Identifier,

    // Keywords
    AbortKeyword,
    AbsKeyword,
    AbstractKeyword,
    AcceptKeyword,
    AccessKeyword,
    AliasedKeyword,
    AllKeyword,
    AndKeyword,
    ArrayKeyword,
    AtKeyword,
    BeginKeyword,
    BodyKeyword,
    CaseKeyword,
    ConstantKeyword,
    DeclareKeyword,
    DelayKeyword,
    DeltaKeyword,
    DigitsKeyword,
    DoKeyword,
    ElseKeyword,
    ElsifKeyword,
    EndKeyword,
    EntryKeyword,
    ExceptionKeyword,
    ExitKeyword,
    ForKeyword,
    FunctionKeyword,
    GenericKeyword,
    GotoKeyword,
    IfKeyword,
    InKeyword,
    InterfaceKeyword,
    IsKeyword,
    LimitedKeyword,
    LoopKeyword,
    ModKeyword,
    NewKeyword,
    NotKeyword,
    NullKeyword,
    OfKeyword,
    OrKeyword,
    OthersKeyword,
    OutKeyword,
    OverridingKeyword,
    PackageKeyword,
    PragmaKeyword,
    PrivateKeyword,
    ProcedureKeyword,
    ProtectedKeyword,
    RaiseKeyword,
    RangeKeyword,
    RecordKeyword,
    RemKeyword,
    RenamesKeyword,
    RequeueKeyword,
    ReturnKeyword,
    ReverseKeyword,
    SelectKeyword,
    SeparateKeyword,
    SubtypeKeyword,
    SynchronizedKeyword,
    TaggedKeyword,
    TaskKeyword,
    TerminateKeyword,
    ThenKeyword,
    TypeKeyword,
    UntilKeyword,
    UseKeyword,
    WhenKeyword,
    WhileKeyword,
    WithKeyword,
    XorKeyword,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Star,
    Slash,
    Power,
    DoubleStar,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Assignment,
    ColonEqual,
    Arrow,
    LeftShift,
    RightShift,
    Box,
    Ampersand,
    Pipe,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    Dot,
    DotDot,

    // Composite nodes
    SourceFile,

    // Error handling
    Error,
    Eof,
}

impl SyntaxKind for AdaSyntaxKind {
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
        !matches!(self, Self::SourceFile)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::SourceFile)
    }
}

pub type AdaToken = oak_core::Token<AdaSyntaxKind>;
