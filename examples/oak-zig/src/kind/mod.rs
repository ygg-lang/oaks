use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ZigSyntaxKind {
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
    At, // @
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

impl TokenType for ZigSyntaxKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Const
            | Self::Var
            | Self::Fn
            | Self::Struct
            | Self::Union
            | Self::Enum
            | Self::Opaque
            | Self::Type
            | Self::Comptime
            | Self::Inline
            | Self::NoInline
            | Self::Pub
            | Self::Export
            | Self::Extern
            | Self::Packed
            | Self::Align
            | Self::CallConv
            | Self::LinkSection
            | Self::If
            | Self::Else
            | Self::Switch
            | Self::While
            | Self::For
            | Self::Break
            | Self::Continue
            | Self::Return
            | Self::Defer
            | Self::ErrDefer
            | Self::Unreachable
            | Self::NoReturn
            | Self::ErrorKeyword
            | Self::Test
            | Self::Async
            | Self::Await
            | Self::Suspend
            | Self::Resume
            | Self::Cancel
            | Self::Undefined
            | Self::Null
            | Self::Volatile
            | Self::AllowZero
            | Self::NoAlias
            | Self::And
            | Self::Or
            | Self::AnyFrame
            | Self::AnyType
            | Self::ThreadLocal
            | Self::OrElse
            | Self::CatchKeyword
            | Self::TryKeyword
            | Self::AwaitKeyword => UniversalTokenRole::Keyword,

            Self::Bool
            | Self::I8
            | Self::I16
            | Self::I32
            | Self::I64
            | Self::I128
            | Self::Isize
            | Self::U8
            | Self::U16
            | Self::U32
            | Self::U64
            | Self::U128
            | Self::Usize
            | Self::F16
            | Self::F32
            | Self::F64
            | Self::F80
            | Self::F128
            | Self::CShort
            | Self::CUshort
            | Self::CInt
            | Self::CUint
            | Self::CLong
            | Self::CUlong
            | Self::CLongLong
            | Self::CUlongLong
            | Self::CLongDouble
            | Self::CVoid
            | Self::Void
            | Self::ComptimeInt
            | Self::ComptimeFloat => UniversalTokenRole::Keyword,

            Self::Identifier => UniversalTokenRole::Name,

            Self::StringLiteral | Self::CharLiteral | Self::IntegerLiteral | Self::FloatLiteral | Self::BooleanLiteral => UniversalTokenRole::Literal,

            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::StarStar
            | Self::PlusPercent
            | Self::MinusPercent
            | Self::StarPercent
            | Self::PlusPlus
            | Self::Ampersand
            | Self::Pipe
            | Self::Caret
            | Self::Tilde
            | Self::LessLess
            | Self::GreaterGreater
            | Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::AndAnd
            | Self::OrOr
            | Self::Assign
            | Self::PlusAssign
            | Self::MinusAssign
            | Self::StarAssign
            | Self::SlashAssign
            | Self::PercentAssign
            | Self::AmpersandAssign
            | Self::PipeAssign
            | Self::CaretAssign
            | Self::LessLessAssign
            | Self::GreaterGreaterAssign
            | Self::At => UniversalTokenRole::Operator,

            Self::LeftParen
            | Self::RightParen
            | Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracket
            | Self::RightBracket
            | Self::Semicolon
            | Self::Comma
            | Self::Dot
            | Self::DotDot
            | Self::DotDotDot
            | Self::Colon
            | Self::Question
            | Self::Exclamation
            | Self::Arrow
            | Self::FatArrow => UniversalTokenRole::Punctuation,

            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment | Self::DocComment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment | Self::DocComment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment | Self::DocComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for ZigSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::FnDeclaration | Self::StructDeclaration | Self::EnumDeclaration | Self::UnionDeclaration => UniversalElementRole::Definition,
            Self::VarDeclaration | Self::IfStatement | Self::WhileStatement | Self::ForStatement | Self::ReturnStatement => UniversalElementRole::Statement,
            Self::Block => UniversalElementRole::Container,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
