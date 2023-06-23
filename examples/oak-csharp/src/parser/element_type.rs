use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum CSharpElementType {
    // Tokens (copied from CSharpTokenType)
    Whitespace,
    Newline,
    Comment,
    Identifier,
    Number,
    String,
    Character,
    VerbatimString,
    InterpolatedString,
    NumberLiteral,
    StringLiteral,
    CharLiteral,

    // Keywords
    Abstract,
    As,
    Base,
    Bool,
    Break,
    Byte,
    Case,
    Catch,
    Char,
    Checked,
    Class,
    Const,
    Continue,
    Decimal,
    Default,
    Delegate,
    Do,
    Double,
    Else,
    Enum,
    Event,
    Explicit,
    Extern,
    False,
    Finally,
    Fixed,
    Float,
    For,
    Foreach,
    Goto,
    If,
    Implicit,
    In,
    Int,
    Interface,
    Internal,
    Is,
    Lock,
    Long,
    Namespace,
    New,
    Null,
    Object,
    Operator,
    Out,
    Override,
    Params,
    Private,
    Protected,
    Public,
    Readonly,
    Record,
    Ref,
    Return,
    Sbyte,
    Sealed,
    Short,
    Sizeof,
    Stackalloc,
    Static,
    Struct,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Uint,
    Ulong,
    Unchecked,
    Unsafe,
    Ushort,
    Using,
    Virtual,
    Void,
    Volatile,
    While,

    // Long keyword variants
    AbstractKeyword,
    AsKeyword,
    BaseKeyword,
    BoolKeyword,
    BreakKeyword,
    ByteKeyword,
    CaseKeyword,
    CatchKeyword,
    CharKeyword,
    CheckedKeyword,
    ClassKeyword,
    ConstKeyword,
    ContinueKeyword,
    DecimalKeyword,
    DefaultKeyword,
    DelegateKeyword,
    DoKeyword,
    DoubleKeyword,
    ElseKeyword,
    EnumKeyword,
    EventKeyword,
    ExplicitKeyword,
    ExternKeyword,
    FalseKeyword,
    FinallyKeyword,
    FixedKeyword,
    FloatKeyword,
    ForKeyword,
    ForeachKeyword,
    GotoKeyword,
    IfKeyword,
    ImplicitKeyword,
    InKeyword,
    IntKeyword,
    InterfaceKeyword,
    InternalKeyword,
    IsKeyword,
    LockKeyword,
    LongKeyword,
    NamespaceKeyword,
    NewKeyword,
    NullKeyword,
    ObjectKeyword,
    OperatorKeyword,
    OutKeyword,
    OverrideKeyword,
    ParamsKeyword,
    PrivateKeyword,
    ProtectedKeyword,
    PublicKeyword,
    ReadonlyKeyword,
    RefKeyword,
    ReturnKeyword,
    SbyteKeyword,
    SealedKeyword,
    ShortKeyword,
    SizeofKeyword,
    StackallocKeyword,
    StaticKeyword,
    StringKeyword,
    StructKeyword,
    SwitchKeyword,
    ThisKeyword,
    ThrowKeyword,
    TrueKeyword,
    TryKeyword,
    TypeofKeyword,
    UintKeyword,
    UlongKeyword,
    UncheckedKeyword,
    UnsafeKeyword,
    UshortKeyword,
    UsingKeyword,
    VirtualKeyword,
    VoidKeyword,
    VolatileKeyword,
    WhileKeyword,

    // Contextual keywords
    AddKeyword,
    AliasKeyword,
    AscendingKeyword,
    ByKeyword,
    DescendingKeyword,
    FromKeyword,
    GetKeyword,
    GlobalKeyword,
    GroupKeyword,
    IntoKeyword,
    JoinKeyword,
    LetKeyword,
    OrderbyKeyword,
    PartialKeyword,
    RemoveKeyword,
    SelectKeyword,
    SetKeyword,
    ValueKeyword,
    VarKeyword,
    WhereKeyword,
    YieldKeyword,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    LeftShift,
    RightShift,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    Question,
    QuestionQuestion,
    Increment,
    Decrement,
    Arrow,
    Lambda,

    // Assignment operators
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    AmpersandAssign,
    PipeAssign,
    CaretAssign,
    LeftShiftAssign,
    RightShiftAssign,
    QuestionQuestionAssign,
    AndAssign,
    OrAssign,
    XorAssign,

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
    ColonColon,
    Dot,
    QuestionDot,
    At,
    Hash,
    Dollar,

    Eof,
    Error,

    // Non-terminal elements
    IfStatement,
    WhileStatement,
    ForStatement,
    Block,
    ReturnStatement,
    MethodDeclaration,
    PropertyDeclaration,
    FieldDeclaration,
    IndexerDeclaration,
    IdentifierName,
    LiteralExpression,
    BinaryExpression,
    InvocationExpression,
    ElementAccessExpression,
    MemberAccessExpression,
    AssignmentExpression,
    BreakStatement,
    ContinueStatement,
    ExpressionStatement,
    ForeachStatement,
    NamespaceDeclaration,
    UsingDirective,
    ClassDeclaration,
    InterfaceDeclaration,
    StructDeclaration,
    EnumDeclaration,
    RecordDeclaration,
    DelegateDeclaration,
    EventDeclaration,
    AwaitExpression,
    AttributeList,
    Attribute,
    Root,
}

impl CSharpElementType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Abstract
                | Self::As
                | Self::Base
                | Self::Bool
                | Self::Break
                | Self::Byte
                | Self::Case
                | Self::Catch
                | Self::Char
                | Self::Checked
                | Self::Class
                | Self::Const
                | Self::Continue
                | Self::Decimal
                | Self::Default
                | Self::Delegate
                | Self::Do
                | Self::Double
                | Self::Else
                | Self::Enum
                | Self::Event
                | Self::Explicit
                | Self::Extern
                | Self::False
                | Self::Finally
                | Self::Fixed
                | Self::Float
                | Self::For
                | Self::Foreach
                | Self::Goto
                | Self::If
                | Self::Implicit
                | Self::In
                | Self::Int
                | Self::Interface
                | Self::Internal
                | Self::Is
                | Self::Lock
                | Self::Long
                | Self::Namespace
                | Self::New
                | Self::Null
                | Self::Object
                | Self::Operator
                | Self::Out
                | Self::Override
                | Self::Params
                | Self::Private
                | Self::Protected
                | Self::Public
                | Self::Readonly
                | Self::Ref
                | Self::Return
                | Self::Sbyte
                | Self::Sealed
                | Self::Short
                | Self::Sizeof
                | Self::Stackalloc
                | Self::Static
                | Self::Struct
                | Self::Switch
                | Self::This
                | Self::Throw
                | Self::True
                | Self::Try
                | Self::Typeof
                | Self::Uint
                | Self::Ulong
                | Self::Unchecked
                | Self::Unsafe
                | Self::Ushort
                | Self::Using
                | Self::Virtual
                | Self::Void
                | Self::Volatile
                | Self::While
                | Self::AbstractKeyword
                | Self::AsKeyword
                | Self::BaseKeyword
                | Self::BoolKeyword
                | Self::BreakKeyword
                | Self::ByteKeyword
                | Self::CaseKeyword
                | Self::CatchKeyword
                | Self::CharKeyword
                | Self::CheckedKeyword
                | Self::ClassKeyword
                | Self::ConstKeyword
                | Self::ContinueKeyword
                | Self::DecimalKeyword
                | Self::DefaultKeyword
                | Self::DelegateKeyword
                | Self::DoKeyword
                | Self::DoubleKeyword
                | Self::ElseKeyword
                | Self::EnumKeyword
                | Self::EventKeyword
                | Self::ExplicitKeyword
                | Self::ExternKeyword
                | Self::FalseKeyword
                | Self::FinallyKeyword
                | Self::FixedKeyword
                | Self::FloatKeyword
                | Self::ForKeyword
                | Self::ForeachKeyword
                | Self::GotoKeyword
                | Self::IfKeyword
                | Self::ImplicitKeyword
                | Self::InKeyword
                | Self::IntKeyword
                | Self::InterfaceKeyword
                | Self::InternalKeyword
                | Self::IsKeyword
                | Self::LockKeyword
                | Self::LongKeyword
                | Self::NamespaceKeyword
                | Self::NewKeyword
                | Self::NullKeyword
                | Self::ObjectKeyword
                | Self::OperatorKeyword
                | Self::OutKeyword
                | Self::OverrideKeyword
                | Self::ParamsKeyword
                | Self::PrivateKeyword
                | Self::ProtectedKeyword
                | Self::PublicKeyword
                | Self::ReadonlyKeyword
                | Self::RefKeyword
                | Self::ReturnKeyword
                | Self::SbyteKeyword
                | Self::SealedKeyword
                | Self::ShortKeyword
                | Self::SizeofKeyword
                | Self::StackallocKeyword
                | Self::StaticKeyword
                | Self::StringKeyword
                | Self::StructKeyword
                | Self::SwitchKeyword
                | Self::ThisKeyword
                | Self::ThrowKeyword
                | Self::TrueKeyword
                | Self::TryKeyword
                | Self::TypeofKeyword
                | Self::UintKeyword
                | Self::UlongKeyword
                | Self::UncheckedKeyword
                | Self::UnsafeKeyword
                | Self::UshortKeyword
                | Self::UsingKeyword
                | Self::VirtualKeyword
                | Self::VoidKeyword
                | Self::VolatileKeyword
                | Self::WhileKeyword
                | Self::AddKeyword
                | Self::AliasKeyword
                | Self::AscendingKeyword
                | Self::ByKeyword
                | Self::DescendingKeyword
                | Self::FromKeyword
                | Self::GetKeyword
                | Self::GlobalKeyword
                | Self::GroupKeyword
                | Self::IntoKeyword
                | Self::JoinKeyword
                | Self::LetKeyword
                | Self::OrderbyKeyword
                | Self::PartialKeyword
                | Self::RemoveKeyword
                | Self::SelectKeyword
                | Self::SetKeyword
                | Self::ValueKeyword
                | Self::VarKeyword
                | Self::WhereKeyword
                | Self::YieldKeyword
        )
    }
}

impl ElementType for CSharpElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::CSharpTokenType> for CSharpElementType {
    fn from(token: crate::lexer::token_type::CSharpTokenType) -> Self {
        unsafe { std::mem::transmute(token as u16) }
    }
}
