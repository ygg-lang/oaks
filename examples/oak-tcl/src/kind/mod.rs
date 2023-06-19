use oak_core::SyntaxKind;
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

impl SyntaxKind for TclSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(
            self,
            TclSyntaxKind::Whitespace
                | TclSyntaxKind::Newline
                | TclSyntaxKind::Comment
                | TclSyntaxKind::LineComment
                | TclSyntaxKind::BlockComment
        )
    }

    fn is_comment(&self) -> bool {
        matches!(self, TclSyntaxKind::Comment | TclSyntaxKind::LineComment | TclSyntaxKind::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, TclSyntaxKind::Whitespace | TclSyntaxKind::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(
            self,
            TclSyntaxKind::Root
                | TclSyntaxKind::SourceFile
                | TclSyntaxKind::Module
                | TclSyntaxKind::VariableDeclaration
                | TclSyntaxKind::FunctionDeclaration
                | TclSyntaxKind::ClassDeclaration
                | TclSyntaxKind::InterfaceDeclaration
                | TclSyntaxKind::TypeAliasDeclaration
                | TclSyntaxKind::EnumDeclaration
                | TclSyntaxKind::NamespaceDeclaration
                | TclSyntaxKind::ImportDeclaration
                | TclSyntaxKind::ExportDeclaration
                | TclSyntaxKind::BinaryExpression
                | TclSyntaxKind::UnaryExpression
                | TclSyntaxKind::ConditionalExpression
                | TclSyntaxKind::CallExpression
                | TclSyntaxKind::NewExpression
                | TclSyntaxKind::MemberExpression
                | TclSyntaxKind::ArrayExpression
                | TclSyntaxKind::ObjectExpression
                | TclSyntaxKind::FunctionExpression
                | TclSyntaxKind::ArrowFunction
                | TclSyntaxKind::TemplateExpression
                | TclSyntaxKind::TaggedTemplateExpression
                | TclSyntaxKind::AsExpression
                | TclSyntaxKind::TypeAssertionExpression
                | TclSyntaxKind::NonNullExpression
                | TclSyntaxKind::ExpressionStatement
                | TclSyntaxKind::BlockStatement
                | TclSyntaxKind::IfStatement
                | TclSyntaxKind::WhileStatement
                | TclSyntaxKind::ForStatement
                | TclSyntaxKind::ForInStatement
                | TclSyntaxKind::ForOfStatement
                | TclSyntaxKind::DoWhileStatement
                | TclSyntaxKind::SwitchStatement
                | TclSyntaxKind::CaseClause
                | TclSyntaxKind::DefaultClause
                | TclSyntaxKind::TryStatement
                | TclSyntaxKind::CatchClause
                | TclSyntaxKind::FinallyClause
                | TclSyntaxKind::ThrowStatement
                | TclSyntaxKind::ReturnStatement
                | TclSyntaxKind::BreakStatement
                | TclSyntaxKind::ContinueStatement
                | TclSyntaxKind::DebuggerStatement
                | TclSyntaxKind::WithStatement
                | TclSyntaxKind::BindingPattern
                | TclSyntaxKind::ArrayBindingPattern
                | TclSyntaxKind::ObjectBindingPattern
                | TclSyntaxKind::BindingElement
                | TclSyntaxKind::TypeReference
                | TclSyntaxKind::TypeLiteral
                | TclSyntaxKind::FunctionType
                | TclSyntaxKind::ConstructorType
                | TclSyntaxKind::ArrayType
                | TclSyntaxKind::TupleType
                | TclSyntaxKind::UnionType
                | TclSyntaxKind::IntersectionType
                | TclSyntaxKind::ConditionalType
                | TclSyntaxKind::MappedType
                | TclSyntaxKind::IndexedAccessType
                | TclSyntaxKind::TypeQuery
                | TclSyntaxKind::TypePredicate
                | TclSyntaxKind::Error
        )
    }

    fn is_element_type(&self) -> bool {
        !self.is_token_type()
    }
}
