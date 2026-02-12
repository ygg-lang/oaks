//! Element types for the Groovy language.

use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Groovy AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum GroovyElementType {
    /// Root node.
    Root,
    /// Source file.
    SourceFile,
    // --- Literals ---
    /// Integer literal.
    IntLiteral,
    /// Floating point literal.
    FloatLiteral,
    /// String literal.
    StringLiteral,
    /// Character literal.
    CharLiteral,
    /// Boolean literal.
    BooleanLiteral,
    /// Null literal.
    NullLiteral,

    // --- Identifiers ---
    /// Identifier.
    Identifier,

    // --- Keywords ---
    /// `abstract` keyword.
    AbstractKeyword,
    /// `as` keyword.
    AsKeyword,
    /// `assert` keyword.
    AssertKeyword,
    /// `break` keyword.
    BreakKeyword,
    /// `case` keyword.
    CaseKeyword,
    /// `catch` keyword.
    CatchKeyword,
    /// `class` keyword.
    ClassKeyword,
    /// `const` keyword.
    ConstKeyword,
    /// `continue` keyword.
    ContinueKeyword,
    /// `def` keyword.
    DefKeyword,
    /// `default` keyword.
    DefaultKeyword,
    /// `do` keyword.
    DoKeyword,
    /// `else` keyword.
    ElseKeyword,
    /// `enum` keyword.
    EnumKeyword,
    /// `extends` keyword.
    ExtendsKeyword,
    /// `final` keyword.
    FinalKeyword,
    /// `finally` keyword.
    FinallyKeyword,
    /// `for` keyword.
    ForKeyword,
    /// `goto` keyword.
    GotoKeyword,
    /// `if` keyword.
    IfKeyword,
    /// `implements` keyword.
    ImplementsKeyword,
    /// `import` keyword.
    ImportKeyword,
    /// `in` keyword.
    InKeyword,
    /// `instanceof` keyword.
    InstanceofKeyword,
    /// `interface` keyword.
    InterfaceKeyword,
    /// `native` keyword.
    NativeKeyword,
    /// `new` keyword.
    NewKeyword,
    /// `package` keyword.
    PackageKeyword,
    /// `private` keyword.
    PrivateKeyword,
    /// `protected` keyword.
    ProtectedKeyword,
    /// `public` keyword.
    PublicKeyword,
    /// `return` keyword.
    ReturnKeyword,
    /// `static` keyword.
    StaticKeyword,
    /// `strictfp` keyword.
    StrictfpKeyword,
    /// `super` keyword.
    SuperKeyword,
    /// `switch` keyword.
    SwitchKeyword,
    /// `synchronized` keyword.
    SynchronizedKeyword,
    /// `this` keyword.
    ThisKeyword,
    /// `throw` keyword.
    ThrowKeyword,
    /// `throws` keyword.
    ThrowsKeyword,
    /// `trait` keyword.
    TraitKeyword,
    /// `transient` keyword.
    TransientKeyword,
    /// `try` keyword.
    TryKeyword,
    /// `void` keyword.
    VoidKeyword,
    /// `volatile` keyword.
    VolatileKeyword,
    /// `while` keyword.
    WhileKeyword,

    // --- Operators ---
    /// Plus `+`.
    Plus,
    /// Minus `-`.
    Minus,
    /// Star `*`.
    Star,
    /// Slash `/`.
    Slash,
    /// Percent `%`.
    Percent,
    /// Power `**`.
    Power,

    /// Assign `=`.
    Assign,
    /// Plus assign `+=`.
    PlusAssign,
    /// Minus assign `-=`.
    MinusAssign,
    /// Star assign `*=`.
    StarAssign,
    /// Slash assign `/=`.
    SlashAssign,
    /// Percent assign `%=`.
    PercentAssign,
    /// Power assign `**=`.
    PowerAssign,

    /// Equal `==`.
    Equal,
    /// Not equal `!=`.
    NotEqual,
    /// Less than `<`.
    Less,
    /// Greater than `>`.
    Greater,
    /// Less than or equal `<=`.
    LessEqual,
    /// Greater than or equal `>=`.
    GreaterEqual,
    /// Spaceship operator `<=>`.
    Spaceship,

    /// Logical AND `&&`.
    LogicalAnd,
    /// Logical OR `||`.
    LogicalOr,
    /// Logical NOT `!`.
    LogicalNot,

    /// Bitwise AND `&`.
    BitAnd,
    /// Bitwise OR `|`.
    BitOr,
    /// Bitwise XOR `^`.
    BitXor,
    /// Bitwise NOT `~`.
    BitNot,
    /// Left shift `<<`.
    LeftShift,
    /// Right shift `>>`.
    RightShift,
    /// Unsigned right shift `>>>`.
    UnsignedRightShift,

    /// Increment `++`.
    Increment,
    /// Decrement `--`.
    Decrement,

    /// Question mark `?`.
    Question,
    /// Colon `:`.
    Colon,
    /// Elvis operator `?:`.
    Elvis,
    /// Safe navigation operator `?.`.
    SafeNavigation,

    // --- Delimiters ---
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Comma `,`.
    Comma,
    /// Period `.`.
    Period,
    /// Semicolon `;`.
    Semicolon,
    /// At symbol `@`.
    At,

    // --- Whitespace and Comments ---
    /// Whitespace.
    Whitespace,
    /// Comment.
    Comment,

    // --- Special ---
    /// Newline.
    Newline,
    /// End of file.
    Eof,
    /// Error token.
    Error,
}

impl ElementType for GroovyElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::GroovyTokenType> for GroovyElementType {
    fn from(token: crate::lexer::token_type::GroovyTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
