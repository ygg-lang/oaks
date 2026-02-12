use oak_core::{Token, TokenType, UniversalTokenRole};

/// Perl token type.
pub type PerlToken = Token<PerlTokenType>;

impl PerlTokenType {
    /// Returns `true` if this token type is a regular token.
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    /// Returns `true` if this token type is an internal element (used for incremental parsing).
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
            | Self::Chdir
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

/// Perl token type.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PerlTokenType {
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,

    /// Comment.
    Comment,

    /// String literal.
    StringLiteral,
    /// Number literal.
    NumberLiteral,
    /// Regex literal.
    RegexLiteral,

    /// Identifier.
    Identifier,
    /// `package` keyword.
    Package,
    /// `use` keyword.
    Use,
    /// `sub` keyword.
    Sub,
    /// `my` keyword.
    My,
    /// `our` keyword.
    Our,
    /// `local` keyword.
    Local,
    /// `if` keyword.
    If,
    /// `elsif` keyword.
    Elsif,
    /// `else` keyword.
    Else,
    /// `unless` keyword.
    Unless,
    /// `while` keyword.
    While,
    /// `until` keyword.
    Until,
    /// `for` keyword.
    For,
    /// `foreach` keyword.
    Foreach,
    /// `do` keyword.
    Do,
    /// `last` keyword.
    Last,
    /// `next` keyword.
    Next,
    /// `redo` keyword.
    Redo,
    /// `return` keyword.
    Return,
    /// `die` keyword.
    Die,
    /// `warn` keyword.
    Warn,
    /// `print` keyword.
    Print,
    /// `printf` keyword.
    Printf,
    /// `chomp` keyword.
    Chomp,
    /// `chop` keyword.
    Chop,
    /// `length` keyword.
    Length,
    /// `substr` keyword.
    Substr,
    /// `index` keyword.
    Index,
    /// `rindex` keyword.
    Rindex,
    /// `split` keyword.
    Split,
    /// `join` keyword.
    Join,
    /// `push` keyword.
    Push,
    /// `pop` keyword.
    Pop,
    /// `shift` keyword.
    Shift,
    /// `unshift` keyword.
    Unshift,
    /// `sort` keyword.
    Sort,
    /// `reverse` keyword.
    Reverse,
    /// `keys` keyword.
    Keys,
    /// `values` keyword.
    Values,
    /// `each` keyword.
    Each,
    /// `exists` keyword.
    Exists,
    /// `delete` keyword.
    Delete,
    /// `defined` keyword.
    Defined,
    /// `undef` keyword.
    Undef,
    /// `ref` keyword.
    Ref,
    /// `bless` keyword.
    Bless,
    /// `new` keyword.
    New,
    /// `can` keyword.
    Can,
    /// `isa` keyword.
    Isa,
    /// `scalar` keyword.
    Scalar,
    /// `array` keyword.
    Array,
    /// `hash` keyword.
    Hash,
    /// `code` keyword.
    Code,
    /// `glob` keyword.
    Glob,
    /// `open` keyword.
    Open,
    /// `close` keyword.
    Close,
    /// `read` keyword.
    Read,
    /// `write` keyword.
    Write,
    /// `seek` keyword.
    Seek,
    /// `tell` keyword.
    Tell,
    /// `binmode` keyword.
    Binmode,
    /// `chdir` keyword.
    Chdir,
    /// `mkdir` keyword.
    Mkdir,
    /// `rmdir` keyword.
    Rmdir,
    /// `opendir` keyword.
    Opendir,
    /// `readdir` keyword.
    Readdir,
    /// `closedir` keyword.
    Closedir,
    /// `stat` keyword.
    Stat,
    /// `lstat` keyword.
    Lstat,
    /// `chmod` keyword.
    Chmod,
    /// `chown` keyword.
    Chown,
    /// `link` keyword.
    Link,
    /// `unlink` keyword.
    Unlink,
    /// `rename` keyword.
    Rename,
    /// `symlink` keyword.
    Symlink,
    /// `readlink` keyword.
    Readlink,
    /// `eval` keyword.
    Eval,
    /// `require` keyword.
    Require,
    /// `import` keyword.
    Import,
    /// `no` keyword.
    No,
    /// `strict` keyword.
    Strict,
    /// `warnings` keyword.
    Warnings,
    /// `vars` keyword.
    Vars,
    /// `subs` keyword.
    Subs,
    /// `refs` keyword.
    Refs,

    /// Plus `+`.
    Plus,
    /// Minus `-`.
    Minus,
    /// Increment `++`.
    Increment,
    /// Decrement `--`.
    Decrement,
    /// Multiplication `*`.
    Star,
    /// Division `/`.
    Slash,
    /// Modulo `%`.
    Percent,
    /// Exponentiation `**`.
    Power,
    /// String concatenation `.`.
    Concat,
    /// String repetition `x`.
    Repeat,
    /// Regex match `=~`.
    Match,
    /// Regex not match `!~`.
    NotMatch,
    /// Regex substitution `s///`.
    Substitute,
    /// Character transliteration `tr///`.
    Transliterate,
    /// Equal `==`.
    Equal,
    /// Not equal `!=`.
    NotEqual,
    /// Less than `<`.
    LessThan,
    /// Less than or equal `<=`.
    LessEqual,
    /// Greater than `>`.
    GreaterThan,
    /// Greater than or equal `>=`.
    GreaterEqual,
    /// Spaceship operator `<=>`.
    Spaceship,
    /// String equal `eq`.
    StringEqual,
    /// String not equal `ne`.
    StringNotEqual,
    /// String less than `lt`.
    StringLess,
    /// String less than or equal `le`.
    StringLessEqual,
    /// String greater than `gt`.
    StringGreater,
    /// String greater than or equal `ge`.
    StringGreaterEqual,
    /// String comparison `cmp`.
    StringCompare,
    /// Logical AND `and`.
    And,
    /// Logical OR `or`.
    Or,
    /// Logical NOT `not`.
    Not,
    /// Logical XOR `xor`.
    Xor,
    /// Logical AND `&&`.
    LogicalAnd,
    /// Logical OR `||`.
    LogicalOr,
    /// Logical NOT `!`.
    LogicalNot,
    /// Logical XOR.
    LogicalXor,
    /// Bitwise AND `&`.
    BitwiseAnd,
    /// Bitwise OR `|`.
    BitwiseOr,
    /// Bitwise XOR `^`.
    BitwiseXor,
    /// Bitwise NOT `~`.
    BitwiseNot,
    /// Left shift `<<`.
    LeftShift,
    /// Right shift `>>`.
    RightShift,
    /// Assignment `=`.
    Assign,
    /// Plus assignment `+=`.
    PlusAssign,
    /// Minus assignment `-=`.
    MinusAssign,
    /// Multiplication assignment `*=`.
    MultiplyAssign,
    /// Division assignment `/=`.
    DivideAssign,
    /// Modulo assignment `%=`.
    ModuloAssign,
    /// Exponentiation assignment `**=`.
    PowerAssign,
    /// Concatenation assignment `.=`.
    ConcatAssign,
    /// Logical AND assignment `&&=`.
    LogicalAndAssign,
    /// Logical OR assignment `||=`.
    LogicalOrAssign,
    /// Bitwise AND assignment `&=`.
    BitwiseAndAssign,
    /// Bitwise OR assignment `|=`.
    BitwiseOrAssign,
    /// Bitwise XOR assignment `^=`.
    BitwiseXorAssign,
    /// Left shift assignment `<<=`.
    LeftShiftAssign,
    /// Right shift assignment `>>=`.
    RightShiftAssign,

    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Semicolon `;`.
    Semicolon,
    /// Comma `,`.
    Comma,
    /// Arrow `->`.
    Arrow,
    /// Fat arrow `=>`.
    FatArrow,
    /// Dot `.`.
    Dot,
    /// Range operator `..`.
    Range,
    /// Ellipsis `...`.
    Ellipsis,

    /// Dollar sign `$`.
    Dollar,
    /// At sign `@`.
    At,
    /// Percent sign `%`.
    Percent_,
    /// Ampersand `&`.
    Ampersand,
    /// Backslash `\`.
    Backslash,
    /// Question mark `?`.
    Question,
    /// Colon `:`.
    Colon,
    /// Double colon `::`.
    DoubleColon,
    /// Single quote `'`.
    Quote,
    /// Double quote `"`.
    DoubleQuote,
    /// Backtick `` ` ``.
    Backtick,

    /// Internal program node.
    InternalProgram,
    /// Internal statement node.
    InternalStatement,
    /// Internal expression node.
    InternalExpression,
    /// Internal block node.
    InternalBlock,
    /// Internal subroutine declaration node.
    InternalSubroutineDeclaration,
    /// Internal package declaration node.
    InternalPackageDeclaration,
    /// Internal use statement node.
    InternalUseStatement,
    /// Internal variable declaration node.
    InternalVariableDeclaration,
    /// Internal assignment node.
    InternalAssignment,
    /// Internal function call node.
    InternalFunctionCall,
    /// Internal method call node.
    InternalMethodCall,
    /// Internal array access node.
    InternalArrayAccess,
    /// Internal hash access node.
    InternalHashAccess,
    /// Internal reference node.
    InternalReference,
    /// Internal dereference node.
    InternalDereference,
    /// Internal conditional expression node.
    InternalConditionalExpression,
    /// Internal loop statement node.
    InternalLoopStatement,
    /// Internal if statement node.
    InternalIfStatement,
    /// Internal unless statement node.
    InternalUnlessStatement,
    /// Internal while statement node.
    InternalWhileStatement,
    /// Internal until statement node.
    InternalUntilStatement,
    /// Internal for statement node.
    InternalForStatement,
    /// Internal foreach statement node.
    InternalForeachStatement,
    /// Internal do statement node.
    InternalDoStatement,
    /// Internal eval statement node.
    InternalEvalStatement,
    /// Internal regex match node.
    InternalRegexMatch,
    /// Internal regex substitution node.
    InternalRegexSubstitution,
    /// Internal regex transliteration node.
    InternalRegexTransliteration,

    /// Error.
    Error,
    /// End of stream.
    Eof,
}

/// Perl syntax node.
pub type PerlNode<'a> = oak_core::tree::RedNode<'a, crate::PerlLanguage>;
