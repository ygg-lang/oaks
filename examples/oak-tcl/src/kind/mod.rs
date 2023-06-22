use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TclSyntaxKind {
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

    // Tcl 关键字
    If,
    Else,
    ElseIf,
    For,
    While,
    ForEach,
    Proc,
    Return,
    Break,
    Continue,
    Set,
    Unset,
    Global,
    Upvar,
    Variable,

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
    Dollar,

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
    Number,
    BigIntLiteral,
    TemplateString,
    RegexLiteral,

    // 标识符
    Identifier,
    IdentifierName,

    // 注释和空白
    Comment,
    LineComment,
    BlockComment,
    Whitespace,
    Newline,

    // 特殊符号
    Eof,
}

impl TokenType for TclSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            TclSyntaxKind::Whitespace | TclSyntaxKind::Newline => UniversalTokenRole::Whitespace,
            TclSyntaxKind::Comment | TclSyntaxKind::LineComment | TclSyntaxKind::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for TclSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
