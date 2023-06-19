use oak_core::SyntaxKind;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerlSyntaxKind {
    // Basic tokens
    Whitespace,
    Newline,

    // Comments
    Comment,

    // Literals
    StringLiteral,
    NumberLiteral,
    RegexLiteral,

    // Identifiers and keywords
    Identifier,
    Package,
    Use,
    Sub,
    My,
    Our,
    Local,
    If,
    Elsif,
    Else,
    Unless,
    While,
    Until,
    For,
    Foreach,
    Do,
    Last,
    Next,
    Redo,
    Return,
    Die,
    Warn,
    Print,
    Printf,
    Chomp,
    Chop,
    Length,
    Substr,
    Index,
    Rindex,
    Split,
    Join,
    Push,
    Pop,
    Shift,
    Unshift,
    Sort,
    Reverse,
    Keys,
    Values,
    Each,
    Exists,
    Delete,
    Defined,
    Undef,
    Ref,
    Bless,
    New,
    Can,
    Isa,
    Scalar,
    Array,
    Hash,
    Code,
    Glob,
    Open,
    Close,
    Read,
    Write,
    Seek,
    Tell,

    Binmode,
    Chodir,
    Mkdir,
    Rmdir,
    Opendir,
    Readdir,
    Closedir,
    Stat,
    Lstat,
    Chmod,
    Chown,
    Link,
    Unlink,
    Rename,
    Symlink,
    Readlink,
    Eval,
    Require,
    Import,
    No,
    Strict,
    Warnings,
    Vars,
    Subs,
    Refs,

    // Operators
    Plus,
    Minus,
    Increment,
    Decrement,
    Star,
    Slash,
    Percent,
    Power,
    Concat,
    Repeat,
    Match,
    NotMatch,
    Substitute,
    Transliterate,
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    Spaceship,
    StringEqual,
    StringNotEqual,
    StringLess,
    StringLessEqual,
    StringGreater,
    StringGreaterEqual,
    StringCompare,
    And,
    Or,
    Not,
    Xor,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    LogicalXor,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    PowerAssign,
    ConcatAssign,
    LogicalAndAssign,
    LogicalOrAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LeftShiftAssign,
    RightShiftAssign,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Arrow,
    FatArrow,
    Dot,
    Range,
    Ellipsis,

    // Special characters
    Dollar,
    At,
    Percent_,
    Ampersand,
    Backslash,
    Question,
    Colon,
    DoubleColon,
    Quote,
    DoubleQuote,
    Backtick,

    // Composite nodes
    Program,
    Statement,
    Expression,
    Block,
    SubroutineDeclaration,
    PackageDeclaration,
    UseStatement,
    VariableDeclaration,
    Assignment,
    FunctionCall,
    MethodCall,
    ArrayAccess,
    HashAccess,
    Reference,
    Dereference,
    ConditionalExpression,
    LoopStatement,
    IfStatement,
    UnlessStatement,
    WhileStatement,
    UntilStatement,
    ForStatement,
    ForeachStatement,
    DoStatement,
    EvalStatement,
    RegexMatch,
    RegexSubstitution,
    RegexTransliteration,

    // Error and EOF
    Error,
    Eof,
}

impl SyntaxKind for PerlSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
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
            Self::LoopStatement
                | Self::IfStatement
                | Self::UnlessStatement
                | Self::WhileStatement
                | Self::UntilStatement
                | Self::ForStatement
                | Self::ForeachStatement
                | Self::DoStatement
                | Self::EvalStatement
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            Self::LoopStatement
                | Self::IfStatement
                | Self::UnlessStatement
                | Self::WhileStatement
                | Self::UntilStatement
                | Self::ForStatement
                | Self::ForeachStatement
                | Self::DoStatement
                | Self::EvalStatement
        )
    }
}

pub type PerlToken = oak_core::Token<PerlSyntaxKind>;
pub type PerlNode = oak_core::tree::RedNode<PerlSyntaxKind>;
