//! Zig token types and roles.

use oak_core::{Token, TokenType, UniversalTokenRole};

/// A token in the Zig language.
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

/// Zig token types.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ZigTokenType {
    /// Root element.
    Root,
    /// Whitespace characters.
    Whitespace,
    /// Newline character.
    Newline,
    /// Comment.
    Comment,
    /// Documentation comment.
    DocComment,
    /// Error token.
    Error,
    /// End of file.
    Eof,

    /// Identifier.
    Identifier,
    /// String literal.
    StringLiteral,
    /// Character literal.
    CharLiteral,
    /// Integer literal.
    IntegerLiteral,
    /// Floating-point literal.
    FloatLiteral,
    /// Boolean literal.
    BooleanLiteral,

    /// `const` keyword.
    Const,
    /// `var` keyword.
    Var,
    /// `fn` keyword.
    Fn,
    /// `struct` keyword.
    Struct,
    /// `union` keyword.
    Union,
    /// `enum` keyword.
    Enum,
    /// `opaque` keyword.
    Opaque,
    /// `type` keyword.
    Type,
    /// `comptime` keyword.
    Comptime,
    /// `inline` keyword.
    Inline,
    /// `noinline` keyword.
    NoInline,
    /// `pub` keyword.
    Pub,
    /// `export` keyword.
    Export,
    /// `extern` keyword.
    Extern,
    /// `packed` keyword.
    Packed,
    /// `align` keyword.
    Align,
    /// `callconv` keyword.
    CallConv,
    /// `linksection` keyword.
    LinkSection,

    /// `if` keyword.
    If,
    /// `else` keyword.
    Else,
    /// `switch` keyword.
    Switch,
    /// `while` keyword.
    While,
    /// `for` keyword.
    For,
    /// `break` keyword.
    Break,
    /// `continue` keyword.
    Continue,
    /// `return` keyword.
    Return,
    /// `defer` keyword.
    Defer,
    /// `errdefer` keyword.
    ErrDefer,
    /// `unreachable` keyword.
    Unreachable,
    /// `noreturn` keyword.
    NoReturn,

    /// `error` keyword.
    ErrorKeyword,

    /// `test` keyword.
    Test,
    /// `async` keyword.
    Async,
    /// `await` keyword.
    Await,
    /// `suspend` keyword.
    Suspend,
    /// `resume` keyword.
    Resume,
    /// `cancel` keyword.
    Cancel,

    /// `undefined` keyword.
    Undefined,
    /// `null` keyword.
    Null,
    /// `volatile` keyword.
    Volatile,
    /// `allowzero` keyword.
    AllowZero,
    /// `noalias` keyword.
    NoAlias,

    /// `and` keyword.
    And,
    /// `or` keyword.
    Or,
    /// `anyframe` keyword.
    AnyFrame,
    /// `anytype` keyword.
    AnyType,
    /// `threadlocal` keyword.
    ThreadLocal,

    /// `bool` type.
    Bool,
    /// `i8` type.
    I8,
    /// `i16` type.
    I16,
    /// `i32` type.
    I32,
    /// `i64` type.
    I64,
    /// `i128` type.
    I128,
    /// `isize` type.
    Isize,
    /// `u8` type.
    U8,
    /// `u16` type.
    U16,
    /// `u32` type.
    U32,
    /// `u64` type.
    U64,
    /// `u128` type.
    U128,
    /// `usize` type.
    Usize,
    /// `f16` type.
    F16,
    /// `f32` type.
    F32,
    /// `f64` type.
    F64,
    /// `f80` type.
    F80,
    /// `f128` type.
    F128,
    /// `c_short` type.
    CShort,
    /// `c_ushort` type.
    CUshort,
    /// `c_int` type.
    CInt,
    /// `c_uint` type.
    CUint,
    /// `c_long` type.
    CLong,
    /// `c_ulong` type.
    CUlong,
    /// `c_longlong` type.
    CLongLong,
    /// `c_ulonglong` type.
    CUlongLong,
    /// `c_longdouble` type.
    CLongDouble,
    /// `c_void` type.
    CVoid,
    /// `void` type.
    Void,
    /// `comptime_int` type.
    ComptimeInt,
    /// `comptime_float` type.
    ComptimeFloat,

    /// `+` operator.
    Plus,
    /// `-` operator.
    Minus,
    /// `*` operator.
    Star,
    /// `/` operator.
    Slash,
    /// `%` operator.
    Percent,
    /// `**` operator.
    StarStar,
    /// `+%` operator.
    PlusPercent,
    /// `-%` operator.
    MinusPercent,
    /// `*%` operator.
    StarPercent,
    /// `++` operator.
    PlusPlus,
    /// `--` operator.
    MinusMinus,

    /// `&` operator.
    Ampersand,
    /// `|` operator.
    Pipe,
    /// `^` operator.
    Caret,
    /// `~` operator.
    Tilde,
    /// `<<` operator.
    LessLess,
    /// `>>` operator.
    GreaterGreater,

    /// `==` operator.
    Equal,
    /// `!=` operator.
    NotEqual,
    /// `<` operator.
    Less,
    /// `>` operator.
    Greater,
    /// `<=` operator.
    LessEqual,
    /// `>=` operator.
    GreaterEqual,

    /// `and` logical operator.
    AndAnd,
    /// `or` logical operator.
    OrOr,

    /// `=` operator.
    Assign,
    /// `+=` operator.
    PlusAssign,
    /// `-=` operator.
    MinusAssign,
    /// `*=` operator.
    StarAssign,
    /// `/=` operator.
    SlashAssign,
    /// `%=` operator.
    PercentAssign,
    /// `&=` operator.
    AmpersandAssign,
    /// `|=` operator.
    PipeAssign,
    /// `^=` operator.
    CaretAssign,
    /// `<<=` operator.
    LessLessAssign,
    /// `>>=` operator.
    GreaterGreaterAssign,

    /// `(` symbol.
    LeftParen,
    /// `)` symbol.
    RightParen,
    /// `{` symbol.
    LeftBrace,
    /// `}` symbol.
    RightBrace,
    /// `[` symbol.
    LeftBracket,
    /// `]` symbol.
    RightBracket,
    /// `;` symbol.
    Semicolon,
    /// `,` symbol.
    Comma,
    /// `.` symbol.
    Dot,
    /// `..` symbol.
    DotDot,
    /// `...` symbol.
    DotDotDot,
    /// `.?` operator.
    DotQuestion,
    /// `.*` operator.
    DotStar,
    /// `:` symbol.
    Colon,
    /// `?` symbol.
    Question,
    /// `!` symbol.
    Exclamation,
    /// `->` operator.
    Arrow,
    /// `=>` operator.
    FatArrow,

    /// `orelse` operator.
    OrElse,
    /// `catch` operator.
    CatchKeyword,
    /// `try` operator.
    TryKeyword,
    /// `await` operator.
    AwaitKeyword,

    /// `@` symbol.
    At,
    /// Built-in identifier.
    BuiltinIdentifier,

    /// Start of a string literal.
    StringStart,
    /// End of a string literal.
    StringEnd,
    /// Content of a string literal.
    StringContent,
    /// Start of string interpolation.
    InterpolationStart,
    /// End of string interpolation.
    InterpolationEnd,

    /// Start of a multiline string.
    MultilineStringStart,
    /// End of a multiline string.
    MultilineStringEnd,
    /// Content of a multiline string.
    MultilineStringContent,

    /// Compile-time directive.
    CompileDirective,

    /// Text content.
    Text,

    /// Variable declaration non-terminal.
    VarDeclaration,
    /// Function declaration non-terminal.
    FnDeclaration,
    /// Struct declaration non-terminal.
    StructDeclaration,
    /// Enum declaration non-terminal.
    EnumDeclaration,
    /// Union declaration non-terminal.
    UnionDeclaration,
    /// Block non-terminal.
    Block,
    /// If statement non-terminal.
    IfStatement,
    /// While statement non-terminal.
    WhileStatement,
    /// For statement non-terminal.
    ForStatement,
    /// Return statement non-terminal.
    ReturnStatement,
}
