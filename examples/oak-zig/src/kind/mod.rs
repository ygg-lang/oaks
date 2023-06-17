#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZigSyntaxKind {
    // 基础 kind
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,

    // 字面    Identifier,
    StringLiteral,
    CharLiteral,
    IntegerLiteral,
    FloatLiteral,
    BooleanLiteral,

    // Zig 关键- 基本结构
    Const,
    Var,
    Fn,
    Struct,
    Union,
    Enum,
    Opaque,
    Type,
    Comptime,
    Inline,
    NoInline,
    Pub,
    Export,
    Extern,
    Packed,
    Align,
    CallConv,
    LinkSection,

    // Zig 关键- 控制    If,
    Else,
    Switch,
    While,
    For,
    Break,
    Continue,
    Return,
    Defer,
    ErrDefer,
    Unreachable,
    NoReturn,

    // Zig 关键- 错误处理
    Try,
    Catch,
    Orelse,
    Error,

    // Zig 关键- 测试和异    Test,
    Async,
    Await,
    Suspend,
    Resume,
    Cancel,

    // Zig 关键- 内存管理
    Undefined,
    Null,
    Volatile,
    AllowZero,
    NoAlias,

    // Zig 关键- 其他
    And,
    Or,
    AnyFrame,
    AnyType,
    ThreadLocal,

    // 基本类型
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    F16,
    F32,
    F64,
    F80,
    F128,
    C_Short,
    C_UShort,
    C_Int,
    C_UInt,
    C_Long,
    C_ULong,
    C_LongLong,
    C_ULongLong,
    C_LongDouble,
    C_Void,
    Void,
    NoReturn,
    Comptime_Int,
    Comptime_Float,

    // 操作    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    StarStar,       // **
    PlusPercent,    // +%
    MinusPercent,   // -%
    StarPercent,    // *%
    PlusPlus,       // ++
    StarStar,       // **
    
    // 位操作符
    Ampersand,      // &
    Pipe,           // |
    Caret,          // ^
    Tilde,          // ~
    LessLess,       // <<
    GreaterGreater, // >>

    // 比较操作    Equal,          // ==
    NotEqual,       // !=
    Less,           // <
    Greater,        // >
    LessEqual,      // <=
    GreaterEqual,   // >=

    // 逻辑操作    AndAnd,         // and
    OrOr,           // or

    // 赋值操作符
    Assign,         // =
    PlusAssign,     // +=
    MinusAssign,    // -=
    StarAssign,     // *=
    SlashAssign,    // /=
    PercentAssign,  // %=
    AmpersandAssign, // &=
    PipeAssign,     // |=
    CaretAssign,    // ^=
    LessLessAssign, // <<=
    GreaterGreaterAssign, // >>=

    // 标点符号
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    Semicolon,      // ;
    Comma,          // ,
    Dot,            // .
    DotDot,         // ..
    DotDotDot,      // ...
    Colon,          // :
    Question,       // ?
    Exclamation,    // !
    Arrow,          // =>
    FatArrow,       // =>

    // 特殊操作    Orelse,         // orelse
    Catch,          // catch
    Try,            // try
    Await,          // await

    // 内置函数前缀
    At,             // @

    // 字符串插    StringStart,
    StringEnd,
    StringContent,
    InterpolationStart,
    InterpolationEnd,

    // 多行字符    MultilineStringStart,
    MultilineStringEnd,
    MultilineStringContent,

    // 文档注释
    DocComment,

    // 编译时指    CompileDirective,

    // 其他
    Text,
}

impl oak_core::SyntaxKind for ZigSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment | Self::DocComment)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn is_eof(&self) -> bool {
        matches!(self, Self::Eof)
    }
}
