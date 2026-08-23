//! Python element types.

use oak_core::{ElementType, UniversalElementRole};

/// Python element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        match token {
            crate::lexer::token_type::PythonTokenType::Whitespace => PythonElementType::Whitespace,
            crate::lexer::token_type::PythonTokenType::Comment => PythonElementType::Comment,
            crate::lexer::token_type::PythonTokenType::Identifier => PythonElementType::Identifier,
            crate::lexer::token_type::PythonTokenType::Number => PythonElementType::Number,
            crate::lexer::token_type::PythonTokenType::String => PythonElementType::String,
            crate::lexer::token_type::PythonTokenType::Bytes => PythonElementType::Bytes,
            crate::lexer::token_type::PythonTokenType::FString => PythonElementType::FString,
            crate::lexer::token_type::PythonTokenType::AndKeyword => PythonElementType::AndKeyword,
            crate::lexer::token_type::PythonTokenType::AsKeyword => PythonElementType::AsKeyword,
            crate::lexer::token_type::PythonTokenType::AssertKeyword => PythonElementType::AssertKeyword,
            crate::lexer::token_type::PythonTokenType::AsyncKeyword => PythonElementType::AsyncKeyword,
            crate::lexer::token_type::PythonTokenType::AwaitKeyword => PythonElementType::AwaitKeyword,
            crate::lexer::token_type::PythonTokenType::BreakKeyword => PythonElementType::BreakKeyword,
            crate::lexer::token_type::PythonTokenType::ClassKeyword => PythonElementType::ClassKeyword,
            crate::lexer::token_type::PythonTokenType::ContinueKeyword => PythonElementType::ContinueKeyword,
            crate::lexer::token_type::PythonTokenType::DefKeyword => PythonElementType::DefKeyword,
            crate::lexer::token_type::PythonTokenType::DelKeyword => PythonElementType::DelKeyword,
            crate::lexer::token_type::PythonTokenType::ElifKeyword => PythonElementType::ElifKeyword,
            crate::lexer::token_type::PythonTokenType::ElseKeyword => PythonElementType::ElseKeyword,
            crate::lexer::token_type::PythonTokenType::ExceptKeyword => PythonElementType::ExceptKeyword,
            crate::lexer::token_type::PythonTokenType::FalseKeyword => PythonElementType::FalseKeyword,
            crate::lexer::token_type::PythonTokenType::FinallyKeyword => PythonElementType::FinallyKeyword,
            crate::lexer::token_type::PythonTokenType::ForKeyword => PythonElementType::ForKeyword,
            crate::lexer::token_type::PythonTokenType::FromKeyword => PythonElementType::FromKeyword,
            crate::lexer::token_type::PythonTokenType::GlobalKeyword => PythonElementType::GlobalKeyword,
            crate::lexer::token_type::PythonTokenType::IfKeyword => PythonElementType::IfKeyword,
            crate::lexer::token_type::PythonTokenType::ImportKeyword => PythonElementType::ImportKeyword,
            crate::lexer::token_type::PythonTokenType::InKeyword => PythonElementType::InKeyword,
            crate::lexer::token_type::PythonTokenType::IsKeyword => PythonElementType::IsKeyword,
            crate::lexer::token_type::PythonTokenType::LambdaKeyword => PythonElementType::LambdaKeyword,
            crate::lexer::token_type::PythonTokenType::NoneKeyword => PythonElementType::NoneKeyword,
            crate::lexer::token_type::PythonTokenType::NonlocalKeyword => PythonElementType::NonlocalKeyword,
            crate::lexer::token_type::PythonTokenType::NotKeyword => PythonElementType::NotKeyword,
            crate::lexer::token_type::PythonTokenType::OrKeyword => PythonElementType::OrKeyword,
            crate::lexer::token_type::PythonTokenType::PassKeyword => PythonElementType::PassKeyword,
            crate::lexer::token_type::PythonTokenType::RaiseKeyword => PythonElementType::RaiseKeyword,
            crate::lexer::token_type::PythonTokenType::ReturnKeyword => PythonElementType::ReturnKeyword,
            crate::lexer::token_type::PythonTokenType::TrueKeyword => PythonElementType::TrueKeyword,
            crate::lexer::token_type::PythonTokenType::TryKeyword => PythonElementType::TryKeyword,
            crate::lexer::token_type::PythonTokenType::WhileKeyword => PythonElementType::WhileKeyword,
            crate::lexer::token_type::PythonTokenType::WithKeyword => PythonElementType::WithKeyword,
            crate::lexer::token_type::PythonTokenType::YieldKeyword => PythonElementType::YieldKeyword,
            crate::lexer::token_type::PythonTokenType::Plus => PythonElementType::Plus,
            crate::lexer::token_type::PythonTokenType::Minus => PythonElementType::Minus,
            crate::lexer::token_type::PythonTokenType::Star => PythonElementType::Star,
            crate::lexer::token_type::PythonTokenType::DoubleStar => PythonElementType::DoubleStar,
            crate::lexer::token_type::PythonTokenType::Slash => PythonElementType::Slash,
            crate::lexer::token_type::PythonTokenType::DoubleSlash => PythonElementType::DoubleSlash,
            crate::lexer::token_type::PythonTokenType::Percent => PythonElementType::Percent,
            crate::lexer::token_type::PythonTokenType::At => PythonElementType::At,
            crate::lexer::token_type::PythonTokenType::LeftShift => PythonElementType::LeftShift,
            crate::lexer::token_type::PythonTokenType::RightShift => PythonElementType::RightShift,
            crate::lexer::token_type::PythonTokenType::Ampersand => PythonElementType::Ampersand,
            crate::lexer::token_type::PythonTokenType::Pipe => PythonElementType::Pipe,
            crate::lexer::token_type::PythonTokenType::Caret => PythonElementType::Caret,
            crate::lexer::token_type::PythonTokenType::Tilde => PythonElementType::Tilde,
            crate::lexer::token_type::PythonTokenType::Less => PythonElementType::Less,
            crate::lexer::token_type::PythonTokenType::Greater => PythonElementType::Greater,
            crate::lexer::token_type::PythonTokenType::LessEqual => PythonElementType::LessEqual,
            crate::lexer::token_type::PythonTokenType::GreaterEqual => PythonElementType::GreaterEqual,
            crate::lexer::token_type::PythonTokenType::Equal => PythonElementType::Equal,
            crate::lexer::token_type::PythonTokenType::NotEqual => PythonElementType::NotEqual,
            crate::lexer::token_type::PythonTokenType::Assign => PythonElementType::Assign,
            crate::lexer::token_type::PythonTokenType::PlusAssign => PythonElementType::PlusAssign,
            crate::lexer::token_type::PythonTokenType::MinusAssign => PythonElementType::MinusAssign,
            crate::lexer::token_type::PythonTokenType::StarAssign => PythonElementType::StarAssign,
            crate::lexer::token_type::PythonTokenType::DoubleStarAssign => PythonElementType::DoubleStarAssign,
            crate::lexer::token_type::PythonTokenType::SlashAssign => PythonElementType::SlashAssign,
            crate::lexer::token_type::PythonTokenType::DoubleSlashAssign => PythonElementType::DoubleSlashAssign,
            crate::lexer::token_type::PythonTokenType::PercentAssign => PythonElementType::PercentAssign,
            crate::lexer::token_type::PythonTokenType::AtAssign => PythonElementType::AtAssign,
            crate::lexer::token_type::PythonTokenType::AmpersandAssign => PythonElementType::AmpersandAssign,
            crate::lexer::token_type::PythonTokenType::PipeAssign => PythonElementType::PipeAssign,
            crate::lexer::token_type::PythonTokenType::CaretAssign => PythonElementType::CaretAssign,
            crate::lexer::token_type::PythonTokenType::LeftShiftAssign => PythonElementType::LeftShiftAssign,
            crate::lexer::token_type::PythonTokenType::RightShiftAssign => PythonElementType::RightShiftAssign,
            crate::lexer::token_type::PythonTokenType::LeftParen => PythonElementType::LeftParen,
            crate::lexer::token_type::PythonTokenType::RightParen => PythonElementType::RightParen,
            crate::lexer::token_type::PythonTokenType::LeftBracket => PythonElementType::LeftBracket,
            crate::lexer::token_type::PythonTokenType::RightBracket => PythonElementType::RightBracket,
            crate::lexer::token_type::PythonTokenType::LeftBrace => PythonElementType::LeftBrace,
            _ => PythonElementType::Error,
        }
    }
}
