use oak_core::{ElementType, UniversalElementRole};

/// Zig element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ZigElementType {
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
    /// Generic literal.
    Literal,

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

    /// Function declaration.
    FnDeclaration,
    /// Variable declaration.
    VarDeclaration,
    /// Struct declaration.
    StructDeclaration,
    /// Enum declaration.
    EnumDeclaration,
    /// Union declaration.
    UnionDeclaration,
    /// If statement.
    IfStatement,
    /// While statement.
    WhileStatement,
    /// For statement.
    ForStatement,
    /// Return statement.
    ReturnStatement,
    /// Block of code.
    Block,
    /// Binary expression.
    BinaryExpr,
    /// Unary expression.
    UnaryExpr,

    /// Container field.
    ContainerField,
    /// Break statement.
    BreakStatement,
    /// Continue statement.
    ContinueStatement,
    /// Defer statement.
    DeferStatement,
}

impl ElementType for ZigElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ZigTokenType> for ZigElementType {
    fn from(token: crate::lexer::token_type::ZigTokenType) -> Self {
        use crate::lexer::token_type::ZigTokenType as T;
        match token {
            T::Root => ZigElementType::Root,
            T::Whitespace => ZigElementType::Whitespace,
            T::Newline => ZigElementType::Newline,
            T::Comment => ZigElementType::Comment,
            T::DocComment => ZigElementType::DocComment,
            T::Error => ZigElementType::Error,
            T::Eof => ZigElementType::Eof,
            T::Identifier => ZigElementType::Identifier,
            T::StringLiteral => ZigElementType::StringLiteral,
            T::CharLiteral => ZigElementType::CharLiteral,
            T::IntegerLiteral => ZigElementType::IntegerLiteral,
            T::FloatLiteral => ZigElementType::FloatLiteral,
            T::BooleanLiteral => ZigElementType::BooleanLiteral,
            T::Const => ZigElementType::Const,
            T::Var => ZigElementType::Var,
            T::Fn => ZigElementType::Fn,
            T::Struct => ZigElementType::Struct,
            T::Union => ZigElementType::Union,
            T::Enum => ZigElementType::Enum,
            T::Opaque => ZigElementType::Opaque,
            T::Type => ZigElementType::Type,
            T::Comptime => ZigElementType::Comptime,
            T::Inline => ZigElementType::Inline,
            T::NoInline => ZigElementType::NoInline,
            T::Pub => ZigElementType::Pub,
            T::Export => ZigElementType::Export,
            T::Extern => ZigElementType::Extern,
            T::Packed => ZigElementType::Packed,
            T::Align => ZigElementType::Align,
            T::CallConv => ZigElementType::CallConv,
            T::LinkSection => ZigElementType::LinkSection,
            T::If => ZigElementType::If,
            T::Else => ZigElementType::Else,
            T::Switch => ZigElementType::Switch,
            T::While => ZigElementType::While,
            T::For => ZigElementType::For,
            T::Break => ZigElementType::Break,
            T::Continue => ZigElementType::Continue,
            T::Return => ZigElementType::Return,
            T::Defer => ZigElementType::Defer,
            T::ErrDefer => ZigElementType::ErrDefer,
            T::Unreachable => ZigElementType::Unreachable,
            T::NoReturn => ZigElementType::NoReturn,
            T::ErrorKeyword => ZigElementType::ErrorKeyword,
            T::Test => ZigElementType::Test,
            T::Async => ZigElementType::Async,
            T::Await => ZigElementType::Await,
            T::Suspend => ZigElementType::Suspend,
            T::Resume => ZigElementType::Resume,
            T::Cancel => ZigElementType::Cancel,
            T::Undefined => ZigElementType::Undefined,
            T::Null => ZigElementType::Null,
            T::Volatile => ZigElementType::Volatile,
            T::AllowZero => ZigElementType::AllowZero,
            T::NoAlias => ZigElementType::NoAlias,
            T::And => ZigElementType::And,
            T::Or => ZigElementType::Or,
            T::AnyFrame => ZigElementType::AnyFrame,
            T::AnyType => ZigElementType::AnyType,
            T::ThreadLocal => ZigElementType::ThreadLocal,
            T::Bool => ZigElementType::Bool,
            T::I8 => ZigElementType::I8,
            T::I16 => ZigElementType::I16,
            T::I32 => ZigElementType::I32,
            T::I64 => ZigElementType::I64,
            T::I128 => ZigElementType::I128,
            T::Isize => ZigElementType::Isize,
            T::U8 => ZigElementType::U8,
            T::U16 => ZigElementType::U16,
            T::U32 => ZigElementType::U32,
            T::U64 => ZigElementType::U64,
            T::U128 => ZigElementType::U128,
            T::Usize => ZigElementType::Usize,
            T::F16 => ZigElementType::F16,
            T::F32 => ZigElementType::F32,
            T::F64 => ZigElementType::F64,
            T::F80 => ZigElementType::F80,
            T::F128 => ZigElementType::F128,
            T::CShort => ZigElementType::CShort,
            T::CUshort => ZigElementType::CUshort,
            T::CInt => ZigElementType::CInt,
            T::CUint => ZigElementType::CUint,
            T::CLong => ZigElementType::CLong,
            T::CUlong => ZigElementType::CUlong,
            T::CLongLong => ZigElementType::CLongLong,
            T::CUlongLong => ZigElementType::CUlongLong,
            T::CLongDouble => ZigElementType::CLongDouble,
            T::CVoid => ZigElementType::CVoid,
            T::Void => ZigElementType::Void,
            T::ComptimeInt => ZigElementType::ComptimeInt,
            T::ComptimeFloat => ZigElementType::ComptimeFloat,
            T::Plus => ZigElementType::Plus,
            T::Minus => ZigElementType::Minus,
            T::Star => ZigElementType::Star,
            T::Slash => ZigElementType::Slash,
            T::Percent => ZigElementType::Percent,
            T::StarStar => ZigElementType::StarStar,
            T::PlusPercent => ZigElementType::PlusPercent,
            T::MinusPercent => ZigElementType::MinusPercent,
            T::StarPercent => ZigElementType::StarPercent,
            T::PlusPlus => ZigElementType::PlusPlus,
            T::MinusMinus => ZigElementType::MinusMinus,
            T::Ampersand => ZigElementType::Ampersand,
            T::Pipe => ZigElementType::Pipe,
            T::Caret => ZigElementType::Caret,
            T::Tilde => ZigElementType::Tilde,
            T::LessLess => ZigElementType::LessLess,
            T::GreaterGreater => ZigElementType::GreaterGreater,
            T::Equal => ZigElementType::Equal,
            T::NotEqual => ZigElementType::NotEqual,
            T::Less => ZigElementType::Less,
            T::Greater => ZigElementType::Greater,
            T::LessEqual => ZigElementType::LessEqual,
            T::GreaterEqual => ZigElementType::GreaterEqual,
            T::AndAnd => ZigElementType::AndAnd,
            T::OrOr => ZigElementType::OrOr,
            T::Assign => ZigElementType::Assign,
            T::PlusAssign => ZigElementType::PlusAssign,
            T::MinusAssign => ZigElementType::MinusAssign,
            T::StarAssign => ZigElementType::StarAssign,
            T::SlashAssign => ZigElementType::SlashAssign,
            T::PercentAssign => ZigElementType::PercentAssign,
            T::AmpersandAssign => ZigElementType::AmpersandAssign,
            T::PipeAssign => ZigElementType::PipeAssign,
            T::CaretAssign => ZigElementType::CaretAssign,
            T::LessLessAssign => ZigElementType::LessLessAssign,
            T::GreaterGreaterAssign => ZigElementType::GreaterGreaterAssign,
            T::LeftParen => ZigElementType::LeftParen,
            T::RightParen => ZigElementType::RightParen,
            T::LeftBrace => ZigElementType::LeftBrace,
            T::RightBrace => ZigElementType::RightBrace,
            T::LeftBracket => ZigElementType::LeftBracket,
            T::RightBracket => ZigElementType::RightBracket,
            T::Semicolon => ZigElementType::Semicolon,
            T::Comma => ZigElementType::Comma,
            T::Dot => ZigElementType::Dot,
            T::DotDot => ZigElementType::DotDot,
            T::DotDotDot => ZigElementType::DotDotDot,
            T::DotQuestion => ZigElementType::DotQuestion,
            T::DotStar => ZigElementType::DotStar,
            T::Colon => ZigElementType::Colon,
            T::Question => ZigElementType::Question,
            T::Exclamation => ZigElementType::Exclamation,
            T::Arrow => ZigElementType::Arrow,
            T::FatArrow => ZigElementType::FatArrow,
            T::OrElse => ZigElementType::OrElse,
            T::CatchKeyword => ZigElementType::CatchKeyword,
            T::TryKeyword => ZigElementType::TryKeyword,
            T::AwaitKeyword => ZigElementType::AwaitKeyword,
            T::At => ZigElementType::At,
            T::BuiltinIdentifier => ZigElementType::BuiltinIdentifier,
            T::StringStart => ZigElementType::StringStart,
            T::StringEnd => ZigElementType::StringEnd,
            T::StringContent => ZigElementType::StringContent,
            T::InterpolationStart => ZigElementType::InterpolationStart,
            T::InterpolationEnd => ZigElementType::InterpolationEnd,
            T::MultilineStringStart => ZigElementType::MultilineStringStart,
            T::MultilineStringEnd => ZigElementType::MultilineStringEnd,
            T::MultilineStringContent => ZigElementType::MultilineStringContent,
            T::CompileDirective => ZigElementType::CompileDirective,
            T::Text => ZigElementType::Text,
            T::FnDeclaration => ZigElementType::FnDeclaration,
            T::VarDeclaration => ZigElementType::VarDeclaration,
            T::StructDeclaration => ZigElementType::StructDeclaration,
            T::EnumDeclaration => ZigElementType::EnumDeclaration,
            T::UnionDeclaration => ZigElementType::UnionDeclaration,
            T::Block => ZigElementType::Block,
            T::IfStatement => ZigElementType::IfStatement,
            T::WhileStatement => ZigElementType::WhileStatement,
            T::ForStatement => ZigElementType::ForStatement,
            T::ReturnStatement => ZigElementType::ReturnStatement,
        }
    }
}
