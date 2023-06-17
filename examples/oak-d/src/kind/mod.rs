use oak_core::SyntaxKind;

/// D 语言语法种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DSyntaxKind {
    // 节点类型
    Root,
    Module,
    Declaration,
    Statement,
    Expression,
    Type,

    // 关键字
    ModuleKeyword,
    ImportKeyword,
    PublicKeyword,
    PrivateKeyword,
    ProtectedKeyword,
    PackageKeyword,
    ExportKeyword,
    StaticKeyword,
    FinalKeyword,
    AbstractKeyword,
    OverrideKeyword,
    SynchronizedKeyword,
    ConstKeyword,
    ImmutableKeyword,
    InoutKeyword,
    SharedKeyword,
    ClassKeyword,
    StructKeyword,
    InterfaceKeyword,
    UnionKeyword,
    EnumKeyword,
    FunctionKeyword,
    DelegateKeyword,
    IfKeyword,
    ElseKeyword,
    WhileKeyword,
    ForKeyword,
    ForeachKeyword,
    DoKeyword,
    SwitchKeyword,
    CaseKeyword,
    DefaultKeyword,
    BreakKeyword,
    ContinueKeyword,
    ReturnKeyword,
    GotoKeyword,
    TryKeyword,
    CatchKeyword,
    FinallyKeyword,
    ThrowKeyword,
    ScopeKeyword,
    WithKeyword,
    SynchronizedKeyword2,
    AsmKeyword,
    MixinKeyword,
    TemplateKeyword,
    ThisKeyword,
    SuperKeyword,
    NullKeyword,
    TrueKeyword,
    FalseKeyword,
    CastKeyword,
    NewKeyword,
    DeleteKeyword,
    TypeofKeyword,
    TypeidKeyword,
    IsKeyword,
    InKeyword,
    OutKeyword,
    RefKeyword,
    LazyKeyword,
    AutoKeyword,
    AliasKeyword,
    TypedefKeyword,
    ExternKeyword,
    PureKeyword,
    NothrowKeyword,
    SafeKeyword,
    TrustedKeyword,
    SystemKeyword,
    NogcKeyword,
    PropertyKeyword,
    DisableKeyword,
    DeprecatedKeyword,
    VersionKeyword,
    DebugKeyword,
    UnitTestKeyword,
    InvariantKeyword,
    BodyKeyword,
    PragmaKeyword,
    AlignKeyword,

    // 基本类型
    VoidType,
    BoolType,
    ByteType,
    UbyteType,
    ShortType,
    UshortType,
    IntType,
    UintType,
    LongType,
    UlongType,
    CentType,
    UcentType,
    FloatType,
    DoubleType,
    RealType,
    IfloatType,
    IdoubleType,
    IrealType,
    CfloatType,
    CdoubleType,
    CrealType,
    CharType,
    WcharType,
    DcharType,
    StringType,
    WstringType,
    DstringType,

    // 操作符
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Identity,
    NotIdentity,
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    ConcatenateAssign,
    LogicalAnd,
    LogicalOr,
    Increment,
    Decrement,
    Not,
    Question,
    Dollar,
    At,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Dot,
    DotDot,
    DotDotDot,
    Colon,
    Arrow,
    Hash,

    // 字面量
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,

    // 标识符和注释
    Identifier,
    LineComment,
    BlockComment,
    NestedComment,

    // 空白和特殊
    Whitespace,
    Newline,
    Eof,
    Error,
}

impl SyntaxKind for DSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment | Self::NestedComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        matches!(
            self,
            Self::ModuleKeyword
                | Self::ImportKeyword
                | Self::PublicKeyword
                | Self::PrivateKeyword
                | Self::ProtectedKeyword
                | Self::PackageKeyword
                | Self::ExportKeyword
                | Self::StaticKeyword
                | Self::FinalKeyword
                | Self::AbstractKeyword
                | Self::OverrideKeyword
                | Self::SynchronizedKeyword
                | Self::ConstKeyword
                | Self::ImmutableKeyword
                | Self::InoutKeyword
                | Self::SharedKeyword
                | Self::ClassKeyword
                | Self::StructKeyword
                | Self::InterfaceKeyword
                | Self::UnionKeyword
                | Self::EnumKeyword
                | Self::FunctionKeyword
                | Self::DelegateKeyword
                | Self::IfKeyword
                | Self::ElseKeyword
                | Self::WhileKeyword
                | Self::ForKeyword
                | Self::ForeachKeyword
                | Self::DoKeyword
                | Self::SwitchKeyword
                | Self::CaseKeyword
                | Self::DefaultKeyword
                | Self::BreakKeyword
                | Self::ContinueKeyword
                | Self::ReturnKeyword
                | Self::GotoKeyword
                | Self::TryKeyword
                | Self::CatchKeyword
                | Self::FinallyKeyword
                | Self::ThrowKeyword
                | Self::ScopeKeyword
                | Self::WithKeyword
                | Self::SynchronizedKeyword2
                | Self::AsmKeyword
                | Self::MixinKeyword
                | Self::TemplateKeyword
                | Self::ThisKeyword
                | Self::SuperKeyword
                | Self::NullKeyword
                | Self::TrueKeyword
                | Self::FalseKeyword
                | Self::CastKeyword
                | Self::NewKeyword
                | Self::DeleteKeyword
                | Self::TypeofKeyword
                | Self::TypeidKeyword
                | Self::IsKeyword
                | Self::InKeyword
                | Self::OutKeyword
                | Self::RefKeyword
                | Self::LazyKeyword
                | Self::AutoKeyword
                | Self::AliasKeyword
                | Self::TypedefKeyword
                | Self::ExternKeyword
                | Self::PureKeyword
                | Self::NothrowKeyword
                | Self::SafeKeyword
                | Self::TrustedKeyword
                | Self::SystemKeyword
                | Self::NogcKeyword
                | Self::PropertyKeyword
                | Self::DisableKeyword
                | Self::DeprecatedKeyword
                | Self::VersionKeyword
                | Self::DebugKeyword
                | Self::UnitTestKeyword
                | Self::InvariantKeyword
                | Self::BodyKeyword
                | Self::PragmaKeyword
                | Self::AlignKeyword
                | Self::VoidType
                | Self::BoolType
                | Self::ByteType
                | Self::UbyteType
                | Self::ShortType
                | Self::UshortType
                | Self::IntType
                | Self::UintType
                | Self::LongType
                | Self::UlongType
                | Self::CentType
                | Self::UcentType
                | Self::FloatType
                | Self::DoubleType
                | Self::RealType
                | Self::IfloatType
                | Self::IdoubleType
                | Self::IrealType
                | Self::CfloatType
                | Self::CdoubleType
                | Self::CrealType
                | Self::CharType
                | Self::WcharType
                | Self::DcharType
                | Self::StringType
                | Self::WstringType
                | Self::DstringType
                | Self::Plus
                | Self::Minus
                | Self::Multiply
                | Self::Divide
                | Self::Modulo
                | Self::BitwiseAnd
                | Self::BitwiseOr
                | Self::BitwiseXor
                | Self::BitwiseNot
                | Self::LeftShift
                | Self::RightShift
                | Self::UnsignedRightShift
                | Self::Equal
                | Self::NotEqual
                | Self::Less
                | Self::LessEqual
                | Self::Greater
                | Self::GreaterEqual
                | Self::Identity
                | Self::NotIdentity
                | Self::Assign
                | Self::PlusAssign
                | Self::MinusAssign
                | Self::MultiplyAssign
                | Self::DivideAssign
                | Self::ModuloAssign
                | Self::BitwiseAndAssign
                | Self::BitwiseOrAssign
                | Self::BitwiseXorAssign
                | Self::LeftShiftAssign
                | Self::RightShiftAssign
                | Self::UnsignedRightShiftAssign
                | Self::ConcatenateAssign
                | Self::LogicalAnd
                | Self::LogicalOr
                | Self::Increment
                | Self::Decrement
                | Self::Not
                | Self::Question
                | Self::Dollar
                | Self::At
                | Self::LeftParen
                | Self::RightParen
                | Self::LeftBracket
                | Self::RightBracket
                | Self::LeftBrace
                | Self::RightBrace
                | Self::Semicolon
                | Self::Comma
                | Self::Dot
                | Self::DotDot
                | Self::DotDotDot
                | Self::Colon
                | Self::Arrow
                | Self::Hash
                | Self::IntegerLiteral
                | Self::FloatLiteral
                | Self::StringLiteral
                | Self::CharLiteral
                | Self::Identifier
                | Self::LineComment
                | Self::BlockComment
                | Self::NestedComment
                | Self::Whitespace
                | Self::Newline
                | Self::Eof
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Root | Self::Module | Self::Declaration | Self::Statement | Self::Expression | Self::Type)
    }
}
