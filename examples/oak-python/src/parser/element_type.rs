//! Python element types.

use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Python element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum PythonElementType {
    /// Whitespace
    Whitespace,
    /// Comment
    Comment,
    /// Identifier
    Identifier,

    /// Number literal
    Number,
    /// String literal
    String,
    /// Bytes literal
    Bytes,
    /// Formatted string literal
    FString,

    /// `and`
    AndKeyword,
    /// `as`
    AsKeyword,
    /// `assert`
    AssertKeyword,
    /// `async`
    AsyncKeyword,
    /// `await`
    AwaitKeyword,
    /// `break`
    BreakKeyword,
    /// `class`
    ClassKeyword,
    /// `continue`
    ContinueKeyword,
    /// `def`
    DefKeyword,
    /// `del`
    DelKeyword,
    /// `elif`
    ElifKeyword,
    /// `else`
    ElseKeyword,
    /// `except`
    ExceptKeyword,
    /// `False`
    FalseKeyword,
    /// `finally`
    FinallyKeyword,
    /// `for`
    ForKeyword,
    /// `from`
    FromKeyword,
    /// `global`
    GlobalKeyword,
    /// `if`
    IfKeyword,
    /// `import`
    ImportKeyword,
    /// `in`
    InKeyword,
    /// `is`
    IsKeyword,
    /// `lambda`
    LambdaKeyword,
    /// `None`
    NoneKeyword,
    /// `nonlocal`
    NonlocalKeyword,
    /// `not`
    NotKeyword,
    /// `or`
    OrKeyword,
    /// `pass`
    PassKeyword,
    /// `raise`
    RaiseKeyword,
    /// `return`
    ReturnKeyword,
    /// `True`
    TrueKeyword,
    /// `try`
    TryKeyword,
    /// `while`
    WhileKeyword,
    /// `with`
    WithKeyword,
    /// `yield`
    YieldKeyword,

    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `**`
    DoubleStar,
    /// `/`
    Slash,
    /// `//`
    DoubleSlash,
    /// `%`
    Percent,
    /// `@`
    At,
    /// `<<`
    LeftShift,
    /// `>>`
    RightShift,
    /// `&`
    Ampersand,
    /// `|`
    Pipe,
    /// `^`
    Caret,
    /// `~`
    Tilde,
    /// `<`
    Less,
    /// `>`
    Greater,
    /// `<=`
    LessEqual,
    /// `>=`
    GreaterEqual,
    /// `==`
    Equal,
    /// `!=`
    NotEqual,

    /// `=`
    Assign,
    /// `+=`
    PlusAssign,
    /// `-=`
    MinusAssign,
    /// `*=`
    StarAssign,
    /// `**=`
    DoubleStarAssign,
    /// `/=`
    SlashAssign,
    /// `//=`
    DoubleSlashAssign,
    /// `%=`
    PercentAssign,
    /// `@=`
    AtAssign,
    /// `&=`
    AmpersandAssign,
    /// `|=`
    PipeAssign,
    /// `^=`
    CaretAssign,
    /// `<<=`
    LeftShiftAssign,
    /// `>>=`
    RightShiftAssign,

    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `;`
    Semicolon,
    /// `.`
    Dot,
    /// `->`
    Arrow,
    /// `...`
    Ellipsis,

    /// Newline
    Newline,
    /// Indent
    Indent,
    /// Dedent
    Dedent,
    /// End of stream
    Eof,
    /// Error node
    Error,

    /// Root node
    Root,
    /// Module
    Module,
    /// Interactive module
    InteractiveModule,
    /// Expression module
    ExpressionModule,

    /// Name expression
    Name,
    /// Constant expression
    Constant,
    /// Joined string expression
    JoinedStr,
    /// Expression
    Expr,
    /// Tuple expression
    Tuple,
    /// Generator expression
    GeneratorExp,
    /// List expression
    List,
    /// List comprehension
    ListComp,
    /// Dictionary expression
    Dict,
    /// Dictionary comprehension
    DictComp,
    /// Set comprehension
    SetComp,
    /// Set expression
    Set,
    /// Unary operation
    UnaryOp,
    /// Keyword argument
    Keyword,
    /// Starred expression
    Starred,
    /// Call expression
    Call,
    /// Slice expression
    Slice,
    /// Subscript expression
    Subscript,
    /// Attribute expression
    Attribute,
    /// Binary operation
    BinOp,
    /// Boolean operation
    BoolOp,
    /// Comparison expression
    Compare,
    /// If expression
    IfExp,
    /// Lambda expression
    Lambda,
    /// Yield expression
    Yield,
    /// Yield from expression
    YieldFrom,
    /// Named expression (walrus operator)
    NamedExpr,
    /// Formatted value expression
    FormattedValue,
    /// Await expression
    Await,

    /// Suite of statements
    Suite,
    /// Decorator
    Decorator,
    /// Assignment statement
    AssignStmt,
    /// With item
    WithItem,
    /// Return statement (keyword)
    Return,
    /// Return statement
    ReturnStmt,
    /// Pass statement (keyword)
    Pass,
    /// Pass statement
    PassStmt,
    /// Break statement (keyword)
    Break,
    /// Break statement
    BreakStmt,
    /// Continue statement (keyword)
    Continue,
    /// Continue statement
    ContinueStmt,
    /// Global statement (keyword)
    Global,
    /// Global statement
    GlobalStmt,
    /// Nonlocal statement (keyword)
    Nonlocal,
    /// Nonlocal statement
    NonlocalStmt,
    /// Assert statement (keyword)
    Assert,
    /// Assert statement
    AssertStmt,
    /// If statement (keyword)
    If,
    /// If statement
    IfStmt,
    /// While statement (keyword)
    While,
    /// While statement
    WhileStmt,
    /// For statement (keyword)
    For,
    /// For statement
    ForStmt,
    /// Async for statement
    AsyncFor,
    /// Try statement (keyword)
    Try,
    /// Try statement
    TryStmt,
    /// Except handler
    ExceptHandler,
    /// With statement (keyword)
    With,
    /// With statement
    WithStmt,
    /// Async with statement
    AsyncWith,
    /// Function definition
    FunctionDef,
    /// Async function definition
    AsyncFunctionDef,
    /// Class definition
    ClassDef,
    /// Import statement (keyword)
    Import,
    /// Import from statement (keyword)
    ImportFrom,
    /// Import statement
    ImportStmt,
    /// Import from statement
    ImportFromStmt,
    /// Expression statement
    ExprStmt,
    /// Delete statement (keyword)
    Delete,
    /// Delete statement
    DeleteStmt,
    /// Raise statement (keyword)
    Raise,
    /// Raise statement
    RaiseStmt,
    /// Arguments list
    Arguments,
    /// Single argument
    Arg,
    /// Import alias
    Alias,
    /// Comprehension
    Comprehension,
}

impl From<u16> for PythonElementType {
    fn from(d: u16) -> PythonElementType {
        unsafe { core::mem::transmute::<u16, PythonElementType>(d) }
    }
}

impl PythonElementType {
    /// Returns true if the element type is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::AndKeyword
                | Self::AsKeyword
                | Self::AssertKeyword
                | Self::AsyncKeyword
                | Self::AwaitKeyword
                | Self::BreakKeyword
                | Self::ClassKeyword
                | Self::ContinueKeyword
                | Self::DefKeyword
                | Self::DelKeyword
                | Self::ElifKeyword
                | Self::ElseKeyword
                | Self::ExceptKeyword
                | Self::FalseKeyword
                | Self::FinallyKeyword
                | Self::ForKeyword
                | Self::FromKeyword
                | Self::GlobalKeyword
                | Self::IfKeyword
                | Self::ImportKeyword
                | Self::InKeyword
                | Self::IsKeyword
                | Self::LambdaKeyword
                | Self::NoneKeyword
                | Self::NonlocalKeyword
                | Self::NotKeyword
                | Self::OrKeyword
                | Self::PassKeyword
                | Self::RaiseKeyword
                | Self::ReturnKeyword
                | Self::TrueKeyword
                | Self::TryKeyword
                | Self::WhileKeyword
                | Self::WithKeyword
                | Self::YieldKeyword
        )
    }
}

impl PythonElementType {
    /// Returns true if the element type is a trivia (whitespace or comment).
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl ElementType for PythonElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::PythonTokenType> for PythonElementType {
    fn from(token: crate::lexer::token_type::PythonTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
