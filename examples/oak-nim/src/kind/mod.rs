use oak_core::SyntaxKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NimSyntaxKind {
    // Whitespace and comments
    Whitespace,
    Newline,
    CommentToken,

    // Keywords
    AddrKeyword,
    AndKeyword,
    AsKeyword,
    AsmKeyword,
    BindKeyword,
    BlockKeyword,
    BreakKeyword,
    CaseKeyword,
    CastKeyword,
    ConceptKeyword,
    ConstKeyword,
    ContinueKeyword,
    ConverterKeyword,
    DeferKeyword,
    DiscardKeyword,
    DistinctKeyword,
    DivKeyword,
    DoKeyword,
    ElifKeyword,
    ElseKeyword,
    EndKeyword,
    EnumKeyword,
    ExceptKeyword,
    ExportKeyword,
    FinallyKeyword,
    ForKeyword,
    FromKeyword,
    FuncKeyword,
    IfKeyword,
    ImportKeyword,
    InKeyword,
    IncludeKeyword,
    InterfaceKeyword,
    IsKeyword,
    IteratorKeyword,
    LetKeyword,
    MacroKeyword,
    MethodKeyword,
    MixinKeyword,
    ModKeyword,
    NilKeyword,
    NotKeyword,
    NotnilKeyword,
    ObjectKeyword,
    OfKeyword,
    OrKeyword,
    OutKeyword,
    ProcKeyword,
    PtrKeyword,
    RaiseKeyword,
    RefKeyword,
    ReturnKeyword,
    ShlKeyword,
    ShrKeyword,
    StaticKeyword,
    TemplateKeyword,
    TryKeyword,
    TupleKeyword,
    TypeKeyword,
    UsingKeyword,
    VarKeyword,
    WhenKeyword,
    WhileKeyword,
    XorKeyword,
    YieldKeyword,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    LeftShift,
    RightShift,
    DotDot,
    Arrow,
    At,

    // Punctuation
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
    Question,
    Exclamation,
    Dollar,
    Backtick,

    // Literals
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BoolLiteral,

    // Identifiers
    Identifier,

    // Special
    Root,
    Error,
    Eof,
}

impl SyntaxKind for NimSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::CommentToken)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::CommentToken)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Root)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Root)
    }
}
