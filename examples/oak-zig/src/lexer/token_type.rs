use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type ZigToken = Token<ZigTokenType>;

impl TokenType for ZigTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment | Self::DocComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment | Self::DocComment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ZigTokenType {
    Root,
    // 基础 kind
    Whitespace,
    Newline,
    Comment,
    DocComment,
    Error,
    Eof,

    // 字面量
    Identifier,
    StringLiteral,
    CharLiteral,
    IntegerLiteral,
    FloatLiteral,
    BooleanLiteral,

    // Zig 关键字 - 基本结构
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

    // Zig 关键字 - 控制流
    If,
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

    // Zig 关键字 - 错误处理
    ErrorKeyword,

    // Zig 关键字 - 测试和异步
    Test,
    Async,
    Await,
    Suspend,
    Resume,
    Cancel,

    // Zig 关键字 - 内存管理
    Undefined,
    Null,
    Volatile,
    AllowZero,
    NoAlias,

    // Zig 关键字 - 其他
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
    CShort,
    CUshort,
    CInt,
    CUint,
    CLong,
    CUlong,
    CLongLong,
    CUlongLong,
    CLongDouble,
    CVoid,
    Void,
    ComptimeInt,
    ComptimeFloat,

    // 操作符
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %
    StarStar,     // **
    PlusPercent,  // +%
    MinusPercent, // -%
    StarPercent,  // *%
    PlusPlus,     // ++
    MinusMinus,   // --

    // 位操作符
    Ampersand,      // &
    Pipe,           // |
    Caret,          // ^
    Tilde,          // ~
    LessLess,       // <<
    GreaterGreater, // >>

    // 比较操作符
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // 逻辑操作符
    AndAnd, // and
    OrOr,   // or

    // 赋值操作符
    Assign,               // =
    PlusAssign,           // +=
    MinusAssign,          // -=
    StarAssign,           // *=
    SlashAssign,          // /=
    PercentAssign,        // %=
    AmpersandAssign,      // &=
    PipeAssign,           // |=
    CaretAssign,          // ^=
    LessLessAssign,       // <<=
    GreaterGreaterAssign, // >>=

    // 标点符号
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Semicolon,    // ;
    Comma,        // ,
    Dot,          // .
    DotDot,       // ..
    DotDotDot,    // ...
    DotQuestion,  // .?
    DotStar,      // .*
    Colon,        // :
    Question,     // ?
    Exclamation,  // !
    Arrow,        // ->
    FatArrow,     // =>

    // 特殊操作符
    OrElse,       // orelse
    CatchKeyword, // catch
    TryKeyword,   // try
    AwaitKeyword, // await

    // 内置函数前缀
    At, // ↯
    BuiltinIdentifier,

    // 字符串插值
    StringStart,
    StringEnd,
    StringContent,
    InterpolationStart,
    InterpolationEnd,

    // 多行字符串
    MultilineStringStart,
    MultilineStringEnd,
    MultilineStringContent,

    // 编译时指令
    CompileDirective,

    // 其他
    Text,

    // 非终结符
    VarDeclaration,
    FnDeclaration,
    StructDeclaration,
    EnumDeclaration,
    UnionDeclaration,
    Block,
    IfStatement,
    WhileStatement,
    ForStatement,
    ReturnStatement,
}
