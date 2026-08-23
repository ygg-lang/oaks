//! Element types for the D parser.

use oak_core::{ElementType, UniversalElementRole};

/// Element types for the D programming language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DElementType {
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
    /// A class definition.
    Class,
    /// A struct definition.
    Struct,
    /// An interface definition.
    Interface,
    /// A function definition.
    Function,
    /// A block statement.
    Block,
    /// An if statement.
    IfStatement,
    /// A while statement.
    WhileStatement,
    /// A for statement.
    ForStatement,
    /// A return statement.
    ReturnStatement,
    /// An expression statement.
    ExpressionStatement,
    /// A literal value.
    Literal,
    /// A parenthesized expression.
    ParenthesizedExpression,
    /// A binary expression.
    BinaryExpression,
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

impl ElementType for DElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,

            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::DTokenType> for DElementType {
    fn from(token: crate::lexer::token_type::DTokenType) -> Self {
                match token {
            crate::lexer::token_type::DTokenType::Root => Self::Root,
            crate::lexer::token_type::DTokenType::Module => Self::Module,
            crate::lexer::token_type::DTokenType::Declaration => Self::Declaration,
            crate::lexer::token_type::DTokenType::Statement => Self::Statement,
            crate::lexer::token_type::DTokenType::Expression => Self::Expression,
            crate::lexer::token_type::DTokenType::Type => Self::Type,
            crate::lexer::token_type::DTokenType::Aggregate => Self::Aggregate,
            crate::lexer::token_type::DTokenType::Import => Self::Import,
            crate::lexer::token_type::DTokenType::ModuleKeyword => Self::ModuleKeyword,
            crate::lexer::token_type::DTokenType::ImportKeyword => Self::ImportKeyword,
            crate::lexer::token_type::DTokenType::PublicKeyword => Self::PublicKeyword,
            crate::lexer::token_type::DTokenType::PrivateKeyword => Self::PrivateKeyword,
            crate::lexer::token_type::DTokenType::ProtectedKeyword => Self::ProtectedKeyword,
            crate::lexer::token_type::DTokenType::PackageKeyword => Self::PackageKeyword,
            crate::lexer::token_type::DTokenType::ExportKeyword => Self::ExportKeyword,
            crate::lexer::token_type::DTokenType::StaticKeyword => Self::StaticKeyword,
            crate::lexer::token_type::DTokenType::FinalKeyword => Self::FinalKeyword,
            crate::lexer::token_type::DTokenType::AbstractKeyword => Self::AbstractKeyword,
            crate::lexer::token_type::DTokenType::OverrideKeyword => Self::OverrideKeyword,
            crate::lexer::token_type::DTokenType::SynchronizedKeyword => Self::SynchronizedKeyword,
            crate::lexer::token_type::DTokenType::ConstKeyword => Self::ConstKeyword,
            crate::lexer::token_type::DTokenType::ImmutableKeyword => Self::ImmutableKeyword,
            crate::lexer::token_type::DTokenType::InoutKeyword => Self::InoutKeyword,
            crate::lexer::token_type::DTokenType::SharedKeyword => Self::SharedKeyword,
            crate::lexer::token_type::DTokenType::ClassKeyword => Self::ClassKeyword,
            crate::lexer::token_type::DTokenType::StructKeyword => Self::StructKeyword,
            crate::lexer::token_type::DTokenType::InterfaceKeyword => Self::InterfaceKeyword,
            crate::lexer::token_type::DTokenType::UnionKeyword => Self::UnionKeyword,
            crate::lexer::token_type::DTokenType::EnumKeyword => Self::EnumKeyword,
            crate::lexer::token_type::DTokenType::FunctionKeyword => Self::FunctionKeyword,
            crate::lexer::token_type::DTokenType::DelegateKeyword => Self::DelegateKeyword,
            crate::lexer::token_type::DTokenType::IfKeyword => Self::IfKeyword,
            crate::lexer::token_type::DTokenType::ElseKeyword => Self::ElseKeyword,
            crate::lexer::token_type::DTokenType::WhileKeyword => Self::WhileKeyword,
            crate::lexer::token_type::DTokenType::ForKeyword => Self::ForKeyword,
            crate::lexer::token_type::DTokenType::ForeachKeyword => Self::ForeachKeyword,
            crate::lexer::token_type::DTokenType::DoKeyword => Self::DoKeyword,
            crate::lexer::token_type::DTokenType::SwitchKeyword => Self::SwitchKeyword,
            crate::lexer::token_type::DTokenType::CaseKeyword => Self::CaseKeyword,
            crate::lexer::token_type::DTokenType::DefaultKeyword => Self::DefaultKeyword,
            crate::lexer::token_type::DTokenType::BreakKeyword => Self::BreakKeyword,
            crate::lexer::token_type::DTokenType::ContinueKeyword => Self::ContinueKeyword,
            crate::lexer::token_type::DTokenType::ReturnKeyword => Self::ReturnKeyword,
            crate::lexer::token_type::DTokenType::GotoKeyword => Self::GotoKeyword,
            crate::lexer::token_type::DTokenType::TryKeyword => Self::TryKeyword,
            crate::lexer::token_type::DTokenType::CatchKeyword => Self::CatchKeyword,
            crate::lexer::token_type::DTokenType::FinallyKeyword => Self::FinallyKeyword,
            crate::lexer::token_type::DTokenType::ThrowKeyword => Self::ThrowKeyword,
            crate::lexer::token_type::DTokenType::ScopeKeyword => Self::ScopeKeyword,
            crate::lexer::token_type::DTokenType::WithKeyword => Self::WithKeyword,
            crate::lexer::token_type::DTokenType::SynchronizedKeyword2 => Self::SynchronizedKeyword2,
            crate::lexer::token_type::DTokenType::AsmKeyword => Self::AsmKeyword,
            crate::lexer::token_type::DTokenType::MixinKeyword => Self::MixinKeyword,
            crate::lexer::token_type::DTokenType::TemplateKeyword => Self::TemplateKeyword,
            crate::lexer::token_type::DTokenType::ThisKeyword => Self::ThisKeyword,
            crate::lexer::token_type::DTokenType::SuperKeyword => Self::SuperKeyword,
            crate::lexer::token_type::DTokenType::NullKeyword => Self::NullKeyword,
            crate::lexer::token_type::DTokenType::TrueKeyword => Self::TrueKeyword,
            crate::lexer::token_type::DTokenType::FalseKeyword => Self::FalseKeyword,
            crate::lexer::token_type::DTokenType::CastKeyword => Self::CastKeyword,
            crate::lexer::token_type::DTokenType::NewKeyword => Self::NewKeyword,
            crate::lexer::token_type::DTokenType::DeleteKeyword => Self::DeleteKeyword,
            crate::lexer::token_type::DTokenType::TypeofKeyword => Self::TypeofKeyword,
            crate::lexer::token_type::DTokenType::TypeidKeyword => Self::TypeidKeyword,
            crate::lexer::token_type::DTokenType::IsKeyword => Self::IsKeyword,
            crate::lexer::token_type::DTokenType::InKeyword => Self::InKeyword,
            crate::lexer::token_type::DTokenType::OutKeyword => Self::OutKeyword,
            crate::lexer::token_type::DTokenType::RefKeyword => Self::RefKeyword,
            crate::lexer::token_type::DTokenType::LazyKeyword => Self::LazyKeyword,
            crate::lexer::token_type::DTokenType::AutoKeyword => Self::AutoKeyword,
            crate::lexer::token_type::DTokenType::AliasKeyword => Self::AliasKeyword,
            crate::lexer::token_type::DTokenType::TypedefKeyword => Self::TypedefKeyword,
            crate::lexer::token_type::DTokenType::ExternKeyword => Self::ExternKeyword,
            crate::lexer::token_type::DTokenType::PureKeyword => Self::PureKeyword,
            crate::lexer::token_type::DTokenType::NothrowKeyword => Self::NothrowKeyword,
            crate::lexer::token_type::DTokenType::SafeKeyword => Self::SafeKeyword,
            crate::lexer::token_type::DTokenType::TrustedKeyword => Self::TrustedKeyword,
            crate::lexer::token_type::DTokenType::SystemKeyword => Self::SystemKeyword,
            crate::lexer::token_type::DTokenType::NogcKeyword => Self::NogcKeyword,
            crate::lexer::token_type::DTokenType::PropertyKeyword => Self::PropertyKeyword,
            crate::lexer::token_type::DTokenType::DisableKeyword => Self::DisableKeyword,
            crate::lexer::token_type::DTokenType::DeprecatedKeyword => Self::DeprecatedKeyword,
            crate::lexer::token_type::DTokenType::VersionKeyword => Self::VersionKeyword,
            crate::lexer::token_type::DTokenType::DebugKeyword => Self::DebugKeyword,
            crate::lexer::token_type::DTokenType::UnitTestKeyword => Self::UnitTestKeyword,
            crate::lexer::token_type::DTokenType::InvariantKeyword => Self::InvariantKeyword,
            crate::lexer::token_type::DTokenType::BodyKeyword => Self::BodyKeyword,
            crate::lexer::token_type::DTokenType::PragmaKeyword => Self::PragmaKeyword,
            crate::lexer::token_type::DTokenType::AlignKeyword => Self::AlignKeyword,
            crate::lexer::token_type::DTokenType::VoidType => Self::VoidType,
            crate::lexer::token_type::DTokenType::BoolType => Self::BoolType,
            crate::lexer::token_type::DTokenType::ByteType => Self::ByteType,
            crate::lexer::token_type::DTokenType::UbyteType => Self::UbyteType,
            crate::lexer::token_type::DTokenType::ShortType => Self::ShortType,
            crate::lexer::token_type::DTokenType::UshortType => Self::UshortType,
            crate::lexer::token_type::DTokenType::IntType => Self::IntType,
            crate::lexer::token_type::DTokenType::UintType => Self::UintType,
            crate::lexer::token_type::DTokenType::LongType => Self::LongType,
            crate::lexer::token_type::DTokenType::UlongType => Self::UlongType,
            crate::lexer::token_type::DTokenType::CentType => Self::CentType,
            crate::lexer::token_type::DTokenType::UcentType => Self::UcentType,
            crate::lexer::token_type::DTokenType::FloatType => Self::FloatType,
            crate::lexer::token_type::DTokenType::DoubleType => Self::DoubleType,
            crate::lexer::token_type::DTokenType::RealType => Self::RealType,
            crate::lexer::token_type::DTokenType::IfloatType => Self::IfloatType,
            crate::lexer::token_type::DTokenType::IdoubleType => Self::IdoubleType,
            crate::lexer::token_type::DTokenType::IrealType => Self::IrealType,
            crate::lexer::token_type::DTokenType::CfloatType => Self::CfloatType,
            crate::lexer::token_type::DTokenType::CdoubleType => Self::CdoubleType,
            crate::lexer::token_type::DTokenType::CrealType => Self::CrealType,
            crate::lexer::token_type::DTokenType::CharType => Self::CharType,
            crate::lexer::token_type::DTokenType::WcharType => Self::WcharType,
            crate::lexer::token_type::DTokenType::DcharType => Self::DcharType,
            crate::lexer::token_type::DTokenType::StringType => Self::StringType,
            crate::lexer::token_type::DTokenType::WstringType => Self::WstringType,
            crate::lexer::token_type::DTokenType::DstringType => Self::DstringType,
            crate::lexer::token_type::DTokenType::Plus => Self::Plus,
            crate::lexer::token_type::DTokenType::Minus => Self::Minus,
            crate::lexer::token_type::DTokenType::Multiply => Self::Multiply,
            crate::lexer::token_type::DTokenType::Divide => Self::Divide,
            crate::lexer::token_type::DTokenType::Modulo => Self::Modulo,
            crate::lexer::token_type::DTokenType::BitwiseAnd => Self::BitwiseAnd,
            crate::lexer::token_type::DTokenType::BitwiseOr => Self::BitwiseOr,
            crate::lexer::token_type::DTokenType::BitwiseXor => Self::BitwiseXor,
            crate::lexer::token_type::DTokenType::BitwiseNot => Self::BitwiseNot,
            crate::lexer::token_type::DTokenType::LeftShift => Self::LeftShift,
            crate::lexer::token_type::DTokenType::RightShift => Self::RightShift,
            crate::lexer::token_type::DTokenType::UnsignedRightShift => Self::UnsignedRightShift,
            crate::lexer::token_type::DTokenType::Equal => Self::Equal,
            crate::lexer::token_type::DTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::DTokenType::Less => Self::Less,
            crate::lexer::token_type::DTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::DTokenType::Greater => Self::Greater,
            crate::lexer::token_type::DTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::DTokenType::Identity => Self::Identity,
            crate::lexer::token_type::DTokenType::NotIdentity => Self::NotIdentity,
            crate::lexer::token_type::DTokenType::Assign => Self::Assign,
            crate::lexer::token_type::DTokenType::PlusAssign => Self::PlusAssign,
            crate::lexer::token_type::DTokenType::MinusAssign => Self::MinusAssign,
            crate::lexer::token_type::DTokenType::MultiplyAssign => Self::MultiplyAssign,
            crate::lexer::token_type::DTokenType::DivideAssign => Self::DivideAssign,
            crate::lexer::token_type::DTokenType::ModuloAssign => Self::ModuloAssign,
            crate::lexer::token_type::DTokenType::BitwiseAndAssign => Self::BitwiseAndAssign,
            crate::lexer::token_type::DTokenType::BitwiseOrAssign => Self::BitwiseOrAssign,
            crate::lexer::token_type::DTokenType::BitwiseXorAssign => Self::BitwiseXorAssign,
            crate::lexer::token_type::DTokenType::LeftShiftAssign => Self::LeftShiftAssign,
            crate::lexer::token_type::DTokenType::RightShiftAssign => Self::RightShiftAssign,
            crate::lexer::token_type::DTokenType::UnsignedRightShiftAssign => Self::UnsignedRightShiftAssign,
            crate::lexer::token_type::DTokenType::ConcatenateAssign => Self::ConcatenateAssign,
            crate::lexer::token_type::DTokenType::LogicalAnd => Self::LogicalAnd,
            crate::lexer::token_type::DTokenType::LogicalOr => Self::LogicalOr,
            crate::lexer::token_type::DTokenType::Increment => Self::Increment,
            crate::lexer::token_type::DTokenType::Decrement => Self::Decrement,
            crate::lexer::token_type::DTokenType::Not => Self::Not,
            crate::lexer::token_type::DTokenType::Question => Self::Question,
            crate::lexer::token_type::DTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::DTokenType::At => Self::At,
            crate::lexer::token_type::DTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::DTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::DTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::DTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::DTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::DTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::DTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::DTokenType::Comma => Self::Comma,
            crate::lexer::token_type::DTokenType::Dot => Self::Dot,
            crate::lexer::token_type::DTokenType::DotDot => Self::DotDot,
            crate::lexer::token_type::DTokenType::DotDotDot => Self::DotDotDot,
            crate::lexer::token_type::DTokenType::Colon => Self::Colon,
            crate::lexer::token_type::DTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::DTokenType::Hash => Self::Hash,
            crate::lexer::token_type::DTokenType::IntegerLiteral => Self::IntegerLiteral,
            crate::lexer::token_type::DTokenType::FloatLiteral => Self::FloatLiteral,
            crate::lexer::token_type::DTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::DTokenType::CharLiteral => Self::CharLiteral,
            crate::lexer::token_type::DTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::DTokenType::LineComment => Self::LineComment,
            crate::lexer::token_type::DTokenType::BlockComment => Self::BlockComment,
            crate::lexer::token_type::DTokenType::NestedComment => Self::NestedComment,
            crate::lexer::token_type::DTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::DTokenType::Newline => Self::Newline,
            crate::lexer::token_type::DTokenType::Eof => Self::Eof,
            crate::lexer::token_type::DTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
