use oak_core::SyntaxKind;

/// C# 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CSharpSyntaxKind {
    // 基础 kind
    Whitespace,
    Comment,
    Identifier,

    // 字面量
    Number,
    String,
    Character,
    VerbatimString,
    InterpolatedString,
    NumberLiteral,
    StringLiteral,
    CharLiteral,

    // 关键字（简化版本）
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

    // 关键字（带 Keyword 后缀）
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

    // 上下文关键字
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

    // 运算符
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

    // 赋值运算符
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

    // 分隔符
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

    // 特殊
    Newline,
    Eof,
    Error,

    // 语法节点
    Root,
    CompilationUnit,
    NamespaceDeclaration,
    UsingDirective,
    ClassDeclaration,
    StructDeclaration,
    InterfaceDeclaration,
    EnumDeclaration,
    DelegateDeclaration,
    MethodDeclaration,
    PropertyDeclaration,
    FieldDeclaration,
    EventDeclaration,
    IndexerDeclaration,
    ConstructorDeclaration,
    DestructorDeclaration,
    OperatorDeclaration,
    ConversionOperatorDeclaration,
    Parameter,
    TypeParameter,
    Constraint,
    Attribute,
    AttributeList,
    Block,
    ExpressionStatement,
    IfStatement,
    SwitchStatement,
    WhileStatement,
    ForStatement,
    ForeachStatement,
    DoStatement,
    TryStatement,
    CatchClause,
    FinallyClause,
    ThrowStatement,
    ReturnStatement,
    BreakStatement,
    ContinueStatement,
    GotoStatement,
    LabeledStatement,
    LockStatement,
    UsingStatement,
    FixedStatement,
    UnsafeStatement,
    CheckedStatement,
    UncheckedStatement,
    YieldStatement,
    LocalDeclarationStatement,
    BinaryExpression,
    UnaryExpression,
    AssignmentExpression,
    ConditionalExpression,
    InvocationExpression,
    MemberAccessExpression,
    ElementAccessExpression,
    CastExpression,
    AsExpression,
    IsExpression,
    TypeOfExpression,
    SizeOfExpression,
    DefaultExpression,
    LiteralExpression,
    ThisExpression,
    BaseExpression,
    IdentifierName,
    QualifiedName,
    GenericName,
    AliasQualifiedName,
    PredefinedType,
    ArrayType,
    PointerType,
    NullableType,
    TupleType,
    RefType,
    ArrayCreationExpression,
    ImplicitArrayCreationExpression,
    StackAllocArrayCreationExpression,
    ObjectCreationExpression,
    AnonymousObjectCreationExpression,
    ArrayInitializerExpression,
    CollectionInitializerExpression,
    ComplexElementInitializerExpression,
    ObjectInitializerExpression,
    MemberInitializerExpression,
    LambdaExpression,
    AnonymousMethodExpression,
    QueryExpression,
    QueryBody,
    FromClause,
    LetClause,
    WhereClause,
    JoinClause,
    JoinIntoClause,
    OrderByClause,
    Ordering,
    SelectClause,
    GroupClause,
    QueryContinuation,
    OmittedArraySizeExpression,
    InterpolatedStringExpression,
    InterpolatedStringText,
    Interpolation,
    InterpolationAlignmentClause,
    InterpolationFormatClause,
    GlobalStatement,
    SimpleLambdaExpression,
    ParenthesizedLambdaExpression,
    InitializerExpression,
    ImplicitElementAccess,
    PostfixUnaryExpression,
    PrefixUnaryExpression,
    AwaitExpression,
    NameColon,
    DeclarationExpression,
    TupleExpression,
    TupleElement,
    SingleVariableDesignation,
    ParenthesizedVariableDesignation,
    DiscardDesignation,
    RefExpression,
    RefTypeExpression,
    RefValueExpression,
    MakeRefExpression,
    CheckedExpression,
    UncheckedExpression,
    DefaultLiteralExpression,
    ConditionalAccessExpression,
    MemberBindingExpression,
    ElementBindingExpression,
    ImplicitStackAllocArrayCreationExpression,
    IsPatternExpression,
    ThrowExpression,
    WhenClause,
    ConstantPattern,
    DeclarationPattern,
    VarPattern,
    RecursivePattern,
    PositionalPatternClause,
    PropertyPatternClause,
    Subpattern,
    SwitchExpression,
    SwitchExpressionArm,
    DiscardPattern,
    TuplePattern,
    ParenthesizedPattern,
    RelationalPattern,
    TypePattern,
    BinaryPattern,
    UnaryPattern,
    ListPattern,
    SlicePattern,
    RangeExpression,
    IndexExpression,
    WithExpression,
    AnonymousObjectMemberDeclarator,
    ArgumentList,
    BracketedArgumentList,
    Argument,
    NameEquals,
    TypeArgumentList,
    TypeParameterList,
    TypeParameterConstraintClause,
    ConstructorConstraint,
    ClassOrStructConstraint,
    TypeConstraint,
    BaseList,
    SimpleBaseType,
    PrimaryConstructorBaseType,
    AccessorList,
    AccessorDeclaration,
    ParameterList,
    BracketedParameterList,
    ArrowExpressionClause,
    EqualsValueClause,
    VariableDeclaration,
    VariableDeclarator,
    SeparatedSyntaxList,
    SyntaxList,
}

impl SyntaxKind for CSharpSyntaxKind {
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
        !matches!(
            self,
            Self::Root
                | Self::CompilationUnit
                | Self::NamespaceDeclaration
                | Self::ClassDeclaration
                | Self::StructDeclaration
                | Self::InterfaceDeclaration
                | Self::EnumDeclaration
                | Self::DelegateDeclaration
                | Self::MethodDeclaration
                | Self::PropertyDeclaration
                | Self::FieldDeclaration
                | Self::EventDeclaration
                | Self::IndexerDeclaration
                | Self::ConstructorDeclaration
                | Self::DestructorDeclaration
                | Self::OperatorDeclaration
                | Self::ConversionOperatorDeclaration
                | Self::Parameter
                | Self::TypeParameter
                | Self::Constraint
                | Self::Attribute
                | Self::AttributeList
                | Self::Block
                | Self::ExpressionStatement
                | Self::IfStatement
                | Self::SwitchStatement
                | Self::WhileStatement
                | Self::ForStatement
                | Self::ForeachStatement
                | Self::DoStatement
                | Self::TryStatement
                | Self::CatchClause
                | Self::FinallyClause
                | Self::ThrowStatement
                | Self::ReturnStatement
                | Self::BreakStatement
                | Self::ContinueStatement
                | Self::GotoStatement
                | Self::LabeledStatement
                | Self::LockStatement
                | Self::UsingStatement
                | Self::FixedStatement
                | Self::UnsafeStatement
                | Self::CheckedStatement
                | Self::UncheckedStatement
                | Self::YieldStatement
                | Self::LocalDeclarationStatement
                | Self::BinaryExpression
                | Self::UnaryExpression
                | Self::AssignmentExpression
                | Self::ConditionalExpression
                | Self::InvocationExpression
                | Self::MemberAccessExpression
                | Self::ElementAccessExpression
                | Self::CastExpression
                | Self::AsExpression
                | Self::IsExpression
                | Self::TypeOfExpression
                | Self::SizeOfExpression
                | Self::DefaultExpression
                | Self::LiteralExpression
                | Self::ThisExpression
                | Self::BaseExpression
                | Self::IdentifierName
                | Self::QualifiedName
                | Self::GenericName
                | Self::AliasQualifiedName
                | Self::PredefinedType
                | Self::ArrayType
                | Self::PointerType
                | Self::NullableType
                | Self::TupleType
                | Self::RefType
                | Self::ArrayCreationExpression
                | Self::ImplicitArrayCreationExpression
                | Self::StackAllocArrayCreationExpression
                | Self::ObjectCreationExpression
                | Self::AnonymousObjectCreationExpression
                | Self::ArrayInitializerExpression
                | Self::CollectionInitializerExpression
                | Self::ComplexElementInitializerExpression
                | Self::ObjectInitializerExpression
                | Self::MemberInitializerExpression
                | Self::LambdaExpression
                | Self::AnonymousMethodExpression
                | Self::QueryExpression
                | Self::QueryBody
                | Self::FromClause
                | Self::LetClause
                | Self::WhereClause
                | Self::JoinClause
                | Self::JoinIntoClause
                | Self::OrderByClause
                | Self::Ordering
                | Self::SelectClause
                | Self::GroupClause
                | Self::QueryContinuation
                | Self::OmittedArraySizeExpression
                | Self::InterpolatedStringExpression
                | Self::InterpolatedStringText
                | Self::Interpolation
                | Self::InterpolationAlignmentClause
                | Self::InterpolationFormatClause
                | Self::GlobalStatement
                | Self::SimpleLambdaExpression
                | Self::ParenthesizedLambdaExpression
                | Self::InitializerExpression
                | Self::ImplicitElementAccess
                | Self::PostfixUnaryExpression
                | Self::PrefixUnaryExpression
                | Self::AwaitExpression
                | Self::NameColon
                | Self::DeclarationExpression
                | Self::TupleExpression
                | Self::TupleElement
                | Self::SingleVariableDesignation
                | Self::ParenthesizedVariableDesignation
                | Self::DiscardDesignation
                | Self::RefExpression
                | Self::RefTypeExpression
                | Self::RefValueExpression
                | Self::MakeRefExpression
                | Self::CheckedExpression
                | Self::UncheckedExpression
                | Self::DefaultLiteralExpression
                | Self::ConditionalAccessExpression
                | Self::MemberBindingExpression
                | Self::ElementBindingExpression
                | Self::ImplicitStackAllocArrayCreationExpression
                | Self::IsPatternExpression
                | Self::ThrowExpression
                | Self::WhenClause
                | Self::ConstantPattern
                | Self::DeclarationPattern
                | Self::VarPattern
                | Self::RecursivePattern
                | Self::PositionalPatternClause
                | Self::PropertyPatternClause
                | Self::Subpattern
                | Self::SwitchExpression
                | Self::SwitchExpressionArm
                | Self::DiscardPattern
                | Self::TuplePattern
                | Self::ParenthesizedPattern
                | Self::RelationalPattern
                | Self::TypePattern
                | Self::BinaryPattern
                | Self::UnaryPattern
                | Self::ListPattern
                | Self::SlicePattern
                | Self::RangeExpression
                | Self::IndexExpression
                | Self::WithExpression
                | Self::AnonymousObjectMemberDeclarator
                | Self::ArgumentList
                | Self::BracketedArgumentList
                | Self::Argument
                | Self::NameEquals
                | Self::TypeArgumentList
                | Self::TypeParameterList
                | Self::TypeParameterConstraintClause
                | Self::ConstructorConstraint
                | Self::ClassOrStructConstraint
                | Self::TypeConstraint
                | Self::BaseList
                | Self::SimpleBaseType
                | Self::PrimaryConstructorBaseType
                | Self::AccessorList
                | Self::AccessorDeclaration
                | Self::ParameterList
                | Self::BracketedParameterList
                | Self::ArrowExpressionClause
                | Self::EqualsValueClause
                | Self::VariableDeclaration
                | Self::VariableDeclarator
                | Self::SeparatedSyntaxList
                | Self::SyntaxList
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            Self::Root
                | Self::CompilationUnit
                | Self::NamespaceDeclaration
                | Self::ClassDeclaration
                | Self::StructDeclaration
                | Self::InterfaceDeclaration
                | Self::EnumDeclaration
                | Self::DelegateDeclaration
                | Self::MethodDeclaration
                | Self::PropertyDeclaration
                | Self::FieldDeclaration
                | Self::EventDeclaration
                | Self::IndexerDeclaration
                | Self::ConstructorDeclaration
                | Self::DestructorDeclaration
                | Self::OperatorDeclaration
                | Self::ConversionOperatorDeclaration
                | Self::Parameter
                | Self::TypeParameter
                | Self::Constraint
                | Self::Attribute
                | Self::AttributeList
                | Self::Block
                | Self::ExpressionStatement
                | Self::IfStatement
                | Self::SwitchStatement
                | Self::WhileStatement
                | Self::ForStatement
                | Self::ForeachStatement
                | Self::DoStatement
                | Self::TryStatement
                | Self::CatchClause
                | Self::FinallyClause
                | Self::ThrowStatement
                | Self::ReturnStatement
                | Self::BreakStatement
                | Self::ContinueStatement
                | Self::GotoStatement
                | Self::LabeledStatement
                | Self::LockStatement
                | Self::UsingStatement
                | Self::FixedStatement
                | Self::UnsafeStatement
                | Self::CheckedStatement
                | Self::UncheckedStatement
                | Self::YieldStatement
                | Self::LocalDeclarationStatement
                | Self::BinaryExpression
                | Self::UnaryExpression
                | Self::AssignmentExpression
                | Self::ConditionalExpression
                | Self::InvocationExpression
                | Self::MemberAccessExpression
                | Self::ElementAccessExpression
                | Self::CastExpression
                | Self::AsExpression
                | Self::IsExpression
                | Self::TypeOfExpression
                | Self::SizeOfExpression
                | Self::DefaultExpression
                | Self::LiteralExpression
                | Self::ThisExpression
                | Self::BaseExpression
                | Self::IdentifierName
                | Self::QualifiedName
                | Self::GenericName
                | Self::AliasQualifiedName
                | Self::PredefinedType
                | Self::ArrayType
                | Self::PointerType
                | Self::NullableType
                | Self::TupleType
                | Self::RefType
                | Self::ArrayCreationExpression
                | Self::ImplicitArrayCreationExpression
                | Self::StackAllocArrayCreationExpression
                | Self::ObjectCreationExpression
                | Self::AnonymousObjectCreationExpression
                | Self::ArrayInitializerExpression
                | Self::CollectionInitializerExpression
                | Self::ComplexElementInitializerExpression
                | Self::ObjectInitializerExpression
                | Self::MemberInitializerExpression
                | Self::LambdaExpression
                | Self::AnonymousMethodExpression
                | Self::QueryExpression
                | Self::QueryBody
                | Self::FromClause
                | Self::LetClause
                | Self::WhereClause
                | Self::JoinClause
                | Self::JoinIntoClause
                | Self::OrderByClause
                | Self::Ordering
                | Self::SelectClause
                | Self::GroupClause
                | Self::QueryContinuation
                | Self::OmittedArraySizeExpression
                | Self::InterpolatedStringExpression
                | Self::InterpolatedStringText
                | Self::Interpolation
                | Self::InterpolationAlignmentClause
                | Self::InterpolationFormatClause
                | Self::GlobalStatement
                | Self::SimpleLambdaExpression
                | Self::ParenthesizedLambdaExpression
                | Self::InitializerExpression
                | Self::ImplicitElementAccess
                | Self::PostfixUnaryExpression
                | Self::PrefixUnaryExpression
                | Self::AwaitExpression
                | Self::NameColon
                | Self::DeclarationExpression
                | Self::TupleExpression
                | Self::TupleElement
                | Self::SingleVariableDesignation
                | Self::ParenthesizedVariableDesignation
                | Self::DiscardDesignation
                | Self::RefExpression
                | Self::RefTypeExpression
                | Self::RefValueExpression
                | Self::MakeRefExpression
                | Self::CheckedExpression
                | Self::UncheckedExpression
                | Self::DefaultLiteralExpression
                | Self::ConditionalAccessExpression
                | Self::MemberBindingExpression
                | Self::ElementBindingExpression
                | Self::ImplicitStackAllocArrayCreationExpression
                | Self::IsPatternExpression
                | Self::ThrowExpression
                | Self::WhenClause
                | Self::ConstantPattern
                | Self::DeclarationPattern
                | Self::VarPattern
                | Self::RecursivePattern
                | Self::PositionalPatternClause
                | Self::PropertyPatternClause
                | Self::Subpattern
                | Self::SwitchExpression
                | Self::SwitchExpressionArm
                | Self::DiscardPattern
                | Self::TuplePattern
                | Self::ParenthesizedPattern
                | Self::RelationalPattern
                | Self::TypePattern
                | Self::BinaryPattern
                | Self::UnaryPattern
                | Self::ListPattern
                | Self::SlicePattern
                | Self::RangeExpression
                | Self::IndexExpression
                | Self::WithExpression
                | Self::AnonymousObjectMemberDeclarator
                | Self::ArgumentList
                | Self::BracketedArgumentList
                | Self::Argument
                | Self::NameEquals
                | Self::TypeArgumentList
                | Self::TypeParameterList
                | Self::TypeParameterConstraintClause
                | Self::ConstructorConstraint
                | Self::ClassOrStructConstraint
                | Self::TypeConstraint
                | Self::BaseList
                | Self::SimpleBaseType
                | Self::PrimaryConstructorBaseType
                | Self::AccessorList
                | Self::AccessorDeclaration
                | Self::ParameterList
                | Self::BracketedParameterList
                | Self::ArrowExpressionClause
                | Self::EqualsValueClause
                | Self::VariableDeclaration
                | Self::VariableDeclarator
                | Self::SeparatedSyntaxList
                | Self::SyntaxList
        )
    }
}
