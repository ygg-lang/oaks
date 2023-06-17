use oak_core::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeScriptSyntaxKind {
    // 节点种类
    Root,
    SourceFile,
    Module,

    // 声明
    VariableDeclaration,
    FunctionDeclaration,
    ClassDeclaration,
    InterfaceDeclaration,
    TypeAliasDeclaration,
    EnumDeclaration,
    NamespaceDeclaration,
    ImportDeclaration,
    ExportDeclaration,

    // 表达式节点
    BinaryExpression,
    UnaryExpression,
    ConditionalExpression,
    CallExpression,
    NewExpression,
    MemberExpression,
    ArrayExpression,
    ObjectExpression,
    FunctionExpression,
    ArrowFunction,
    TemplateExpression,
    TaggedTemplateExpression,
    AsExpression,
    TypeAssertionExpression,
    NonNullExpression,

    // 语句
    ExpressionStatement,
    BlockStatement,
    IfStatement,
    WhileStatement,
    ForStatement,
    ForInStatement,
    ForOfStatement,
    DoWhileStatement,
    SwitchStatement,
    CaseClause,
    DefaultClause,
    TryStatement,
    CatchClause,
    FinallyClause,
    ThrowStatement,
    ReturnStatement,
    BreakStatement,
    ContinueStatement,
    DebuggerStatement,
    WithStatement,

    // 模式
    BindingPattern,
    ArrayBindingPattern,
    ObjectBindingPattern,
    BindingElement,

    // 类型
    TypeReference,
    TypeLiteral,
    FunctionType,
    ConstructorType,
    ArrayType,
    TupleType,
    UnionType,
    IntersectionType,
    ConditionalType,
    MappedType,
    IndexedAccessType,
    TypeQuery,
    TypePredicate,

    // 错误节点
    Error,

    // 关键字
    Abstract,
    Any,
    As,
    Asserts,
    Async,
    Await,
    Boolean,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Constructor,
    Continue,
    Debugger,
    Declare,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    False,
    Finally,
    For,
    From,
    Function,
    Get,
    Global,
    If,
    Implements,
    Import,
    In,
    Infer,
    Instanceof,
    Interface,
    Is,
    Keyof,
    Let,
    Namespace,
    Never,
    New,
    Null,
    Number,
    Object,
    Of,
    Package,
    Private,
    Protected,
    Public,
    Readonly,
    Require,
    Return,
    Set,
    Static,
    String,
    Super,
    Switch,
    Symbol,
    This,
    Throw,
    True,
    Try,
    Type,
    Typeof,
    Undefined,
    Unique,
    Unknown,
    Var,
    Void,
    While,
    With,
    Yield,

    // 操作符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,

    // 比较操作符
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    EqualEqual,
    NotEqual,
    EqualEqualEqual,
    NotEqualEqual,

    // 逻辑操作符
    AmpersandAmpersand,
    PipePipe,
    Exclamation,

    // 位操作符
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    LeftShift,
    RightShift,
    UnsignedRightShift,

    // 赋值操作符
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    StarStarEqual,
    LeftShiftEqual,
    RightShiftEqual,
    UnsignedRightShiftEqual,
    AmpersandEqual,
    PipeEqual,
    CaretEqual,
    AmpersandAmpersandEqual,
    PipePipeEqual,
    QuestionQuestionEqual,

    // 一元操作符
    PlusPlus,
    MinusMinus,

    // 其他操作符
    Question,
    QuestionQuestion,
    QuestionDot,
    Arrow,

    // 标点符号
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    DotDotDot,
    Colon,

    // 字面量
    StringLiteral,
    NumericLiteral,
    BigIntLiteral,
    TemplateString,
    RegexLiteral,

    // 标识符
    IdentifierName,

    // 注释和空白
    LineComment,
    BlockComment,
    Whitespace,
    Newline,

    // 特殊符号
    Eof,
}

impl SyntaxKind for TypeScriptSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(
            self,
            TypeScriptSyntaxKind::LineComment
                | TypeScriptSyntaxKind::BlockComment
                | TypeScriptSyntaxKind::Whitespace
                | TypeScriptSyntaxKind::Newline
        )
    }

    fn is_comment(&self) -> bool {
        matches!(self, TypeScriptSyntaxKind::LineComment | TypeScriptSyntaxKind::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, TypeScriptSyntaxKind::Whitespace | TypeScriptSyntaxKind::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(
            self,
            TypeScriptSyntaxKind::Root
                | TypeScriptSyntaxKind::SourceFile
                | TypeScriptSyntaxKind::Module
                | TypeScriptSyntaxKind::VariableDeclaration
                | TypeScriptSyntaxKind::FunctionDeclaration
                | TypeScriptSyntaxKind::ClassDeclaration
                | TypeScriptSyntaxKind::InterfaceDeclaration
                | TypeScriptSyntaxKind::TypeAliasDeclaration
                | TypeScriptSyntaxKind::EnumDeclaration
                | TypeScriptSyntaxKind::NamespaceDeclaration
                | TypeScriptSyntaxKind::ImportDeclaration
                | TypeScriptSyntaxKind::ExportDeclaration
                | TypeScriptSyntaxKind::BinaryExpression
                | TypeScriptSyntaxKind::UnaryExpression
                | TypeScriptSyntaxKind::ConditionalExpression
                | TypeScriptSyntaxKind::CallExpression
                | TypeScriptSyntaxKind::NewExpression
                | TypeScriptSyntaxKind::MemberExpression
                | TypeScriptSyntaxKind::ArrayExpression
                | TypeScriptSyntaxKind::ObjectExpression
                | TypeScriptSyntaxKind::FunctionExpression
                | TypeScriptSyntaxKind::ArrowFunction
                | TypeScriptSyntaxKind::TemplateExpression
                | TypeScriptSyntaxKind::TaggedTemplateExpression
                | TypeScriptSyntaxKind::AsExpression
                | TypeScriptSyntaxKind::TypeAssertionExpression
                | TypeScriptSyntaxKind::NonNullExpression
                | TypeScriptSyntaxKind::ExpressionStatement
                | TypeScriptSyntaxKind::BlockStatement
                | TypeScriptSyntaxKind::IfStatement
                | TypeScriptSyntaxKind::WhileStatement
                | TypeScriptSyntaxKind::ForStatement
                | TypeScriptSyntaxKind::ForInStatement
                | TypeScriptSyntaxKind::ForOfStatement
                | TypeScriptSyntaxKind::DoWhileStatement
                | TypeScriptSyntaxKind::SwitchStatement
                | TypeScriptSyntaxKind::CaseClause
                | TypeScriptSyntaxKind::DefaultClause
                | TypeScriptSyntaxKind::TryStatement
                | TypeScriptSyntaxKind::CatchClause
                | TypeScriptSyntaxKind::FinallyClause
                | TypeScriptSyntaxKind::ThrowStatement
                | TypeScriptSyntaxKind::ReturnStatement
                | TypeScriptSyntaxKind::BreakStatement
                | TypeScriptSyntaxKind::ContinueStatement
                | TypeScriptSyntaxKind::DebuggerStatement
                | TypeScriptSyntaxKind::WithStatement
                | TypeScriptSyntaxKind::BindingPattern
                | TypeScriptSyntaxKind::ArrayBindingPattern
                | TypeScriptSyntaxKind::ObjectBindingPattern
                | TypeScriptSyntaxKind::BindingElement
                | TypeScriptSyntaxKind::TypeReference
                | TypeScriptSyntaxKind::TypeLiteral
                | TypeScriptSyntaxKind::FunctionType
                | TypeScriptSyntaxKind::ConstructorType
                | TypeScriptSyntaxKind::ArrayType
                | TypeScriptSyntaxKind::TupleType
                | TypeScriptSyntaxKind::UnionType
                | TypeScriptSyntaxKind::IntersectionType
                | TypeScriptSyntaxKind::ConditionalType
                | TypeScriptSyntaxKind::MappedType
                | TypeScriptSyntaxKind::IndexedAccessType
                | TypeScriptSyntaxKind::TypeQuery
                | TypeScriptSyntaxKind::TypePredicate
                | TypeScriptSyntaxKind::Error
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            TypeScriptSyntaxKind::Root
                | TypeScriptSyntaxKind::SourceFile
                | TypeScriptSyntaxKind::Module
                | TypeScriptSyntaxKind::VariableDeclaration
                | TypeScriptSyntaxKind::FunctionDeclaration
                | TypeScriptSyntaxKind::ClassDeclaration
                | TypeScriptSyntaxKind::InterfaceDeclaration
                | TypeScriptSyntaxKind::TypeAliasDeclaration
                | TypeScriptSyntaxKind::EnumDeclaration
                | TypeScriptSyntaxKind::NamespaceDeclaration
                | TypeScriptSyntaxKind::ImportDeclaration
                | TypeScriptSyntaxKind::ExportDeclaration
                | TypeScriptSyntaxKind::BinaryExpression
                | TypeScriptSyntaxKind::UnaryExpression
                | TypeScriptSyntaxKind::ConditionalExpression
                | TypeScriptSyntaxKind::CallExpression
                | TypeScriptSyntaxKind::NewExpression
                | TypeScriptSyntaxKind::MemberExpression
                | TypeScriptSyntaxKind::ArrayExpression
                | TypeScriptSyntaxKind::ObjectExpression
                | TypeScriptSyntaxKind::FunctionExpression
                | TypeScriptSyntaxKind::ArrowFunction
                | TypeScriptSyntaxKind::TemplateExpression
                | TypeScriptSyntaxKind::TaggedTemplateExpression
                | TypeScriptSyntaxKind::AsExpression
                | TypeScriptSyntaxKind::TypeAssertionExpression
                | TypeScriptSyntaxKind::NonNullExpression
                | TypeScriptSyntaxKind::ExpressionStatement
                | TypeScriptSyntaxKind::BlockStatement
                | TypeScriptSyntaxKind::IfStatement
                | TypeScriptSyntaxKind::WhileStatement
                | TypeScriptSyntaxKind::ForStatement
                | TypeScriptSyntaxKind::ForInStatement
                | TypeScriptSyntaxKind::ForOfStatement
                | TypeScriptSyntaxKind::DoWhileStatement
                | TypeScriptSyntaxKind::SwitchStatement
                | TypeScriptSyntaxKind::CaseClause
                | TypeScriptSyntaxKind::DefaultClause
                | TypeScriptSyntaxKind::TryStatement
                | TypeScriptSyntaxKind::CatchClause
                | TypeScriptSyntaxKind::FinallyClause
                | TypeScriptSyntaxKind::ThrowStatement
                | TypeScriptSyntaxKind::ReturnStatement
                | TypeScriptSyntaxKind::BreakStatement
                | TypeScriptSyntaxKind::ContinueStatement
                | TypeScriptSyntaxKind::DebuggerStatement
                | TypeScriptSyntaxKind::WithStatement
                | TypeScriptSyntaxKind::BindingPattern
                | TypeScriptSyntaxKind::ArrayBindingPattern
                | TypeScriptSyntaxKind::ObjectBindingPattern
                | TypeScriptSyntaxKind::BindingElement
                | TypeScriptSyntaxKind::TypeReference
                | TypeScriptSyntaxKind::TypeLiteral
                | TypeScriptSyntaxKind::FunctionType
                | TypeScriptSyntaxKind::ConstructorType
                | TypeScriptSyntaxKind::ArrayType
                | TypeScriptSyntaxKind::TupleType
                | TypeScriptSyntaxKind::UnionType
                | TypeScriptSyntaxKind::IntersectionType
                | TypeScriptSyntaxKind::ConditionalType
                | TypeScriptSyntaxKind::MappedType
                | TypeScriptSyntaxKind::IndexedAccessType
                | TypeScriptSyntaxKind::TypeQuery
                | TypeScriptSyntaxKind::TypePredicate
                | TypeScriptSyntaxKind::Error
        )
    }
}
