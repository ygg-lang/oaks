use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type PerlToken = Token<PerlTokenType>;

impl PerlTokenType {
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    pub fn is_element(&self) -> bool {
        matches!(
            self,
            Self::InternalProgram
                | Self::InternalStatement
                | Self::InternalExpression
                | Self::InternalBlock
                | Self::InternalSubroutineDeclaration
                | Self::InternalPackageDeclaration
                | Self::InternalUseStatement
                | Self::InternalVariableDeclaration
                | Self::InternalAssignment
                | Self::InternalFunctionCall
                | Self::InternalMethodCall
                | Self::InternalArrayAccess
                | Self::InternalHashAccess
                | Self::InternalReference
                | Self::InternalDereference
                | Self::InternalConditionalExpression
                | Self::InternalLoopStatement
                | Self::InternalIfStatement
                | Self::InternalUnlessStatement
                | Self::InternalWhileStatement
                | Self::InternalUntilStatement
                | Self::InternalForStatement
                | Self::InternalForeachStatement
                | Self::InternalDoStatement
                | Self::InternalEvalStatement
                | Self::InternalRegexMatch
                | Self::InternalRegexSubstitution
                | Self::InternalRegexTransliteration
        )
    }
}

impl TokenType for PerlTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            Self::Whitespace | Self::Newline => Whitespace,
            Self::Comment => Comment,
            Self::StringLiteral | Self::NumberLiteral | Self::RegexLiteral => Literal,
            Self::Identifier => Name,
            Self::Package
            | Self::Use
            | Self::Sub
            | Self::My
            | Self::Our
            | Self::Local
            | Self::If
            | Self::Elsif
            | Self::Else
            | Self::Unless
            | Self::While
            | Self::Until
            | Self::For
            | Self::Foreach
            | Self::Do
            | Self::Last
            | Self::Next
            | Self::Redo
            | Self::Return
            | Self::Die
            | Self::Warn
            | Self::Print
            | Self::Printf
            | Self::Chomp
            | Self::Chop
            | Self::Length
            | Self::Substr
            | Self::Index
            | Self::Rindex
            | Self::Split
            | Self::Join
            | Self::Push
            | Self::Pop
            | Self::Shift
            | Self::Unshift
            | Self::Sort
            | Self::Reverse
            | Self::Keys
            | Self::Values
            | Self::Each
            | Self::Exists
            | Self::Delete
            | Self::Defined
            | Self::Undef
            | Self::Ref
            | Self::Bless
            | Self::New
            | Self::Can
            | Self::Isa
            | Self::Scalar
            | Self::Array
            | Self::Hash
            | Self::Code
            | Self::Glob
            | Self::Open
            | Self::Close
            | Self::Read
            | Self::Write
            | Self::Seek
            | Self::Tell
            | Self::Binmode
            | Self::Chodir
            | Self::Mkdir
            | Self::Rmdir
            | Self::Opendir
            | Self::Readdir
            | Self::Closedir
            | Self::Stat
            | Self::Lstat
            | Self::Chmod
            | Self::Chown
            | Self::Link
            | Self::Unlink
            | Self::Rename
            | Self::Symlink
            | Self::Readlink
            | Self::Eval
            | Self::Require
            | Self::Import
            | Self::No
            | Self::Strict
            | Self::Warnings
            | Self::Vars
            | Self::Subs
            | Self::Refs => Keyword,
            Self::Plus
            | Self::Minus
            | Self::Increment
            | Self::Decrement
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Power
            | Self::Concat
            | Self::Repeat
            | Self::Match
            | Self::NotMatch
            | Self::Substitute
            | Self::Transliterate
            | Self::Equal
            | Self::NotEqual
            | Self::LessThan
            | Self::LessEqual
            | Self::GreaterThan
            | Self::GreaterEqual
            | Self::Spaceship
            | Self::StringEqual
            | Self::StringNotEqual
            | Self::StringLess
            | Self::StringLessEqual
            | Self::StringGreater
            | Self::StringGreaterEqual
            | Self::StringCompare
            | Self::And
            | Self::Or
            | Self::Not
            | Self::Xor
            | Self::LogicalAnd
            | Self::LogicalOr
            | Self::LogicalNot
            | Self::LogicalXor
            | Self::BitwiseAnd
            | Self::BitwiseOr
            | Self::BitwiseXor
            | Self::BitwiseNot
            | Self::LeftShift
            | Self::RightShift
            | Self::Assign
            | Self::PlusAssign
            | Self::MinusAssign
            | Self::MultiplyAssign
            | Self::DivideAssign
            | Self::ModuloAssign
            | Self::PowerAssign
            | Self::ConcatAssign
            | Self::LogicalAndAssign
            | Self::LogicalOrAssign
            | Self::BitwiseAndAssign
            | Self::BitwiseOrAssign
            | Self::BitwiseXorAssign
            | Self::LeftShiftAssign
            | Self::RightShiftAssign
            | Self::Arrow
            | Self::FatArrow => Operator,
            Self::LeftParen
            | Self::RightParen
            | Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracket
            | Self::RightBracket
            | Self::Semicolon
            | Self::Comma
            | Self::Dot
            | Self::Range
            | Self::Ellipsis
            | Self::Dollar
            | Self::At
            | Self::Percent_
            | Self::Ampersand
            | Self::Backslash
            | Self::Question
            | Self::Colon
            | Self::DoubleColon
            | Self::Quote
            | Self::DoubleQuote
            | Self::Backtick => Punctuation,
            Self::Error => Error,
            Self::Eof => Eof,
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerlTokenType {
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

    // Composite nodes (Internal use)
    InternalProgram,
    InternalStatement,
    InternalExpression,
    InternalBlock,
    InternalSubroutineDeclaration,
    InternalPackageDeclaration,
    InternalUseStatement,
    InternalVariableDeclaration,
    InternalAssignment,
    InternalFunctionCall,
    InternalMethodCall,
    InternalArrayAccess,
    InternalHashAccess,
    InternalReference,
    InternalDereference,
    InternalConditionalExpression,
    InternalLoopStatement,
    InternalIfStatement,
    InternalUnlessStatement,
    InternalWhileStatement,
    InternalUntilStatement,
    InternalForStatement,
    InternalForeachStatement,
    InternalDoStatement,
    InternalEvalStatement,
    InternalRegexMatch,
    InternalRegexSubstitution,
    InternalRegexTransliteration,

    // Error and EOF
    Error,
    Eof,
}

pub type PerlNode<'a> = oak_core::tree::RedNode<'a, crate::PerlLanguage>;
