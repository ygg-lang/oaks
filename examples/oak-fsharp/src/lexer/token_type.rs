use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type FSharpToken = Token<FSharpTokenType>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FSharpTokenType {
    // Basic tokens
    Root,
    Expression,
    Whitespace,
    Newline,

    // Identifiers and literals
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BooleanLiteral,
    UnitLiteral,

    // Keywords - Basic
    Let,
    Rec,
    And,
    In,
    If,
    Then,
    Else,
    Elif,
    Match,
    With,
    When,
    Function,
    Fun,

    // Keywords - Type
    Type,
    Val,
    Mutable,
    Of,
    As,

    // Keywords - Modules and Namespaces
    Module,
    Namespace,
    Open,

    // Keywords - Exception Handling
    Try,
    Finally,
    Exception,
    Raise,
    Failwith,

    // Keywords - Loops and Control Flow
    For,
    To,
    Downto,
    Do,
    Done,
    While,
    Yield,
    Return,

    // Keywords - Object Oriented
    Class,
    Interface,
    Inherit,
    Abstract,
    Override,
    Default,
    Member,
    Static,
    New,

    // Keywords - Other
    Lazy,
    Async,
    Seq,
    Use,
    Begin,
    End,
    Struct,
    Sig,

    // Keywords - Boolean and Special Values
    True,
    False,
    Null,
    Or,

    // Keywords - Access Modifiers
    Public,
    Private,
    Internal,

    // Keywords - Other
    Inline,
    Extern,
    Upcast,
    Downcast,
    Assert,
    Global,
    Base,
    This,
    Void,
    Delegate,
    Select,

    // Type keywords
    Obj,
    Unit,
    Int,
    Float,
    String,
    Bool,
    Char,
    Byte,
    SByte,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    NativeInt,
    UNativeInt,
    Decimal,
    BigInt,

    // Operations - Arithmetic
    Plus,     // +
    Minus,    // -
    Star,     // *
    Slash,    // /
    Percent,  // %
    StarStar, // **

    // Operations - Comparison
    Equal,        // =
    NotEqual,     // <>
    LessThan,     // <
    LessEqual,    // <=
    GreaterThan,  // >
    GreaterEqual, // >=

    // Operations - Logic
    AndAnd, // &&
    OrOr,   // ||
    Not,    // not

    // Operations - Bitwise
    BitwiseAnd, // &&&
    BitwiseOr,  // |||
    BitwiseXor, // ^^^
    BitwiseNot, // ~~~
    LeftShift,  // <<<
    RightShift, // >>>

    // Operations - Special
    Arrow,       // ->
    DoubleArrow, // =>
    Pipe,        // |
    PipeRight,   // |>
    DoublePipe,  // ||
    Cons,        // ::
    At,          // @
    Compose,     // >>
    ComposeBack, // <<
    Dollar,      // $

    // Operators - Other
    LogicalAnd,  // &&
    LogicalOr,   // ||
    Ampersand,   // &
    Caret,       // ^
    Tilde,       // ~
    Less,        // <
    Greater,     // >
    PipeGreater, // |>
    Exclamation, // !
    ColonEqual,  // :=
    LArrow,      // <-
    PlusPlus,    // ++
    MinusMinus,  // --

    // Delimiters
    LeftParen,         // (
    RightParen,        // )
    LeftBracket,       // [
    RightBracket,      // ]
    LeftArrayBracket,  // [|
    RightArrayBracket, // |]
    LeftBracketBar,    // [<
    RightBracketBar,   // >]
    LeftBracketAngle,  // [ <
    RightBracketAngle, // > ]
    LeftBrace,         // {
    RightBrace,        // }
    LeftAngle,         // <
    RightAngle,        // >

    // Punctuation
    Comma,       // ,
    Semicolon,   // ;
    Colon,       // :
    DoubleColon, // ::
    Dot,         // .
    DotDot,      // ..
    Question,    // ?
    Underscore,  // _
    Apostrophe,  // '
    Backtick,    // `
    Hash,        // #

    // Comments
    LineComment,  // //
    BlockComment, // (* *)

    // Special
    Error,
    Eof,
}

impl FSharpTokenType {
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
