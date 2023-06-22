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

impl JavaScriptSyntaxKind {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Abstract
                | Self::As
                | Self::Async
                | Self::Await
                | Self::Break
                | Self::Case
                | Self::Catch
                | Self::Class
                | Self::Const
                | Self::Continue
                | Self::Debugger
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
                | Self::Function
                | Self::If
                | Self::Implements
                | Self::Import
                | Self::In
                | Self::Instanceof
                | Self::Interface
                | Self::Let
                | Self::New
                | Self::Null
                | Self::Package
                | Self::Private
                | Self::Protected
                | Self::Public
                | Self::Return
                | Self::Static
                | Self::Super
                | Self::Switch
                | Self::This
                | Self::Throw
                | Self::True
                | Self::Try
                | Self::Typeof
                | Self::Undefined
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
            "as" => Some(Self::As),
            "async" => Some(Self::Async),
            "await" => Some(Self::Await),
            "break" => Some(Self::Break),
            "case" => Some(Self::Case),
            "catch" => Some(Self::Catch),
            "class" => Some(Self::Class),
            "const" => Some(Self::Const),
            "continue" => Some(Self::Continue),
            "debugger" => Some(Self::Debugger),
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
            "function" => Some(Self::Function),
            "if" => Some(Self::If),
            "implements" => Some(Self::Implements),
            "import" => Some(Self::Import),
            "in" => Some(Self::In),
            "instanceof" => Some(Self::Instanceof),
            "interface" => Some(Self::Interface),
            "let" => Some(Self::Let),
            "new" => Some(Self::New),
            "null" => Some(Self::Null),
            "package" => Some(Self::Package),
            "private" => Some(Self::Private),
            "protected" => Some(Self::Protected),
            "public" => Some(Self::Public),
            "return" => Some(Self::Return),
            "static" => Some(Self::Static),
            "super" => Some(Self::Super),
            "switch" => Some(Self::Switch),
            "this" => Some(Self::This),
            "throw" => Some(Self::Throw),
            "true" => Some(Self::True),
            "try" => Some(Self::Try),
            "typeof" => Some(Self::Typeof),
            "undefined" => Some(Self::Undefined),
            "var" => Some(Self::Var),
            "void" => Some(Self::Void),
            "while" => Some(Self::While),
            "with" => Some(Self::With),
            "yield" => Some(Self::Yield),
            _ => None,
        }
    }
}

impl TokenType for JavaScriptSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::IdentifierName => UniversalTokenRole::Name,
            Self::StringLiteral | Self::NumericLiteral | Self::BigIntLiteral | Self::RegexLiteral | Self::TemplateString | Self::True | Self::False | Self::Null | Self::Undefined => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::StarStar
            | Self::PlusPlus
            | Self::MinusMinus
            | Self::LeftShift
            | Self::RightShift
            | Self::UnsignedRightShift
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::EqualEqual
            | Self::NotEqual
            | Self::EqualEqualEqual
            | Self::NotEqualEqual
            | Self::Ampersand
            | Self::Pipe
            | Self::Caret
            | Self::Exclamation
            | Self::Tilde
            | Self::AmpersandAmpersand
            | Self::PipePipe
            | Self::Question
            | Self::QuestionQuestion
            | Self::QuestionDot
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
            | Self::Arrow => UniversalTokenRole::Operator,
            Self::LeftParen
            | Self::RightParen
            | Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracket
            | Self::RightBracket
            | Self::Semicolon
            | Self::Comma
            | Self::Dot
            | Self::DotDotDot
            | Self::Colon => UniversalTokenRole::Punctuation,
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

impl ElementType for JavaScriptSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::Program => UniversalElementRole::Root,
            Self::ErrorNode | Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::Root | Self::Program)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::ErrorNode | Self::Error)
    }
}
