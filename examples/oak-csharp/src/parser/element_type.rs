use crate::lexer::CSharpTokenType;
use oak_core::{ElementType, UniversalElementRole};
use serde::{Deserialize, Serialize};

/// C# element type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CSharpElementType {
    /// Root node of the syntax tree
    Root,
    /// Compilation unit (source file)
    CompilationUnit,
    /// Namespace declaration
    NamespaceDeclaration,
    /// Using directive
    UsingDirective,
    /// Class declaration
    ClassDeclaration,
    /// Struct declaration
    StructDeclaration,
    /// Interface declaration
    InterfaceDeclaration,
    /// Enum declaration
    EnumDeclaration,
    /// Delegate declaration
    DelegateDeclaration,
    /// Method declaration
    MethodDeclaration,
    /// Property declaration
    PropertyDeclaration,
    /// Field declaration
    FieldDeclaration,
    /// Event declaration
    EventDeclaration,
    /// Indexer declaration
    IndexerDeclaration,
    /// Constructor declaration
    ConstructorDeclaration,
    /// Destructor declaration
    DestructorDeclaration,
    /// Operator declaration
    OperatorDeclaration,
    /// Conversion operator declaration
    ConversionOperatorDeclaration,
    /// Parameter
    Parameter,
    /// Type parameter
    TypeParameter,
    /// Constraint
    Constraint,
    /// Attribute
    Attribute,
    /// Attribute list
    AttributeList,
    /// Block statement
    Block,
    /// Expression statement
    ExpressionStatement,
    /// If statement
    IfStatement,
    /// Switch statement
    SwitchStatement,
    /// While statement
    WhileStatement,
    /// For statement
    ForStatement,
    /// Foreach statement
    ForeachStatement,
    /// Do-while statement
    DoStatement,
    /// Try statement
    TryStatement,
    /// Catch clause
    CatchClause,
    /// Finally clause
    FinallyClause,
    /// Throw statement
    ThrowStatement,
    /// Return statement
    ReturnStatement,
    /// Break statement
    BreakStatement,
    /// Continue statement
    ContinueStatement,
    /// Goto statement
    GotoStatement,
    /// Labeled statement
    LabeledStatement,
    /// Lock statement
    LockStatement,
    /// Using statement
    UsingStatement,
    /// Fixed statement
    FixedStatement,
    /// Unsafe statement
    UnsafeStatement,
    /// Checked statement
    CheckedStatement,
    /// Unchecked statement
    UncheckedStatement,
    /// Yield statement
    YieldStatement,
    /// Local declaration statement
    LocalDeclarationStatement,
    /// Binary expression
    BinaryExpression,
    /// Unary expression
    UnaryExpression,
    /// Assignment expression
    AssignmentExpression,
    /// Conditional expression (ternary)
    ConditionalExpression,
    /// Method invocation expression
    InvocationExpression,
    /// Member access expression
    MemberAccessExpression,
    /// Element access expression
    ElementAccessExpression,
    /// Cast expression
    CastExpression,
    /// As expression
    AsExpression,
    /// Is expression
    IsExpression,
    /// Typeof expression
    TypeOfExpression,
    /// Sizeof expression
    SizeOfExpression,
    /// Default value expression
    DefaultExpression,
    /// Literal expression
    LiteralExpression,
    /// This expression
    ThisExpression,
    /// Base expression
    BaseExpression,
    /// Identifier name
    IdentifierName,
    /// Qualified name
    QualifiedName,
    /// Generic name
    GenericName,
    /// Alias qualified name
    AliasQualifiedName,
    /// Predefined type
    PredefinedType,
    /// Array type
    ArrayType,
    /// Pointer type
    PointerType,
    /// Nullable type
    NullableType,
    /// Tuple type
    TupleType,
    /// Ref type
    RefType,
    /// Array creation expression
    ArrayCreationExpression,
    /// Implicit array creation expression
    ImplicitArrayCreationExpression,
    /// Stack alloc array creation expression
    StackAllocArrayCreationExpression,
    /// Object creation expression
    ObjectCreationExpression,
    /// Anonymous object creation expression
    AnonymousObjectCreationExpression,
    /// Array initializer expression
    ArrayInitializerExpression,
    /// Collection initializer expression
    CollectionInitializerExpression,
    /// Complex element initializer expression
    ComplexElementInitializerExpression,
    /// Object initializer expression
    ObjectInitializerExpression,
    /// Member initializer expression
    MemberInitializerExpression,
    /// Lambda expression
    LambdaExpression,
    /// Anonymous method expression
    AnonymousMethodExpression,
    /// Query expression
    QueryExpression,
    /// Query body
    QueryBody,
    /// From clause
    FromClause,
    /// Let clause
    LetClause,
    /// Where clause
    WhereClause,
    /// Join clause
    JoinClause,
    /// Join into clause
    JoinIntoClause,
    /// Order by clause
    OrderByClause,
    /// Ordering
    Ordering,
    /// Select clause
    SelectClause,
    /// Group clause
    GroupClause,
    /// Query continuation
    QueryContinuation,
    /// Omitted array size expression
    OmittedArraySizeExpression,
    /// Interpolated string expression
    InterpolatedStringExpression,
    /// Interpolated string text
    InterpolatedStringText,
    /// Interpolation
    Interpolation,
    /// Interpolation alignment clause
    InterpolationAlignmentClause,
    /// Interpolation format clause
    InterpolationFormatClause,
    /// Global statement
    GlobalStatement,
    /// Simple lambda expression
    SimpleLambdaExpression,
    /// Parenthesized lambda expression
    ParenthesizedLambdaExpression,
    /// Initializer expression
    InitializerExpression,
    /// Implicit element access
    ImplicitElementAccess,
    /// Postfix unary expression
    PostfixUnaryExpression,
    /// Prefix unary expression
    PrefixUnaryExpression,
    /// Await expression
    AwaitExpression,
    /// Name colon
    NameColon,
    /// Declaration expression
    DeclarationExpression,
    /// Tuple expression
    TupleExpression,
    /// Tuple element
    TupleElement,
    /// Single variable designation
    SingleVariableDesignation,
    /// Parenthesized variable designation
    ParenthesizedVariableDesignation,
    /// Discard designation
    DiscardDesignation,
    /// Ref expression
    RefExpression,
    /// Ref type expression
    RefTypeExpression,
    /// Ref value expression
    RefValueExpression,
    /// Make ref expression
    MakeRefExpression,
    /// Checked expression
    CheckedExpression,
    /// Unchecked expression
    UncheckedExpression,
    /// Default literal expression
    DefaultLiteralExpression,
    /// Conditional access expression
    ConditionalAccessExpression,
    /// Member binding expression
    MemberBindingExpression,
    /// Element binding expression
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
    /// Switch expression
    SwitchExpression,
    /// Switch expression arm
    SwitchExpressionArm,
    /// Case pattern switch label
    CasePatternSwitchLabel,
    /// Case switch label
    CaseSwitchLabel,
    /// Discard pattern
    DiscardPattern,
    /// Tuple pattern
    TuplePattern,
    /// Parenthesized pattern
    ParenthesizedPattern,
    /// Relational pattern
    RelationalPattern,
    /// Type pattern
    TypePattern,
    /// Binary pattern
    BinaryPattern,
    /// Unary pattern
    UnaryPattern,
    /// Slice pattern
    SlicePattern,
    /// Range expression
    RangeExpression,
    /// Index expression
    IndexExpression,
    /// With expression
    WithExpression,
    /// Anonymous object member declarator
    AnonymousObjectMemberDeclarator,
    /// Argument list
    ArgumentList,
    /// Bracketed argument list
    BracketedArgumentList,
    /// Argument
    Argument,
    /// Name equals
    NameEquals,
    /// Type argument list
    TypeArgumentList,
    /// Type parameter list
    TypeParameterList,
    /// Type parameter constraint clause
    TypeParameterConstraintClause,
    /// Constructor constraint
    ConstructorConstraint,
    /// Class or struct constraint
    ClassOrStructConstraint,
    /// Type constraint
    TypeConstraint,
    /// Base list
    BaseList,
    /// Simple base type
    SimpleBaseType,
    /// Primary constructor base type
    PrimaryConstructorBaseType,
    /// Accessor list
    AccessorList,
    /// Accessor declaration
    AccessorDeclaration,
    /// Parameter list
    ParameterList,
    /// Bracketed parameter list
    BracketedParameterList,
    /// Arrow expression clause
    ArrowExpressionClause,
    /// Equals value clause
    EqualsValueClause,
    /// Variable declaration
    VariableDeclaration,
    /// Variable declarator
    VariableDeclarator,
    /// Separated syntax list
    SeparatedSyntaxList,
    /// Syntax list
    SyntaxList,
}

impl ElementType for CSharpElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        UniversalElementRole::None
    }
}

impl From<CSharpTokenType> for CSharpElementType {
    fn from(token: CSharpTokenType) -> Self {
        match token {
            CSharpTokenType::Eof => Self::SyntaxList, // Default or Error?
            _ => Self::SyntaxList,
        }
    }
}
