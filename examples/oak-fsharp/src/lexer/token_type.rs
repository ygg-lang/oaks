use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

pub type FSharpToken = Token<FSharpTokenType>;

/// F# token types
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FSharpTokenType {
    /// Root node
    Root,
    /// Expression
    Expression,
    /// Whitespace
    Whitespace,
    /// Newline
    Newline,

    /// Identifier
    Identifier,
    /// Integer literal
    IntegerLiteral,
    /// Float literal
    FloatLiteral,
    /// String literal
    StringLiteral,
    /// Character literal
    CharLiteral,
    /// Boolean literal
    BooleanLiteral,
    /// Unit literal
    UnitLiteral,

    /// The 'let' keyword
    Let,
    /// The 'rec' keyword
    Rec,
    /// The 'and' keyword
    And,
    /// The 'in' keyword
    In,
    /// The 'if' keyword
    If,
    /// The 'then' keyword
    Then,
    /// The 'else' keyword
    Else,
    /// The 'elif' keyword
    Elif,
    /// The 'match' keyword
    Match,
    /// The 'with' keyword
    With,
    /// The 'when' keyword
    When,
    /// The 'function' keyword
    Function,
    /// The 'fun' keyword
    Fun,

    /// The 'type' keyword
    Type,
    /// The 'val' keyword
    Val,
    /// The 'mutable' keyword
    Mutable,
    /// The 'of' keyword
    Of,
    /// The 'as' keyword
    As,

    /// The 'module' keyword
    Module,
    /// The 'namespace' keyword
    Namespace,
    /// The 'open' keyword
    Open,

    /// The 'try' keyword
    Try,
    /// The 'finally' keyword
    Finally,
    /// The 'exception' keyword
    Exception,
    /// The 'raise' keyword
    Raise,
    /// The 'failwith' keyword
    Failwith,

    /// The 'for' keyword
    For,
    /// The 'to' keyword
    To,
    /// The 'downto' keyword
    Downto,
    /// The 'do' keyword
    Do,
    /// The 'done' keyword
    Done,
    /// The 'while' keyword
    While,
    /// The 'yield' keyword
    Yield,
    /// The 'return' keyword
    Return,

    /// The 'class' keyword
    Class,
    /// The 'interface' keyword
    Interface,
    /// The 'inherit' keyword
    Inherit,
    /// The 'abstract' keyword
    Abstract,
    /// The 'override' keyword
    Override,
    /// The 'default' keyword
    Default,
    /// The 'member' keyword
    Member,
    /// The 'static' keyword
    Static,
    /// The 'new' keyword
    New,

    /// The 'lazy' keyword
    Lazy,
    /// The 'async' keyword
    Async,
    /// The 'seq' keyword
    Seq,
    /// The 'use' keyword
    Use,
    /// The 'begin' keyword
    Begin,
    /// The 'end' keyword
    End,
    /// The 'struct' keyword
    Struct,
    /// The 'sig' keyword
    Sig,

    /// The 'true' keyword
    True,
    /// The 'false' keyword
    False,
    /// The 'null' keyword
    Null,
    /// The 'or' keyword
    Or,

    /// The 'public' keyword
    Public,
    /// The 'private' keyword
    Private,
    /// The 'internal' keyword
    Internal,

    /// The 'inline' keyword
    Inline,
    /// The 'extern' keyword
    Extern,
    /// The 'upcast' keyword
    Upcast,
    /// The 'downcast' keyword
    Downcast,
    /// The 'assert' keyword
    Assert,
    /// The 'global' keyword
    Global,
    /// The 'base' keyword
    Base,
    /// The 'this' keyword
    This,
    /// The 'void' keyword
    Void,
    /// The 'delegate' keyword
    Delegate,
    /// The 'select' keyword
    Select,

    /// The 'obj' keyword
    Obj,
    /// The 'unit' keyword
    Unit,
    /// The 'int' keyword
    Int,
    /// The 'float' keyword
    Float,
    /// The 'string' keyword
    String,
    /// The 'bool' keyword
    Bool,
    /// The 'char' keyword
    Char,
    /// The 'byte' keyword
    Byte,
    /// The 'sbyte' keyword
    SByte,
    /// The 'int16' keyword
    Int16,
    /// The 'uint16' keyword
    UInt16,
    /// The 'int32' keyword
    Int32,
    /// The 'uint32' keyword
    UInt32,
    /// The 'int64' keyword
    Int64,
    /// The 'uint64' keyword
    UInt64,
    /// The 'nativeint' keyword
    NativeInt,
    /// The 'unativeint' keyword
    UNativeInt,
    /// The 'decimal' keyword
    Decimal,
    /// The 'bigint' keyword
    BigInt,

    /// The '+' operator
    Plus,
    /// The '-' operator
    Minus,
    /// The '*' operator
    Star,
    /// The '/' operator
    Slash,
    /// The '%' operator
    Percent,
    /// The '**' operator
    StarStar,

    /// The '=' operator
    Equal,
    /// The '<>' operator
    NotEqual,
    /// The '<' operator
    LessThan,
    /// The '<=' operator
    LessEqual,
    /// The '>' operator
    GreaterThan,
    /// The '>=' operator
    GreaterEqual,

    /// The '&&' operator
    AndAnd,
    /// The '||' operator
    OrOr,
    /// The 'not' operator
    Not,

    /// The '&&&' operator
    BitwiseAnd,
    /// The '|||' operator
    BitwiseOr,
    /// The '^^^' operator
    BitwiseXor,
    /// The '~~~' operator
    BitwiseNot,
    /// The '<<<' operator
    LeftShift,
    /// The '>>>' operator
    RightShift,

    /// The '->' operator
    Arrow,
    /// The '=>' operator
    DoubleArrow,
    /// The '|' operator
    Pipe,
    /// The '|>' operator
    PipeRight,
    /// The '||' operator
    DoublePipe,
    /// The '::' operator
    Cons,
    /// The '@' operator
    At,
    /// The '>>' operator
    Compose,
    /// The '<<' operator
    ComposeBack,
    /// The '$' operator
    Dollar,
    /// The '|>' operator
    PipeForward,
    /// The '<|' operator
    PipeBackward,

    /// The '&&' logical operator
    LogicalAnd,
    /// The '||' logical operator
    LogicalOr,
    /// The '&' operator
    Ampersand,
    /// The '^' operator
    Caret,
    /// The '~' operator
    Tilde,
    /// The '<' operator
    Less,
    /// The '>' operator
    Greater,
    /// The '|>' operator
    PipeGreater,
    /// The '!' operator
    Exclamation,
    /// The ':=' operator
    ColonEqual,
    /// The '<-' operator
    LArrow,
    /// The '++' operator
    PlusPlus,
    /// The '--' operator
    MinusMinus,

    /// The '(' delimiter
    LeftParen,
    /// The ')' delimiter
    RightParen,
    /// The '[' delimiter
    LeftBracket,
    /// The ']' delimiter
    RightBracket,
    /// The '[|' delimiter
    LeftArrayBracket,
    /// The '|]' delimiter
    RightArrayBracket,
    /// The '[<' delimiter
    LeftBracketBar,
    /// The '>]' delimiter
    RightBracketBar,
    /// The '[ <' delimiter
    LeftBracketAngle,
    /// The '> ]' delimiter
    RightBracketAngle,
    /// The '{' delimiter
    LeftBrace,
    /// The '}' delimiter
    RightBrace,
    /// The '<' delimiter
    LeftAngle,
    /// The '>' delimiter
    RightAngle,

    /// The ',' punctuation
    Comma,
    /// The ';' punctuation
    Semicolon,
    /// The ':' punctuation
    Colon,
    /// The '::' punctuation
    DoubleColon,
    /// The '.' punctuation
    Dot,
    /// The '..' punctuation
    DotDot,
    /// The '?' punctuation
    Question,
    /// The '_' punctuation
    Underscore,
    /// The ''' punctuation
    Apostrophe,
    /// The '`' punctuation
    Backtick,
    /// The '#' punctuation
    Hash,

    /// Line comment
    LineComment,
    /// Block comment
    BlockComment,

    /// Error
    Error,
    /// End of file
    Eof,
}

impl FSharpTokenType {
    /// Checks if it is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Let
                | Self::Rec
                | Self::And
                | Self::In
                | Self::If
                | Self::Then
                | Self::Else
                | Self::Elif
                | Self::Match
                | Self::With
                | Self::When
                | Self::Function
                | Self::Fun
                | Self::Type
                | Self::Val
                | Self::Mutable
                | Self::Of
                | Self::As
                | Self::Module
                | Self::Namespace
                | Self::Open
                | Self::Try
                | Self::Finally
                | Self::Exception
                | Self::Raise
                | Self::Failwith
                | Self::For
                | Self::To
                | Self::Downto
                | Self::Do
                | Self::Done
                | Self::While
                | Self::Yield
                | Self::Return
                | Self::Class
                | Self::Interface
                | Self::Inherit
                | Self::Abstract
                | Self::Override
                | Self::Default
                | Self::Member
                | Self::Static
                | Self::New
                | Self::Lazy
                | Self::Async
                | Self::Seq
                | Self::Use
                | Self::Begin
                | Self::End
                | Self::Struct
                | Self::Sig
                | Self::True
                | Self::False
                | Self::Null
                | Self::Or
                | Self::Public
                | Self::Private
                | Self::Internal
                | Self::Inline
                | Self::Extern
                | Self::Upcast
                | Self::Downcast
                | Self::Assert
                | Self::Delegate
                | Self::Select
        )
    }
}

impl TokenType for FSharpTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalTokenRole::Error,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::BooleanLiteral => UniversalTokenRole::Literal,
            _ => UniversalTokenRole::None,
        }
    }
}
