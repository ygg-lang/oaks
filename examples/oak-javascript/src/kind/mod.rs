use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// JavaScript 语法种类（包含节点与词法）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JavaScriptSyntaxKind {
    // 节点种类
    Root,
    Program,
    Statement,
    Expression,
    Declaration,
    FunctionDeclaration,
    VariableDeclaration,
    ClassDeclaration,
    ImportDeclaration,
    ExportDeclaration,
    IfStatement,
    WhileStatement,
    ForStatement,
    BlockStatement,
    ExpressionStatement,
    ReturnStatement,
    BreakStatement,
    ContinueStatement,
    ThrowStatement,
    TryStatement,
    CatchClause,
    FinallyClause,
    SwitchStatement,
    CaseClause,
    DefaultClause,

    // 表达式节
    BinaryExpression,
    UnaryExpression,
    AssignmentExpression,
    CallExpression,
    MemberExpression,
    ConditionalExpression,
    ArrayExpression,
    ObjectExpression,
    FunctionExpression,
    ArrowFunctionExpression,
    NewExpression,
    UpdateExpression,
    LogicalExpression,
    SequenceExpression,
    ThisExpression,
    Identifier,
    Literal,
    TemplateLiteral,
    TaggedTemplateExpression,

    // 模式
    ObjectPattern,
    ArrayPattern,
    RestElement,
    AssignmentPattern,
    Property,

    // 错误节点
    ErrorNode,

    // 词法种类 - 关键
    Abstract,
    As,
    Async,
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
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
    Function,
    If,
    Implements,
    Import,
    In,
    Instanceof,
    Interface,
    Let,
    New,
    Null,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Static,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Undefined,
    Var,
    Void,
    While,
    With,
    Yield,

    // 词法种类 - 操作符
    Plus,               // +
    Minus,              // -
    Star,               // *
    Slash,              // /
    Percent,            // %
    StarStar,           // **
    PlusPlus,           // ++
    MinusMinus,         // --
    LeftShift,          // <<
    RightShift,         // >>
    UnsignedRightShift, // >>>
    Less,               // <
    Greater,            // >
    LessEqual,          // <=
    GreaterEqual,       // >=
    EqualEqual,         // ==
    NotEqual,           // !=
    EqualEqualEqual,    // ===
    NotEqualEqual,      // !==
    Ampersand,          // &
    Pipe,               // |
    Caret,              // ^
    Exclamation,        // !
    Tilde,              // ~
    AmpersandAmpersand, // &&
    PipePipe,           // ||
    Question,           // ?
    QuestionQuestion,   // ??
    QuestionDot,        // ?.

    // 赋值操作符
    Equal,                   // =
    PlusEqual,               // +=
    MinusEqual,              // -=
    StarEqual,               // *=
    SlashEqual,              // /=
    PercentEqual,            // %=
    StarStarEqual,           // **=
    LeftShiftEqual,          // <<=
    RightShiftEqual,         // >>=
    UnsignedRightShiftEqual, // >>>=
    AmpersandEqual,          // &=
    PipeEqual,               // |=
    CaretEqual,              // ^=
    AmpersandAmpersandEqual, // &&=
    PipePipeEqual,           // ||=
    QuestionQuestionEqual,   // ??=

    // 标点符号
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Semicolon,    // ;
    Comma,        // ,
    Dot,          // .
    DotDotDot,    // ...
    Colon,        // :
    Arrow,        // =>

    // 字面
    StringLiteral,
    NumericLiteral,
    BigIntLiteral,
    RegexLiteral,
    TemplateString,
    TemplateHead,
    TemplateMiddle,
    TemplateTail,

    // 标识
    IdentifierName,

    // 注释和空
    LineComment,
    BlockComment,
    Whitespace,
    Newline,

    // 特殊
    Eof,
    Error,
}

impl TokenType for JavaScriptSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for JavaScriptSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
