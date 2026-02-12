use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Nim language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum NimElementType {
    // Whitespace and comments
    /// Whitespace characters.
    Whitespace,
    /// A newline character.
    Newline,
    /// A comment token.
    CommentToken,

    // Keywords
    /// The `addr` keyword.
    AddrKeyword,
    /// The `and` keyword.
    AndKeyword,
    /// The `as` keyword.
    AsKeyword,
    /// The `asm` keyword.
    AsmKeyword,
    /// The `bind` keyword.
    BindKeyword,
    /// The `block` keyword.
    BlockKeyword,
    /// The `break` keyword.
    BreakKeyword,
    /// The `case` keyword.
    CaseKeyword,
    /// The `cast` keyword.
    CastKeyword,
    /// The `concept` keyword.
    ConceptKeyword,
    /// The `const` keyword.
    ConstKeyword,
    /// The `continue` keyword.
    ContinueKeyword,
    /// The `converter` keyword.
    ConverterKeyword,
    /// The `defer` keyword.
    DeferKeyword,
    /// The `discard` keyword.
    DiscardKeyword,
    /// The `distinct` keyword.
    DistinctKeyword,
    /// The `div` keyword.
    DivKeyword,
    /// The `do` keyword.
    DoKeyword,
    /// The `elif` keyword.
    ElifKeyword,
    /// The `else` keyword.
    ElseKeyword,
    /// The `end` keyword.
    EndKeyword,
    /// The `enum` keyword.
    EnumKeyword,
    /// The `except` keyword.
    ExceptKeyword,
    /// The `export` keyword.
    ExportKeyword,
    /// The `finally` keyword.
    FinallyKeyword,
    /// The `for` keyword.
    ForKeyword,
    /// The `from` keyword.
    FromKeyword,
    /// The `func` keyword.
    FuncKeyword,
    /// The `if` keyword.
    IfKeyword,
    /// The `import` keyword.
    ImportKeyword,
    /// The `in` keyword.
    InKeyword,
    /// The `include` keyword.
    IncludeKeyword,
    /// The `interface` keyword.
    InterfaceKeyword,
    /// The `is` keyword.
    IsKeyword,
    /// The `iterator` keyword.
    IteratorKeyword,
    /// The `let` keyword.
    LetKeyword,
    /// The `macro` keyword.
    MacroKeyword,
    /// The `method` keyword.
    MethodKeyword,
    /// The `mixin` keyword.
    MixinKeyword,
    /// The `mod` keyword.
    ModKeyword,
    /// The `nil` keyword.
    NilKeyword,
    /// The `not` keyword.
    NotKeyword,
    /// The `notnil` keyword.
    NotnilKeyword,
    /// The `object` keyword.
    ObjectKeyword,
    /// The `of` keyword.
    OfKeyword,
    /// The `or` keyword.
    OrKeyword,
    /// The `out` keyword.
    OutKeyword,
    /// The `proc` keyword.
    ProcKeyword,
    /// The `ptr` keyword.
    PtrKeyword,
    /// The `raise` keyword.
    RaiseKeyword,
    /// The `ref` keyword.
    RefKeyword,
    /// The `return` keyword.
    ReturnKeyword,
    /// The `shl` keyword.
    ShlKeyword,
    /// The `shr` keyword.
    ShrKeyword,
    /// The `static` keyword.
    StaticKeyword,
    /// The `template` keyword.
    TemplateKeyword,
    /// The `try` keyword.
    TryKeyword,
    /// The `tuple` keyword.
    TupleKeyword,
    /// The `type` keyword.
    TypeKeyword,
    /// The `using` keyword.
    UsingKeyword,
    /// The `var` keyword.
    VarKeyword,
    /// The `when` keyword.
    WhenKeyword,
    /// The `while` keyword.
    WhileKeyword,
    /// The `xor` keyword.
    XorKeyword,
    /// The `yield` keyword.
    YieldKeyword,

    // Operators
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
    /// The `=` operator.
    Equal,
    /// The `==` operator.
    EqualEqual,
    /// The `!=` operator.
    NotEqual,
    /// The `<` operator.
    Less,
    /// The `<=` operator.
    LessEqual,
    /// The `>` operator.
    Greater,
    /// The `>=` operator.
    GreaterEqual,
    /// The `&` operator.
    Ampersand,
    /// The `|` operator.
    Pipe,
    /// The `^` operator.
    Caret,
    /// The `~` operator.
    Tilde,
    /// The `<<` operator.
    LeftShift,
    /// The `>>` operator.
    RightShift,
    /// The `..` operator.
    DotDot,
    /// The `->` operator.
    Arrow,
    /// The `@` operator.
    At,

    // Punctuation
    /// An opening parenthesis `(`.
    LeftParen,
    /// A closing parenthesis `)`.
    RightParen,
    /// An opening bracket `[`.
    LeftBracket,
    /// A closing bracket `]`.
    RightBracket,
    /// An opening brace `{`.
    LeftBrace,
    /// A closing brace `}`.
    RightBrace,
    /// A comma `,`.
    Comma,
    /// A semicolon `;`.
    Semicolon,
    /// A colon `:`.
    Colon,
    /// A dot `.`.
    Dot,
    /// A question mark `?`.
    Question,
    /// An exclamation mark `!`.
    Exclamation,
    /// A dollar sign `$`.
    Dollar,
    /// A backtick `` ` ``.
    Backtick,

    // Literals
    /// An integer literal.
    IntLiteral,
    /// A floating-point literal.
    FloatLiteral,
    /// A string literal.
    StringLiteral,
    /// A character literal.
    CharLiteral,
    /// A boolean literal.
    BoolLiteral,

    // Identifiers
    /// An identifier.
    Identifier,

    // Special
    /// Root node of the AST.
    Root,
    /// Procedure declaration.
    ProcDecl,
    /// Variable declaration.
    VarDecl,
    /// Let declaration.
    LetDecl,
    /// Constant declaration.
    ConstDecl,
    /// Type declaration.
    TypeDecl,
    /// If statement.
    IfStmt,
    /// While statement.
    WhileStmt,
    /// For statement.
    ForStmt,
    /// Case statement.
    CaseStmt,
    /// When statement (compile-time).
    WhenStmt,
    /// Static statement (compile-time).
    StaticStmt,
    /// Block statement.
    BlockStmt,
    /// An expression.
    Expression,
    /// A literal.
    Literal,
    /// A comment.
    Comment,
    /// Import declaration.
    ImportDecl,
    /// An error node in the AST.
    ErrorNode,
    /// An error element.
    Error,
    /// End of stream.
    Eof,
}

impl NimElementType {
    /// Returns true if the element type is a token.
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    /// Returns true if the element type is a non-terminal element.
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::ProcDecl | Self::TypeDecl | Self::VarDecl | Self::ConstDecl | Self::LetDecl | Self::ImportDecl | Self::Comment | Self::ErrorNode)
    }
}

impl ElementType for NimElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::ProcDecl | Self::VarDecl | Self::LetDecl | Self::ConstDecl | Self::TypeDecl => UniversalElementRole::Definition,
            Self::ImportDecl => UniversalElementRole::Metadata,
            Self::IfStmt | Self::WhileStmt | Self::ForStmt | Self::CaseStmt | Self::BlockStmt => UniversalElementRole::Statement,
            Self::Expression => UniversalElementRole::Expression,
            Self::Comment => UniversalElementRole::Documentation,
            Self::ErrorNode | Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::NimTokenType> for NimElementType {
    fn from(token: crate::lexer::token_type::NimTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
