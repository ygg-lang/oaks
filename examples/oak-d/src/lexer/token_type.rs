//! Token types for the D lexer.

use oak_core::{Token, TokenType, UniversalTokenRole};

/// A token for the D programming language.
pub type DToken = Token<DTokenType>;

/// Token types for the D programming language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DTokenType {
    /// The root of the parse tree.
    Root,
    /// A module declaration.
    Module,
    /// A declaration.
    Declaration,
    /// A statement.
    Statement,
    /// An expression.
    Expression,
    /// A type.
    Type,
    /// An aggregate (class, struct, etc.).
    Aggregate,
    /// An import declaration.
    Import,
    /// The `module` keyword.
    ModuleKeyword,
    /// The `import` keyword.
    ImportKeyword,
    /// The `public` keyword.
    PublicKeyword,
    /// The `private` keyword.
    PrivateKeyword,
    /// The `protected` keyword.
    ProtectedKeyword,
    /// The `package` keyword.
    PackageKeyword,
    /// The `export` keyword.
    ExportKeyword,
    /// The `static` keyword.
    StaticKeyword,
    /// The `final` keyword.
    FinalKeyword,
    /// The `abstract` keyword.
    AbstractKeyword,
    /// The `override` keyword.
    OverrideKeyword,
    /// The `synchronized` keyword.
    SynchronizedKeyword,
    /// The `const` keyword.
    ConstKeyword,
    /// The `immutable` keyword.
    ImmutableKeyword,
    /// The `inout` keyword.
    InoutKeyword,
    /// The `shared` keyword.
    SharedKeyword,
    /// The `class` keyword.
    ClassKeyword,
    /// The `struct` keyword.
    StructKeyword,
    /// The `interface` keyword.
    InterfaceKeyword,
    /// The `union` keyword.
    UnionKeyword,
    /// The `enum` keyword.
    EnumKeyword,
    /// The `function` keyword.
    FunctionKeyword,
    /// The `delegate` keyword.
    DelegateKeyword,
    /// The `if` keyword.
    IfKeyword,
    /// The `else` keyword.
    ElseKeyword,
    /// The `while` keyword.
    WhileKeyword,
    /// The `for` keyword.
    ForKeyword,
    /// The `foreach` keyword.
    ForeachKeyword,
    /// The `do` keyword.
    DoKeyword,
    /// The `switch` keyword.
    SwitchKeyword,
    /// The `case` keyword.
    CaseKeyword,
    /// The `default` keyword.
    DefaultKeyword,
    /// The `break` keyword.
    BreakKeyword,
    /// The `continue` keyword.
    ContinueKeyword,
    /// The `return` keyword.
    ReturnKeyword,
    /// The `goto` keyword.
    GotoKeyword,
    /// The `try` keyword.
    TryKeyword,
    /// The `catch` keyword.
    CatchKeyword,
    /// The `finally` keyword.
    FinallyKeyword,
    /// The `throw` keyword.
    ThrowKeyword,
    /// The `scope` keyword.
    ScopeKeyword,
    /// The `with` keyword.
    WithKeyword,
    /// Another `synchronized` keyword variant.
    SynchronizedKeyword2,
    /// The `asm` keyword.
    AsmKeyword,
    /// The `mixin` keyword.
    MixinKeyword,
    /// The `template` keyword.
    TemplateKeyword,
    /// The `this` keyword.
    ThisKeyword,
    /// The `super` keyword.
    SuperKeyword,
    /// The `null` keyword.
    NullKeyword,
    /// The `true` keyword.
    TrueKeyword,
    /// The `false` keyword.
    FalseKeyword,
    /// The `cast` keyword.
    CastKeyword,
    /// The `new` keyword.
    NewKeyword,
    /// The `delete` keyword.
    DeleteKeyword,
    /// The `typeof` keyword.
    TypeofKeyword,
    /// The `typeid` keyword.
    TypeidKeyword,
    /// The `is` keyword.
    IsKeyword,
    /// The `in` keyword.
    InKeyword,
    /// The `out` keyword.
    OutKeyword,
    /// The `ref` keyword.
    RefKeyword,
    /// The `lazy` keyword.
    LazyKeyword,
    /// The `auto` keyword.
    AutoKeyword,
    /// The `alias` keyword.
    AliasKeyword,
    /// The `typedef` keyword.
    TypedefKeyword,
    /// The `extern` keyword.
    ExternKeyword,
    /// The `pure` keyword.
    PureKeyword,
    /// The `nothrow` keyword.
    NothrowKeyword,
    /// The `safe` keyword.
    SafeKeyword,
    /// The `trusted` keyword.
    TrustedKeyword,
    /// The `system` keyword.
    SystemKeyword,
    /// The `nogc` keyword.
    NogcKeyword,
    /// The `property` keyword.
    PropertyKeyword,
    /// The `disable` keyword.
    DisableKeyword,
    /// The `deprecated` keyword.
    DeprecatedKeyword,
    /// The `version` keyword.
    VersionKeyword,
    /// The `debug` keyword.
    DebugKeyword,
    /// The `unittest` keyword.
    UnitTestKeyword,
    /// The `invariant` keyword.
    InvariantKeyword,
    /// The `body` keyword.
    BodyKeyword,
    /// The `pragma` keyword.
    PragmaKeyword,
    /// The `align` keyword.
    AlignKeyword,
    /// The `void` type.
    VoidType,
    /// The `bool` type.
    BoolType,
    /// The `byte` type.
    ByteType,
    /// The `ubyte` type.
    UbyteType,
    /// The `short` type.
    ShortType,
    /// The `ushort` type.
    UshortType,
    /// The `int` type.
    IntType,
    /// The `uint` type.
    UintType,
    /// The `long` type.
    LongType,
    /// The `ulong` type.
    UlongType,
    /// The `cent` type.
    CentType,
    /// The `ucent` type.
    UcentType,
    /// The `float` type.
    FloatType,
    /// The `double` type.
    DoubleType,
    /// The `real` type.
    RealType,
    /// The `ifloat` type.
    IfloatType,
    /// The `idouble` type.
    IdoubleType,
    /// The `ireal` type.
    IrealType,
    /// The `cfloat` type.
    CfloatType,
    /// The `cdouble` type.
    CdoubleType,
    /// The `creal` type.
    CrealType,
    /// The `char` type.
    CharType,
    /// The `wchar` type.
    WcharType,
    /// The `dchar` type.
    DcharType,
    /// The `string` type.
    StringType,
    /// The `wstring` type.
    WstringType,
    /// The `dstring` type.
    DstringType,
    /// The `+` operator.
    Plus,
    /// The `-` operator.
    Minus,
    /// The `*` operator.
    Multiply,
    /// The `/` operator.
    Divide,
    /// The `%` operator.
    Modulo,
    /// The `&` operator.
    BitwiseAnd,
    /// The `|` operator.
    BitwiseOr,
    /// The `^` operator.
    BitwiseXor,
    /// The `~` operator.
    BitwiseNot,
    /// The `<<` operator.
    LeftShift,
    /// The `>>` operator.
    RightShift,
    /// The `>>>` operator.
    UnsignedRightShift,
    /// The `==` operator.
    Equal,
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
    /// The `is` operator.
    Identity,
    /// The `!is` operator.
    NotIdentity,
    /// The `=` operator.
    Assign,
    /// The `+=` operator.
    PlusAssign,
    /// The `-=` operator.
    MinusAssign,
    /// The `*=` operator.
    MultiplyAssign,
    /// The `/=` operator.
    DivideAssign,
    /// The `%=` operator.
    ModuloAssign,
    /// The `&=` operator.
    BitwiseAndAssign,
    /// The `|=` operator.
    BitwiseOrAssign,
    /// The `^=` operator.
    BitwiseXorAssign,
    /// The `<<=` operator.
    LeftShiftAssign,
    /// The `>>=` operator.
    RightShiftAssign,
    /// The `>>>=` operator.
    UnsignedRightShiftAssign,
    /// The `~=` operator.
    ConcatenateAssign,
    /// The `&&` operator.
    LogicalAnd,
    /// The `||` operator.
    LogicalOr,
    /// The `++` operator.
    Increment,
    /// The `--` operator.
    Decrement,
    /// The `!` operator.
    Not,
    /// The `?` operator.
    Question,
    /// The `$` operator.
    Dollar,
    /// The `@` operator.
    At,
    /// Opening parenthesis (`(`).
    LeftParen,
    /// Closing parenthesis (`)`).
    RightParen,
    /// Opening bracket (`[`).
    LeftBracket,
    /// Closing bracket (`]`).
    RightBracket,
    /// Opening brace (`{`).
    LeftBrace,
    /// Closing brace (`}`).
    RightBrace,
    /// Semicolon (`;`).
    Semicolon,
    /// Comma (`,`).
    Comma,
    /// Dot (`.`).
    Dot,
    /// Double dot (`..`).
    DotDot,
    /// Triple dot (`...`).
    DotDotDot,
    /// Colon (`:`).
    Colon,
    /// Arrow (`->`).
    Arrow,
    /// Hash (`#`).
    Hash,
    /// An integer literal.
    IntegerLiteral,
    /// A floating-point literal.
    FloatLiteral,
    /// A string literal.
    StringLiteral,
    /// A character literal.
    CharLiteral,
    /// An identifier.
    Identifier,
    /// A line comment.
    LineComment,
    /// A block comment.
    BlockComment,
    /// A nested comment.
    NestedComment,
    /// Whitespace.
    Whitespace,
    /// A newline.
    Newline,
    /// End of stream.
    Eof,
    /// An error.
    Error,
}

impl TokenType for DTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment => UniversalTokenRole::Comment,
            Self::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
