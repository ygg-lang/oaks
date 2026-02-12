use oak_core::{Token, TokenType, UniversalTokenRole};

/// Token type for the V language.
pub type VLangToken = Token<VLangTokenType>;

/// Token types for the V language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VLangTokenType {
    // Basic
    /// An identifier.
    Identifier,
    /// Whitespace.
    Whitespace,
    /// A newline character.
    Newline,
    /// A comment.
    Comment,
    /// A lexical error.
    Error,
    /// End of input.
    Eof,

    // Literals
    /// A string literal.
    StringLiteral,
    /// A character literal.
    CharLiteral,
    /// An integer literal.
    IntegerLiteral,
    /// A floating-point literal.
    FloatLiteral,
    /// A boolean literal.
    BoolLiteral,

    // Keywords
    /// The `module` keyword.
    ModuleKw,
    /// The `import` keyword.
    ImportKw,
    /// The `pub` keyword.
    PubKw,
    /// The `fn` keyword.
    FnKw,
    /// The `struct` keyword.
    StructKw,
    /// The `interface` keyword.
    InterfaceKw,
    /// The `enum` keyword.
    EnumKw,
    /// The `type` keyword.
    TypeKw,
    /// The `const` keyword.
    ConstKw,
    /// The `mut` keyword.
    MutKw,
    /// The `shared` keyword.
    SharedKw,
    /// The `volatile` keyword.
    VolatileKw,
    /// The `unsafe` keyword.
    UnsafeKw,
    /// The `if` keyword.
    IfKw,
    /// The `else` keyword.
    ElseKw,
    /// The `for` keyword.
    ForKw,
    /// The `in` keyword.
    InKw,
    /// The `match` keyword.
    MatchKw,
    /// The `or` keyword.
    OrKw,
    /// The `return` keyword.
    ReturnKw,
    /// The `break` keyword.
    BreakKw,
    /// The `continue` keyword.
    ContinueKw,
    /// The `goto` keyword.
    GotoKw,
    /// The `defer` keyword.
    DeferKw,
    /// The `go` keyword.
    GoKw,
    /// The `select` keyword.
    SelectKw,
    /// The `lock` keyword.
    LockKw,
    /// The `rlock` keyword.
    RlockKw,
    /// The `as` keyword.
    AsKw,
    /// The `is` keyword.
    IsKw,
    /// The `sizeof` keyword.
    SizeofKw,
    /// The `typeof` keyword.
    TypeofKw,
    /// The `offsetof` keyword.
    OffsetofKw,
    /// The `assert` keyword.
    AssertKw,
    /// The `panic` keyword.
    PanicKw,
    /// The `eprintln` keyword.
    EprintlnKw,
    /// The `println` keyword.
    PrintlnKw,
    /// The `print` keyword.
    PrintKw,
    /// The `eprint` keyword.
    EprintKw,
    /// The `bool` keyword.
    BoolKw,
    /// The `i8` keyword.
    I8Kw,
    /// The `i16` keyword.
    I16Kw,
    /// The `i32` keyword.
    I32Kw,
    /// The `i64` keyword.
    I64Kw,
    /// The `u8` keyword.
    U8Kw,
    /// The `u16` keyword.
    U16Kw,
    /// The `u32` keyword.
    U32Kw,
    /// The `u64` keyword.
    U64Kw,
    /// The `int` keyword.
    IntKw,
    /// The `uint` keyword.
    UintKw,
    /// The `f32` keyword.
    F32Kw,
    /// The `f64` keyword.
    F64Kw,
    /// The `string` keyword.
    StringKw,
    /// The `rune` keyword.
    RuneKw,
    /// The `byte` keyword.
    ByteKw,
    /// The `voidptr` keyword.
    VoidptrKw,
    /// The `char` keyword.
    CharKw,

    // Operators and Punctuation
    /// The `+` operator.
    Plus,
    /// The `-` operator.
    Minus,
    /// The `*` operator.
    Star,
    /// The `/` operator.
    Slash,
    /// The `%` operator.
    Percent,
    /// The `+=` operator.
    PlusEq,
    /// The `-=` operator.
    MinusEq,
    /// The `*=` operator.
    StarEq,
    /// The `/=` operator.
    SlashEq,
    /// The `%=` operator.
    PercentEq,
    /// The `++` operator.
    PlusPlus,
    /// The `--` operator.
    MinusMinus,
    /// The `&` operator.
    Ampersand,
    /// The `|` operator.
    Pipe,
    /// The `^` operator.
    Caret,
    /// The `&=` operator.
    AmpersandEq,
    /// The `|=` operator.
    PipeEq,
    /// The `^=` operator.
    CaretEq,
    /// The `&&` operator.
    AndAnd,
    /// The `||` operator.
    OrOr,
    /// The `=` operator.
    Eq,
    /// The `==` operator.
    EqEq,
    /// The `!=` operator.
    Ne,
    /// The `!` operator.
    Bang,
    /// The `<` operator.
    Lt,
    /// The `<=` operator.
    Le,
    /// The `>` operator.
    Gt,
    /// The `>=` operator.
    Ge,
    /// The `<<` operator.
    LeftShift,
    /// The `>>` operator.
    RightShift,
    /// The `<<=` operator.
    LeftShiftEq,
    /// The `>>=` operator.
    RightShiftEq,
    /// The `->` operator.
    Arrow,
    /// The `=>` operator.
    FatArrow,
    /// The `<` operator (alternative).
    LessThan, // Used in lexer/mod.rs:473
    /// The `>` operator (alternative).
    GreaterThan, // Used in lexer/mod.rs:493
    /// The `..` operator.
    DotDot,
    /// The `...` operator.
    DotDotDot,
    /// The `,` punctuation.
    Comma,
    /// The `;` punctuation.
    Semicolon,
    /// The `.` punctuation.
    Dot,
    /// The `:` punctuation.
    Colon,
    /// The `(` punctuation.
    LeftParen,
    /// The `)` punctuation.
    RightParen,
    /// The `[` punctuation.
    LeftBracket,
    /// The `]` punctuation.
    RightBracket,
    /// The `{` punctuation.
    LeftBrace,
    /// The `}` punctuation.
    RightBrace,
    /// The `?` punctuation.
    Question,
    /// The `~` punctuation.
    Tilde,
}

impl TokenType for VLangTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
