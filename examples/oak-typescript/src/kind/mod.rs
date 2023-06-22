use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    ClassBody,
    ImportDeclaration,
    ExportDeclaration,
    ImportClause,
    NamedImports,
    ImportSpecifier,
    Parameter,
    CallArgument,
    PropertyDeclaration,
    MethodDeclaration,
    ConstructorDeclaration,

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
    BooleanLiteral,
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

impl TypeScriptSyntaxKind {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Abstract
                | Self::Any
                | Self::As
                | Self::Asserts
                | Self::Async
                | Self::Await
                | Self::Boolean
                | Self::Break
                | Self::Case
                | Self::Catch
                | Self::Class
                | Self::Const
                | Self::Constructor
                | Self::Continue
                | Self::Debugger
                | Self::Declare
                | Self::Default
                | Self::Delete
                | Self::Do
                | Self::Else
                | Self::Enum
                | Self::Export
                | Self::Extends
                | Self::False
                | Self::Finally
                | Self::For
                | Self::From
                | Self::Function
                | Self::Get
                | Self::Global
                | Self::If
                | Self::Implements
                | Self::Import
                | Self::In
                | Self::Infer
                | Self::Instanceof
                | Self::Interface
                | Self::Is
                | Self::Keyof
                | Self::Let
                | Self::Namespace
                | Self::Never
                | Self::New
                | Self::Null
                | Self::Number
                | Self::Object
                | Self::Of
                | Self::Package
                | Self::Private
                | Self::Protected
                | Self::Public
                | Self::Readonly
                | Self::Require
                | Self::Return
                | Self::Set
                | Self::Static
                | Self::String
                | Self::Super
                | Self::Switch
                | Self::Symbol
                | Self::This
                | Self::Throw
                | Self::True
                | Self::Try
                | Self::Type
                | Self::Typeof
                | Self::Undefined
                | Self::Unique
                | Self::Unknown
                | Self::Var
                | Self::Void
                | Self::While
                | Self::With
                | Self::Yield
        )
    }

    pub fn from_keyword(s: &str) -> Option<Self> {
        match s {
            "abstract" => Some(Self::Abstract),
            "any" => Some(Self::Any),
            "as" => Some(Self::As),
            "asserts" => Some(Self::Asserts),
            "async" => Some(Self::Async),
            "await" => Some(Self::Await),
            "boolean" => Some(Self::Boolean),
            "break" => Some(Self::Break),
            "case" => Some(Self::Case),
            "catch" => Some(Self::Catch),
            "class" => Some(Self::Class),
            "const" => Some(Self::Const),
            "constructor" => Some(Self::Constructor),
            "continue" => Some(Self::Continue),
            "debugger" => Some(Self::Debugger),
            "declare" => Some(Self::Declare),
            "default" => Some(Self::Default),
            "delete" => Some(Self::Delete),
            "do" => Some(Self::Do),
            "else" => Some(Self::Else),
            "enum" => Some(Self::Enum),
            "export" => Some(Self::Export),
            "extends" => Some(Self::Extends),
            "false" => Some(Self::False),
            "finally" => Some(Self::Finally),
            "for" => Some(Self::For),
            "from" => Some(Self::From),
            "function" => Some(Self::Function),
            "get" => Some(Self::Get),
            "global" => Some(Self::Global),
            "if" => Some(Self::If),
            "implements" => Some(Self::Implements),
            "import" => Some(Self::Import),
            "in" => Some(Self::In),
            "infer" => Some(Self::Infer),
            "instanceof" => Some(Self::Instanceof),
            "interface" => Some(Self::Interface),
            "is" => Some(Self::Is),
            "keyof" => Some(Self::Keyof),
            "let" => Some(Self::Let),
            "namespace" => Some(Self::Namespace),
            "never" => Some(Self::Never),
            "new" => Some(Self::New),
            "null" => Some(Self::Null),
            "number" => Some(Self::Number),
            "object" => Some(Self::Object),
            "of" => Some(Self::Of),
            "package" => Some(Self::Package),
            "private" => Some(Self::Private),
            "protected" => Some(Self::Protected),
            "public" => Some(Self::Public),
            "readonly" => Some(Self::Readonly),
            "require" => Some(Self::Require),
            "return" => Some(Self::Return),
            "set" => Some(Self::Set),
            "static" => Some(Self::Static),
            "string" => Some(Self::String),
            "super" => Some(Self::Super),
            "switch" => Some(Self::Switch),
            "symbol" => Some(Self::Symbol),
            "this" => Some(Self::This),
            "throw" => Some(Self::Throw),
            "true" => Some(Self::True),
            "try" => Some(Self::Try),
            "type" => Some(Self::Type),
            "typeof" => Some(Self::Typeof),
            "undefined" => Some(Self::Undefined),
            "unique" => Some(Self::Unique),
            "unknown" => Some(Self::Unknown),
            "var" => Some(Self::Var),
            "void" => Some(Self::Void),
            "while" => Some(Self::While),
            "with" => Some(Self::With),
            "yield" => Some(Self::Yield),
            _ => None,
        }
    }
}

impl TokenType for TypeScriptSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::IdentifierName => UniversalTokenRole::Name,
            Self::True | Self::False | Self::Null | Self::NumericLiteral | Self::StringLiteral | Self::BigIntLiteral | Self::TemplateString | Self::RegexLiteral => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::StarStar
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::EqualEqual
            | Self::NotEqual
            | Self::EqualEqualEqual
            | Self::NotEqualEqual
            | Self::AmpersandAmpersand
            | Self::PipePipe
            | Self::Exclamation
            | Self::Ampersand
            | Self::Pipe
            | Self::Caret
            | Self::Tilde
            | Self::LeftShift
            | Self::RightShift
            | Self::UnsignedRightShift
            | Self::Equal
            | Self::PlusEqual
            | Self::MinusEqual
            | Self::StarEqual
            | Self::SlashEqual
            | Self::PercentEqual
            | Self::StarStarEqual
            | Self::LeftShiftEqual
            | Self::RightShiftEqual
            | Self::UnsignedRightShiftEqual
            | Self::AmpersandEqual
            | Self::PipeEqual
            | Self::CaretEqual
            | Self::AmpersandAmpersandEqual
            | Self::PipePipeEqual
            | Self::QuestionQuestionEqual
            | Self::PlusPlus
            | Self::MinusMinus
            | Self::Question
            | Self::QuestionQuestion
            | Self::QuestionDot
            | Self::Arrow => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Comma | Self::Dot | Self::DotDotDot | Self::Colon => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for TypeScriptSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::SourceFile => UniversalElementRole::Root,
            Self::VariableDeclaration
            | Self::FunctionDeclaration
            | Self::ClassDeclaration
            | Self::InterfaceDeclaration
            | Self::TypeAliasDeclaration
            | Self::EnumDeclaration
            | Self::NamespaceDeclaration
            | Self::ImportDeclaration
            | Self::ExportDeclaration => UniversalElementRole::Definition,
            Self::ExpressionStatement | Self::BlockStatement | Self::IfStatement | Self::WhileStatement | Self::ForStatement | Self::ReturnStatement | Self::BreakStatement | Self::ContinueStatement | Self::ThrowStatement | Self::TryStatement => {
                UniversalElementRole::Statement
            }
            Self::BinaryExpression | Self::UnaryExpression | Self::ConditionalExpression | Self::CallExpression | Self::MemberExpression | Self::ArrayExpression | Self::ObjectExpression | Self::AsExpression => UniversalElementRole::Expression,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::Root | Self::SourceFile)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}
