//! Python token types.

use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Type alias for Python tokens.
pub type PythonToken = Token<PythonTokenType>;

impl PythonTokenType {
    /// Returns true if the token type is a keyword.
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

impl PythonTokenType {
    /// Returns true if the token type is a trivia (whitespace or comment).
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl TokenType for PythonTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

/// Python token types.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum PythonTokenType {
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
    /// Error token
    Error,
}

impl From<PythonTokenType> for u16 {
    fn from(k: PythonTokenType) -> u16 {
        k as u16
    }
}

impl From<u16> for PythonTokenType {
    fn from(d: u16) -> PythonTokenType {
        unsafe { core::mem::transmute::<u16, PythonTokenType>(d) }
    }
}
