use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};

/// Token type for D language syntax
pub type DToken = Token<DSyntaxKind>;

/// D language syntax kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DSyntaxKind {
    /// Root node of the syntax tree
    Root,
    /// Module declaration
    Module,
    /// Declaration statement
    Declaration,
    /// Statement
    Statement,
    /// Expression
    Expression,
    /// type specification
    Type,
    /// Aggregate declaration
    Aggregate,
    /// Import declaration
    Import,
    /// module keyword
    ModuleKeyword,
    /// import keyword
    ImportKeyword,
    /// public keyword
    PublicKeyword,
    /// private keyword
    PrivateKeyword,
    /// protected keyword
    ProtectedKeyword,
    /// package keyword
    PackageKeyword,
    /// export keyword
    ExportKeyword,
    /// static keyword
    StaticKeyword,
    /// final keyword
    FinalKeyword,
    /// abstract keyword
    AbstractKeyword,
    /// override keyword
    OverrideKeyword,
    /// synchronized keyword
    SynchronizedKeyword,
    /// const keyword
    ConstKeyword,
    /// immutable keyword
    ImmutableKeyword,
    /// inout keyword
    InoutKeyword,
    /// shared keyword
    SharedKeyword,
    /// class keyword
    ClassKeyword,
    /// struct keyword
    StructKeyword,
    /// interface keyword
    InterfaceKeyword,
    /// union keyword
    UnionKeyword,
    /// enum keyword
    EnumKeyword,
    /// function keyword
    FunctionKeyword,
    /// delegate keyword
    DelegateKeyword,
    /// if keyword
    IfKeyword,
    /// else keyword
    ElseKeyword,
    /// while keyword
    WhileKeyword,
    /// for keyword
    ForKeyword,
    /// foreach keyword
    ForeachKeyword,
    /// do keyword
    DoKeyword,
    /// switch keyword
    SwitchKeyword,
    /// case keyword
    CaseKeyword,
    /// default keyword
    DefaultKeyword,
    /// break keyword
    BreakKeyword,
    /// continue keyword
    ContinueKeyword,
    /// return keyword
    ReturnKeyword,
    /// goto keyword
    GotoKeyword,
    /// try keyword
    TryKeyword,
    /// catch keyword
    CatchKeyword,
    /// finally keyword
    FinallyKeyword,
    /// throw keyword
    ThrowKeyword,
    /// scope keyword
    ScopeKeyword,
    /// with keyword
    WithKeyword,
    /// synchronized keyword (second occurrence)
    SynchronizedKeyword2,
    /// asm keyword
    AsmKeyword,
    /// mixin keyword
    MixinKeyword,
    /// template keyword
    TemplateKeyword,
    /// this keyword
    ThisKeyword,
    /// super keyword
    SuperKeyword,
    /// null keyword
    NullKeyword,
    /// true keyword
    TrueKeyword,
    /// false keyword
    FalseKeyword,
    /// cast keyword
    CastKeyword,
    /// new keyword
    NewKeyword,
    /// delete keyword
    DeleteKeyword,
    /// typeof keyword
    TypeofKeyword,
    /// typeid keyword
    TypeidKeyword,
    /// is keyword
    IsKeyword,
    /// in keyword
    InKeyword,
    /// out keyword
    OutKeyword,
    /// ref keyword
    RefKeyword,
    /// lazy keyword
    LazyKeyword,
    /// auto keyword
    AutoKeyword,
    /// alias keyword
    AliasKeyword,
    /// typedef keyword
    TypedefKeyword,
    /// extern keyword
    ExternKeyword,
    /// pure keyword
    PureKeyword,
    /// nothrow keyword
    NothrowKeyword,
    /// safe keyword
    SafeKeyword,
    /// trusted keyword
    TrustedKeyword,
    /// system keyword
    SystemKeyword,
    /// nogc keyword
    NogcKeyword,
    /// property keyword
    PropertyKeyword,
    /// disable keyword
    DisableKeyword,
    /// deprecated keyword
    DeprecatedKeyword,
    /// version keyword
    VersionKeyword,
    /// debug keyword
    DebugKeyword,
    /// unittest keyword
    UnitTestKeyword,
    /// invariant keyword
    InvariantKeyword,
    /// body keyword
    BodyKeyword,
    /// pragma keyword
    PragmaKeyword,
    /// align keyword
    AlignKeyword,

    // 基本类型
    /// void type
    VoidType,
    /// bool type
    BoolType,
    /// byte type
    ByteType,
    /// ubyte type
    UbyteType,
    /// short type
    ShortType,
    /// ushort type
    UshortType,
    /// int type
    IntType,
    /// uint type
    UintType,
    /// long type
    LongType,
    /// ulong type
    UlongType,
    /// cent type
    CentType,
    /// ucent type
    UcentType,
    /// float type
    FloatType,
    /// double type
    DoubleType,
    /// real type
    RealType,
    /// ifloat type
    IfloatType,
    /// idouble type
    IdoubleType,
    /// ireal type
    IrealType,
    /// cfloat type
    CfloatType,
    /// cdouble type
    CdoubleType,
    /// creal type
    CrealType,
    /// char type
    CharType,
    /// wchar type
    WcharType,
    /// dchar type
    DcharType,
    /// string type
    StringType,
    /// wstring type
    WstringType,
    /// dstring type
    DstringType,

    // 操作符
    /// plus operator
    Plus,
    /// minus operator
    Minus,
    /// multiply operator
    Multiply,
    /// divide operator
    Divide,
    /// modulo operator
    Modulo,
    /// bitwise and operator
    BitwiseAnd,
    /// bitwise or operator
    BitwiseOr,
    /// bitwise xor operator
    BitwiseXor,
    /// bitwise not operator
    BitwiseNot,
    /// left shift operator
    LeftShift,
    /// right shift operator
    RightShift,
    /// unsigned right shift operator
    UnsignedRightShift,
    /// equal operator
    Equal,
    /// not equal operator
    NotEqual,
    /// less than operator
    Less,
    /// less than or equal operator
    LessEqual,
    /// greater than operator
    Greater,
    /// greater than or equal operator
    GreaterEqual,
    /// identity operator
    Identity,
    /// not identity operator
    NotIdentity,
    /// assign operator
    Assign,
    /// plus assign operator
    PlusAssign,
    /// minus assign operator
    MinusAssign,
    /// multiply assign operator
    MultiplyAssign,
    /// divide assign operator
    DivideAssign,
    /// modulo assign operator
    ModuloAssign,
    /// bitwise and assign operator
    BitwiseAndAssign,
    /// bitwise or assign operator
    BitwiseOrAssign,
    /// bitwise xor assign operator
    BitwiseXorAssign,
    /// left shift assign operator
    LeftShiftAssign,
    /// right shift assign operator
    RightShiftAssign,
    /// unsigned right shift assign operator
    UnsignedRightShiftAssign,
    /// concatenate assign operator
    ConcatenateAssign,
    /// logical and operator
    LogicalAnd,
    /// logical or operator
    LogicalOr,
    /// increment operator
    Increment,
    /// decrement operator
    Decrement,
    /// not operator
    Not,
    /// question operator
    Question,
    /// dollar operator
    Dollar,
    /// at operator
    At,

    // 分隔符
    /// left parenthesis
    LeftParen,
    /// right parenthesis
    RightParen,
    /// left bracket
    LeftBracket,
    /// right bracket
    RightBracket,
    /// left brace
    LeftBrace,
    /// right brace
    RightBrace,
    /// semicolon
    Semicolon,
    /// comma
    Comma,
    /// dot
    Dot,
    /// dot dot
    DotDot,
    /// dot dot dot
    DotDotDot,
    /// colon
    Colon,
    /// arrow
    Arrow,
    /// hash
    Hash,

    // 字面量
    /// integer literal
    IntegerLiteral,
    /// float literal
    FloatLiteral,
    /// string literal
    StringLiteral,
    /// char literal
    CharLiteral,

    // 标识符和注释
    /// identifier
    Identifier,
    /// line comment
    LineComment,
    /// block comment
    BlockComment,
    /// nested comment
    NestedComment,

    // 空白和特殊
    /// whitespace
    Whitespace,
    /// newline
    Newline,
    /// end of file
    Eof,
    /// error
    Error,
}

impl TokenType for DSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
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
            | Self::DstringType => UniversalTokenRole::Keyword,

            Self::Plus
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
            | Self::Arrow => UniversalTokenRole::Operator,

            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Semicolon | Self::Comma | Self::Dot | Self::DotDot | Self::DotDotDot | Self::Colon | Self::Hash => {
                UniversalTokenRole::Punctuation
            }

            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral => UniversalTokenRole::Literal,

            Self::Identifier => UniversalTokenRole::Name,

            Self::LineComment | Self::BlockComment | Self::NestedComment => UniversalTokenRole::Comment,

            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,

            Self::Error => UniversalTokenRole::Error,

            _ => UniversalTokenRole::None,
        }
    }

    /// Check if the syntax kind is trivia (whitespace or newline)
    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment | Self::NestedComment)
    }

    /// Check if the syntax kind is a comment
    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment | Self::NestedComment)
    }

    /// Check if the syntax kind is whitespace
    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for DSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Module => UniversalElementRole::Container,
            Self::Declaration => UniversalElementRole::Definition,
            Self::Statement => UniversalElementRole::Container,
            Self::Expression => UniversalElementRole::Container,
            Self::Type => UniversalElementRole::Typing,
            _ => UniversalElementRole::Container,
        }
    }
}
