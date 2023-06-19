use oak_core::SyntaxKind;
use serde::Serialize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum FSharpSyntaxKind {
    // 基础 tokens
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
    LogicalAnd, // &&
    LogicalOr,  // ||
    Ampersand,  // &
    Caret,      // ^
    Tilde,      // ~
    Less,       // <
    Greater,    // >

    // 分隔符
    LeftParen,         // (
    RightParen,        // )
    LeftBracket,       // [
    RightBracket,      // ]
    LeftArrayBracket,  // [|
    RightArrayBracket, // |]
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

impl SyntaxKind for FSharpSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error | Self::Eof)
    }

    fn is_element_type(&self) -> bool {
        false
    }
}
