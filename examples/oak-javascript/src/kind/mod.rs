use oak_core::SyntaxKind;

/// JavaScript 语法种类（包含节点与词法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    // 词法种类 - 操作    Plus,           // +
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

impl SyntaxKind for JavaScriptSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment | Self::Whitespace | Self::Newline)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(
            self,
            Self::Root
                | Self::Program
                | Self::Statement
                | Self::Expression
                | Self::Declaration
                | Self::FunctionDeclaration
                | Self::VariableDeclaration
                | Self::ClassDeclaration
                | Self::ImportDeclaration
                | Self::ExportDeclaration
                | Self::IfStatement
                | Self::WhileStatement
                | Self::ForStatement
                | Self::BlockStatement
                | Self::ExpressionStatement
                | Self::ReturnStatement
                | Self::BreakStatement
                | Self::ContinueStatement
                | Self::ThrowStatement
                | Self::TryStatement
                | Self::CatchClause
                | Self::FinallyClause
                | Self::SwitchStatement
                | Self::CaseClause
                | Self::DefaultClause
                | Self::BinaryExpression
                | Self::UnaryExpression
                | Self::AssignmentExpression
                | Self::CallExpression
                | Self::MemberExpression
                | Self::ConditionalExpression
                | Self::ArrayExpression
                | Self::ObjectExpression
                | Self::FunctionExpression
                | Self::ArrowFunctionExpression
                | Self::NewExpression
                | Self::UpdateExpression
                | Self::LogicalExpression
                | Self::SequenceExpression
                | Self::ThisExpression
                | Self::Identifier
                | Self::Literal
                | Self::TemplateLiteral
                | Self::TaggedTemplateExpression
                | Self::ObjectPattern
                | Self::ArrayPattern
                | Self::RestElement
                | Self::AssignmentPattern
                | Self::Property
                | Self::ErrorNode
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            Self::Root
                | Self::Program
                | Self::Statement
                | Self::Expression
                | Self::Declaration
                | Self::FunctionDeclaration
                | Self::VariableDeclaration
                | Self::ClassDeclaration
                | Self::ImportDeclaration
                | Self::ExportDeclaration
                | Self::IfStatement
                | Self::WhileStatement
                | Self::ForStatement
                | Self::BlockStatement
                | Self::ExpressionStatement
                | Self::ReturnStatement
                | Self::BreakStatement
                | Self::ContinueStatement
                | Self::ThrowStatement
                | Self::TryStatement
                | Self::CatchClause
                | Self::FinallyClause
                | Self::SwitchStatement
                | Self::CaseClause
                | Self::DefaultClause
                | Self::BinaryExpression
                | Self::UnaryExpression
                | Self::AssignmentExpression
                | Self::CallExpression
                | Self::MemberExpression
                | Self::ConditionalExpression
                | Self::ArrayExpression
                | Self::ObjectExpression
                | Self::FunctionExpression
                | Self::ArrowFunctionExpression
                | Self::NewExpression
                | Self::UpdateExpression
                | Self::LogicalExpression
                | Self::SequenceExpression
                | Self::ThisExpression
                | Self::Identifier
                | Self::Literal
                | Self::TemplateLiteral
                | Self::TaggedTemplateExpression
                | Self::ObjectPattern
                | Self::ArrayPattern
                | Self::RestElement
                | Self::AssignmentPattern
                | Self::Property
                | Self::ErrorNode
        )
    }
}
