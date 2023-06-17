use oak_core::SyntaxKind;

/// Groovy 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GroovySyntaxKind {
    // 字面量
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BooleanLiteral,
    NullLiteral,

    // 标识符
    Identifier,

    // 关键字
    AbstractKeyword,
    AsKeyword,
    AssertKeyword,
    BreakKeyword,
    CaseKeyword,
    CatchKeyword,
    ClassKeyword,
    ConstKeyword,
    ContinueKeyword,
    DefKeyword,
    DefaultKeyword,
    DoKeyword,
    ElseKeyword,
    EnumKeyword,
    ExtendsKeyword,
    FinalKeyword,
    FinallyKeyword,
    ForKeyword,
    GotoKeyword,
    IfKeyword,
    ImplementsKeyword,
    ImportKeyword,
    InKeyword,
    InstanceofKeyword,
    InterfaceKeyword,
    NativeKeyword,
    NewKeyword,
    PackageKeyword,
    PrivateKeyword,
    ProtectedKeyword,
    PublicKeyword,
    ReturnKeyword,
    StaticKeyword,
    StrictfpKeyword,
    SuperKeyword,
    SwitchKeyword,
    SynchronizedKeyword,
    ThisKeyword,
    ThrowKeyword,
    ThrowsKeyword,
    TraitKeyword,
    TransientKeyword,
    TryKeyword,
    VoidKeyword,
    VolatileKeyword,
    WhileKeyword,

    // 操作符
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    Power,   // **

    Assign,        // =
    PlusAssign,    // +=
    MinusAssign,   // -=
    StarAssign,    // *=
    SlashAssign,   // /=
    PercentAssign, // %=
    PowerAssign,   // **=

    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
    Spaceship,    // <=>

    LogicalAnd, // &&
    LogicalOr,  // ||
    LogicalNot, // !

    BitAnd,             // &
    BitOr,              // |
    BitXor,             // ^
    BitNot,             // ~
    LeftShift,          // <<
    RightShift,         // >>
    UnsignedRightShift, // >>>

    Increment, // ++
    Decrement, // --

    Question,       // ?
    Colon,          // :
    Elvis,          // ?:
    SafeNavigation, // ?.

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Period,       // .
    Semicolon,    // ;
    At,           // @

    // 空白和注释
    Whitespace,
    Comment,

    // 特殊
    Newline,
    Eof,
    Error,
}

impl SyntaxKind for GroovySyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Newline)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error | Self::Eof)
    }

    fn is_element_type(&self) -> bool {
        false
    }
}
