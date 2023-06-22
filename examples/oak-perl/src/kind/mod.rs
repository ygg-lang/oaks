use crate::PerlLanguage;
use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl TokenType for PerlSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

pub type PerlToken = oak_core::Token<PerlSyntaxKind>;
pub type PerlNode<'a> = oak_core::tree::RedNode<'a, PerlLanguage>;

impl PerlSyntaxKind {
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    pub fn is_element(&self) -> bool {
        matches!(
            self,
            Self::Program
                | Self::Statement
                | Self::Expression
                | Self::Block
                | Self::SubroutineDeclaration
                | Self::PackageDeclaration
                | Self::UseStatement
                | Self::VariableDeclaration
                | Self::Assignment
                | Self::FunctionCall
                | Self::MethodCall
                | Self::ArrayAccess
                | Self::HashAccess
                | Self::Reference
                | Self::Dereference
                | Self::ConditionalExpression
                | Self::LoopStatement
                | Self::IfStatement
                | Self::UnlessStatement
                | Self::WhileStatement
                | Self::UntilStatement
                | Self::ForStatement
                | Self::ForeachStatement
                | Self::DoStatement
                | Self::EvalStatement
                | Self::RegexMatch
                | Self::RegexSubstitution
                | Self::RegexTransliteration
        )
    }
}

impl ElementType for PerlSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            Self::Program => UniversalElementRole::Root,
            Self::SubroutineDeclaration | Self::PackageDeclaration | Self::VariableDeclaration => UniversalElementRole::Detail,
            _ => UniversalElementRole::None,
        }
    }
}
