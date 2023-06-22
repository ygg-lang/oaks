use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum FSharpSyntaxKind {
    // 基础 tokens
    Root,
    Whitespace,
    Newline,

    // 标识符和字面
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BooleanLiteral,
    UnitLiteral,

    // 关键- 基础
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

    // 关键- 类型
    Type,
    Val,
    Mutable,
    Of,
    As,

    // 关键- 模块和命名空
    Module,
    Namespace,
    Open,

    // 关键- 异常处理
    Try,
    Finally,
    Exception,
    Raise,
    Failwith,

    // 关键- 循环和控制流
    For,
    To,
    Downto,
    Do,
    Done,
    While,
    Yield,
    Return,

    // 关键- 面向对象
    Class,
    Interface,
    Inherit,
    Abstract,
    Override,
    Default,
    Member,
    Static,
    New,

    // 关键- 其他
    Lazy,
    Async,
    Seq,
    Use,
    Begin,
    End,
    Struct,
    Sig,

    // 关键字 - 布尔和特殊值
    True,
    False,
    Null,
    Or,

    // 关键字 - 访问修饰符
    Public,
    Private,
    Internal,

    // 关键字 - 其他
    Inline,
    Extern,
    Upcast,
    Downcast,
    Assert,
    Global,
    Base,
    This,
    Void,

    // 类型关键字
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

    // 运算- 算术
    Plus,     // +
    Minus,    // -
    Star,     // *
    Slash,    // /
    Percent,  // %
    StarStar, // **

    // 运算- 比较
    Equal,        // =
    NotEqual,     // <>
    LessThan,     // <
    LessEqual,    // <=
    GreaterThan,  // >
    GreaterEqual, // >=

    // 运算- 逻辑
    AndAnd, // &&
    OrOr,   // ||
    Not,    // not

    // 运算- 位运
    BitwiseAnd, // &&&
    BitwiseOr,  // |||
    BitwiseXor, // ^^^
    BitwiseNot, // ~~~
    LeftShift,  // <<<
    RightShift, // >>>

    // 运算- 特殊
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

    // 运算符 - 其他
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

    // 分隔符
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

    // 标点符号
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

    // 注释
    LineComment,  // //
    BlockComment, // (* *)

    // 特殊
    Error,
    Eof,
}

impl FSharpSyntaxKind {
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
        )
    }
}

impl TokenType for FSharpSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::BooleanLiteral | Self::UnitLiteral => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::Pipe
            | Self::PipeGreater
            | Self::Ampersand
            | Self::Exclamation
            | Self::ColonEqual
            | Self::Arrow
            | Self::LArrow
            | Self::DoubleColon
            | Self::PlusPlus
            | Self::MinusMinus => UniversalTokenRole::Operator,
            Self::LeftParen
            | Self::RightParen
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracketBar
            | Self::RightBracketBar
            | Self::LeftBracketAngle
            | Self::RightBracketAngle
            | Self::Comma
            | Self::Semicolon
            | Self::Colon
            | Self::Dot
            | Self::DotDot
            | Self::Hash => UniversalTokenRole::Punctuation,
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

impl ElementType for FSharpSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }
}
