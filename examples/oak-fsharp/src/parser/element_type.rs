use oak_core::{ElementType, UniversalElementRole};

/// F# element types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FSharpElementType {
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
    /// Literal
    Literal,

    /// Match case
    MatchCase,
    /// Wildcard pattern
    WildcardPattern,
    /// Identifier pattern
    IdentifierPattern,
    /// Tuple pattern
    TuplePattern,
    /// List pattern
    ListPattern,
    /// Active pattern
    ActivePattern,
    /// Pattern
    Pattern,
    /// Lambda expression
    Lambda,
    /// Parenthesized expression
    Parenthesized,
    /// List expression
    List,
    /// Record expression
    Record,
    /// Function application
    Application,

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

impl FSharpElementType {
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

impl ElementType for FSharpElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::FSharpTokenType> for FSharpElementType {
    fn from(token: crate::lexer::token_type::FSharpTokenType) -> Self {
                match token {
            crate::lexer::token_type::FSharpTokenType::Root => Self::Root,
            crate::lexer::token_type::FSharpTokenType::Expression => Self::Expression,
            crate::lexer::token_type::FSharpTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::FSharpTokenType::Newline => Self::Newline,
            crate::lexer::token_type::FSharpTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::FSharpTokenType::IntegerLiteral => Self::IntegerLiteral,
            crate::lexer::token_type::FSharpTokenType::FloatLiteral => Self::FloatLiteral,
            crate::lexer::token_type::FSharpTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::FSharpTokenType::CharLiteral => Self::CharLiteral,
            crate::lexer::token_type::FSharpTokenType::BooleanLiteral => Self::BooleanLiteral,
            crate::lexer::token_type::FSharpTokenType::UnitLiteral => Self::UnitLiteral,
            crate::lexer::token_type::FSharpTokenType::Let => Self::Let,
            crate::lexer::token_type::FSharpTokenType::Rec => Self::Rec,
            crate::lexer::token_type::FSharpTokenType::And => Self::And,
            crate::lexer::token_type::FSharpTokenType::In => Self::In,
            crate::lexer::token_type::FSharpTokenType::If => Self::If,
            crate::lexer::token_type::FSharpTokenType::Then => Self::Then,
            crate::lexer::token_type::FSharpTokenType::Else => Self::Else,
            crate::lexer::token_type::FSharpTokenType::Elif => Self::Elif,
            crate::lexer::token_type::FSharpTokenType::Match => Self::Match,
            crate::lexer::token_type::FSharpTokenType::With => Self::With,
            crate::lexer::token_type::FSharpTokenType::When => Self::When,
            crate::lexer::token_type::FSharpTokenType::Function => Self::Function,
            crate::lexer::token_type::FSharpTokenType::Fun => Self::Fun,
            crate::lexer::token_type::FSharpTokenType::Type => Self::Type,
            crate::lexer::token_type::FSharpTokenType::Val => Self::Val,
            crate::lexer::token_type::FSharpTokenType::Mutable => Self::Mutable,
            crate::lexer::token_type::FSharpTokenType::Of => Self::Of,
            crate::lexer::token_type::FSharpTokenType::As => Self::As,
            crate::lexer::token_type::FSharpTokenType::Module => Self::Module,
            crate::lexer::token_type::FSharpTokenType::Namespace => Self::Namespace,
            crate::lexer::token_type::FSharpTokenType::Open => Self::Open,
            crate::lexer::token_type::FSharpTokenType::Try => Self::Try,
            crate::lexer::token_type::FSharpTokenType::Finally => Self::Finally,
            crate::lexer::token_type::FSharpTokenType::Exception => Self::Exception,
            crate::lexer::token_type::FSharpTokenType::Raise => Self::Raise,
            crate::lexer::token_type::FSharpTokenType::Failwith => Self::Failwith,
            crate::lexer::token_type::FSharpTokenType::For => Self::For,
            crate::lexer::token_type::FSharpTokenType::To => Self::To,
            crate::lexer::token_type::FSharpTokenType::Downto => Self::Downto,
            crate::lexer::token_type::FSharpTokenType::Do => Self::Do,
            crate::lexer::token_type::FSharpTokenType::Done => Self::Done,
            crate::lexer::token_type::FSharpTokenType::While => Self::While,
            crate::lexer::token_type::FSharpTokenType::Yield => Self::Yield,
            crate::lexer::token_type::FSharpTokenType::Return => Self::Return,
            crate::lexer::token_type::FSharpTokenType::Class => Self::Class,
            crate::lexer::token_type::FSharpTokenType::Interface => Self::Interface,
            crate::lexer::token_type::FSharpTokenType::Inherit => Self::Inherit,
            crate::lexer::token_type::FSharpTokenType::Abstract => Self::Abstract,
            crate::lexer::token_type::FSharpTokenType::Override => Self::Override,
            crate::lexer::token_type::FSharpTokenType::Default => Self::Default,
            crate::lexer::token_type::FSharpTokenType::Member => Self::Member,
            crate::lexer::token_type::FSharpTokenType::Static => Self::Static,
            crate::lexer::token_type::FSharpTokenType::New => Self::New,
            crate::lexer::token_type::FSharpTokenType::Lazy => Self::Lazy,
            crate::lexer::token_type::FSharpTokenType::Async => Self::Async,
            crate::lexer::token_type::FSharpTokenType::Seq => Self::Seq,
            crate::lexer::token_type::FSharpTokenType::Use => Self::Use,
            crate::lexer::token_type::FSharpTokenType::Begin => Self::Begin,
            crate::lexer::token_type::FSharpTokenType::End => Self::End,
            crate::lexer::token_type::FSharpTokenType::Struct => Self::Struct,
            crate::lexer::token_type::FSharpTokenType::Sig => Self::Sig,
            crate::lexer::token_type::FSharpTokenType::True => Self::True,
            crate::lexer::token_type::FSharpTokenType::False => Self::False,
            crate::lexer::token_type::FSharpTokenType::Null => Self::Null,
            crate::lexer::token_type::FSharpTokenType::Or => Self::Or,
            crate::lexer::token_type::FSharpTokenType::Public => Self::Public,
            crate::lexer::token_type::FSharpTokenType::Private => Self::Private,
            crate::lexer::token_type::FSharpTokenType::Internal => Self::Internal,
            crate::lexer::token_type::FSharpTokenType::Inline => Self::Inline,
            crate::lexer::token_type::FSharpTokenType::Extern => Self::Extern,
            crate::lexer::token_type::FSharpTokenType::Upcast => Self::Upcast,
            crate::lexer::token_type::FSharpTokenType::Downcast => Self::Downcast,
            crate::lexer::token_type::FSharpTokenType::Assert => Self::Assert,
            crate::lexer::token_type::FSharpTokenType::Global => Self::Global,
            crate::lexer::token_type::FSharpTokenType::Base => Self::Base,
            crate::lexer::token_type::FSharpTokenType::This => Self::This,
            crate::lexer::token_type::FSharpTokenType::Void => Self::Void,
            crate::lexer::token_type::FSharpTokenType::Delegate => Self::Delegate,
            crate::lexer::token_type::FSharpTokenType::Select => Self::Select,
            crate::lexer::token_type::FSharpTokenType::Obj => Self::Obj,
            crate::lexer::token_type::FSharpTokenType::Unit => Self::Unit,
            crate::lexer::token_type::FSharpTokenType::Int => Self::Int,
            crate::lexer::token_type::FSharpTokenType::Float => Self::Float,
            crate::lexer::token_type::FSharpTokenType::String => Self::String,
            crate::lexer::token_type::FSharpTokenType::Bool => Self::Bool,
            crate::lexer::token_type::FSharpTokenType::Char => Self::Char,
            crate::lexer::token_type::FSharpTokenType::Byte => Self::Byte,
            crate::lexer::token_type::FSharpTokenType::SByte => Self::SByte,
            crate::lexer::token_type::FSharpTokenType::Int16 => Self::Int16,
            crate::lexer::token_type::FSharpTokenType::UInt16 => Self::UInt16,
            crate::lexer::token_type::FSharpTokenType::Int32 => Self::Int32,
            crate::lexer::token_type::FSharpTokenType::UInt32 => Self::UInt32,
            crate::lexer::token_type::FSharpTokenType::Int64 => Self::Int64,
            crate::lexer::token_type::FSharpTokenType::UInt64 => Self::UInt64,
            crate::lexer::token_type::FSharpTokenType::NativeInt => Self::NativeInt,
            crate::lexer::token_type::FSharpTokenType::UNativeInt => Self::UNativeInt,
            crate::lexer::token_type::FSharpTokenType::Decimal => Self::Decimal,
            crate::lexer::token_type::FSharpTokenType::BigInt => Self::BigInt,
            crate::lexer::token_type::FSharpTokenType::Plus => Self::Plus,
            crate::lexer::token_type::FSharpTokenType::Minus => Self::Minus,
            crate::lexer::token_type::FSharpTokenType::Star => Self::Star,
            crate::lexer::token_type::FSharpTokenType::Slash => Self::Slash,
            crate::lexer::token_type::FSharpTokenType::Percent => Self::Percent,
            crate::lexer::token_type::FSharpTokenType::StarStar => Self::StarStar,
            crate::lexer::token_type::FSharpTokenType::Equal => Self::Equal,
            crate::lexer::token_type::FSharpTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::FSharpTokenType::LessThan => Self::LessThan,
            crate::lexer::token_type::FSharpTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::FSharpTokenType::GreaterThan => Self::GreaterThan,
            crate::lexer::token_type::FSharpTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::FSharpTokenType::AndAnd => Self::AndAnd,
            crate::lexer::token_type::FSharpTokenType::OrOr => Self::OrOr,
            crate::lexer::token_type::FSharpTokenType::Not => Self::Not,
            crate::lexer::token_type::FSharpTokenType::BitwiseAnd => Self::BitwiseAnd,
            crate::lexer::token_type::FSharpTokenType::BitwiseOr => Self::BitwiseOr,
            crate::lexer::token_type::FSharpTokenType::BitwiseXor => Self::BitwiseXor,
            crate::lexer::token_type::FSharpTokenType::BitwiseNot => Self::BitwiseNot,
            crate::lexer::token_type::FSharpTokenType::LeftShift => Self::LeftShift,
            crate::lexer::token_type::FSharpTokenType::RightShift => Self::RightShift,
            crate::lexer::token_type::FSharpTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::FSharpTokenType::DoubleArrow => Self::DoubleArrow,
            crate::lexer::token_type::FSharpTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::FSharpTokenType::PipeRight => Self::PipeRight,
            crate::lexer::token_type::FSharpTokenType::DoublePipe => Self::DoublePipe,
            crate::lexer::token_type::FSharpTokenType::Cons => Self::Cons,
            crate::lexer::token_type::FSharpTokenType::At => Self::At,
            crate::lexer::token_type::FSharpTokenType::Compose => Self::Compose,
            crate::lexer::token_type::FSharpTokenType::ComposeBack => Self::ComposeBack,
            crate::lexer::token_type::FSharpTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::FSharpTokenType::LogicalAnd => Self::LogicalAnd,
            crate::lexer::token_type::FSharpTokenType::LogicalOr => Self::LogicalOr,
            crate::lexer::token_type::FSharpTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::FSharpTokenType::Caret => Self::Caret,
            crate::lexer::token_type::FSharpTokenType::Tilde => Self::Tilde,
            crate::lexer::token_type::FSharpTokenType::Less => Self::Less,
            crate::lexer::token_type::FSharpTokenType::Greater => Self::Greater,
            crate::lexer::token_type::FSharpTokenType::PipeGreater => Self::PipeGreater,
            crate::lexer::token_type::FSharpTokenType::Exclamation => Self::Exclamation,
            crate::lexer::token_type::FSharpTokenType::ColonEqual => Self::ColonEqual,
            crate::lexer::token_type::FSharpTokenType::LArrow => Self::LArrow,
            crate::lexer::token_type::FSharpTokenType::PlusPlus => Self::PlusPlus,
            crate::lexer::token_type::FSharpTokenType::MinusMinus => Self::MinusMinus,
            crate::lexer::token_type::FSharpTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::FSharpTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::FSharpTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::FSharpTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::FSharpTokenType::LeftArrayBracket => Self::LeftArrayBracket,
            crate::lexer::token_type::FSharpTokenType::RightArrayBracket => Self::RightArrayBracket,
            crate::lexer::token_type::FSharpTokenType::LeftBracketBar => Self::LeftBracketBar,
            crate::lexer::token_type::FSharpTokenType::RightBracketBar => Self::RightBracketBar,
            crate::lexer::token_type::FSharpTokenType::LeftBracketAngle => Self::LeftBracketAngle,
            crate::lexer::token_type::FSharpTokenType::RightBracketAngle => Self::RightBracketAngle,
            crate::lexer::token_type::FSharpTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::FSharpTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::FSharpTokenType::LeftAngle => Self::LeftAngle,
            crate::lexer::token_type::FSharpTokenType::RightAngle => Self::RightAngle,
            crate::lexer::token_type::FSharpTokenType::Comma => Self::Comma,
            crate::lexer::token_type::FSharpTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::FSharpTokenType::Colon => Self::Colon,
            crate::lexer::token_type::FSharpTokenType::DoubleColon => Self::DoubleColon,
            crate::lexer::token_type::FSharpTokenType::Dot => Self::Dot,
            crate::lexer::token_type::FSharpTokenType::DotDot => Self::DotDot,
            crate::lexer::token_type::FSharpTokenType::Question => Self::Question,
            crate::lexer::token_type::FSharpTokenType::Underscore => Self::Underscore,
            crate::lexer::token_type::FSharpTokenType::Apostrophe => Self::Apostrophe,
            crate::lexer::token_type::FSharpTokenType::Backtick => Self::Backtick,
            crate::lexer::token_type::FSharpTokenType::Hash => Self::Hash,
            crate::lexer::token_type::FSharpTokenType::LineComment => Self::LineComment,
            crate::lexer::token_type::FSharpTokenType::BlockComment => Self::BlockComment,
            crate::lexer::token_type::FSharpTokenType::Error => Self::Error,
            crate::lexer::token_type::FSharpTokenType::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}
