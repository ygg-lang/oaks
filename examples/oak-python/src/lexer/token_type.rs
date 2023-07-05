//! Python token types.

use oak_core::{Token, TokenType, UniversalTokenRole};

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
        self.is_trivia()
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

/// Python token types.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        match d {
            x if x == PythonTokenType::Whitespace as u16 => PythonTokenType::Whitespace,
            x if x == PythonTokenType::Comment as u16 => PythonTokenType::Comment,
            x if x == PythonTokenType::Identifier as u16 => PythonTokenType::Identifier,
            x if x == PythonTokenType::Number as u16 => PythonTokenType::Number,
            x if x == PythonTokenType::String as u16 => PythonTokenType::String,
            x if x == PythonTokenType::Bytes as u16 => PythonTokenType::Bytes,
            x if x == PythonTokenType::FString as u16 => PythonTokenType::FString,
            x if x == PythonTokenType::AndKeyword as u16 => PythonTokenType::AndKeyword,
            x if x == PythonTokenType::AsKeyword as u16 => PythonTokenType::AsKeyword,
            x if x == PythonTokenType::AssertKeyword as u16 => PythonTokenType::AssertKeyword,
            x if x == PythonTokenType::AsyncKeyword as u16 => PythonTokenType::AsyncKeyword,
            x if x == PythonTokenType::AwaitKeyword as u16 => PythonTokenType::AwaitKeyword,
            x if x == PythonTokenType::BreakKeyword as u16 => PythonTokenType::BreakKeyword,
            x if x == PythonTokenType::ClassKeyword as u16 => PythonTokenType::ClassKeyword,
            x if x == PythonTokenType::ContinueKeyword as u16 => PythonTokenType::ContinueKeyword,
            x if x == PythonTokenType::DefKeyword as u16 => PythonTokenType::DefKeyword,
            x if x == PythonTokenType::DelKeyword as u16 => PythonTokenType::DelKeyword,
            x if x == PythonTokenType::ElifKeyword as u16 => PythonTokenType::ElifKeyword,
            x if x == PythonTokenType::ElseKeyword as u16 => PythonTokenType::ElseKeyword,
            x if x == PythonTokenType::ExceptKeyword as u16 => PythonTokenType::ExceptKeyword,
            x if x == PythonTokenType::FalseKeyword as u16 => PythonTokenType::FalseKeyword,
            x if x == PythonTokenType::FinallyKeyword as u16 => PythonTokenType::FinallyKeyword,
            x if x == PythonTokenType::ForKeyword as u16 => PythonTokenType::ForKeyword,
            x if x == PythonTokenType::FromKeyword as u16 => PythonTokenType::FromKeyword,
            x if x == PythonTokenType::GlobalKeyword as u16 => PythonTokenType::GlobalKeyword,
            x if x == PythonTokenType::IfKeyword as u16 => PythonTokenType::IfKeyword,
            x if x == PythonTokenType::ImportKeyword as u16 => PythonTokenType::ImportKeyword,
            x if x == PythonTokenType::InKeyword as u16 => PythonTokenType::InKeyword,
            x if x == PythonTokenType::IsKeyword as u16 => PythonTokenType::IsKeyword,
            x if x == PythonTokenType::LambdaKeyword as u16 => PythonTokenType::LambdaKeyword,
            x if x == PythonTokenType::NoneKeyword as u16 => PythonTokenType::NoneKeyword,
            x if x == PythonTokenType::NonlocalKeyword as u16 => PythonTokenType::NonlocalKeyword,
            x if x == PythonTokenType::NotKeyword as u16 => PythonTokenType::NotKeyword,
            x if x == PythonTokenType::OrKeyword as u16 => PythonTokenType::OrKeyword,
            x if x == PythonTokenType::PassKeyword as u16 => PythonTokenType::PassKeyword,
            x if x == PythonTokenType::RaiseKeyword as u16 => PythonTokenType::RaiseKeyword,
            x if x == PythonTokenType::ReturnKeyword as u16 => PythonTokenType::ReturnKeyword,
            x if x == PythonTokenType::TrueKeyword as u16 => PythonTokenType::TrueKeyword,
            x if x == PythonTokenType::TryKeyword as u16 => PythonTokenType::TryKeyword,
            x if x == PythonTokenType::WhileKeyword as u16 => PythonTokenType::WhileKeyword,
            x if x == PythonTokenType::WithKeyword as u16 => PythonTokenType::WithKeyword,
            x if x == PythonTokenType::YieldKeyword as u16 => PythonTokenType::YieldKeyword,
            x if x == PythonTokenType::Plus as u16 => PythonTokenType::Plus,
            x if x == PythonTokenType::Minus as u16 => PythonTokenType::Minus,
            x if x == PythonTokenType::Star as u16 => PythonTokenType::Star,
            x if x == PythonTokenType::DoubleStar as u16 => PythonTokenType::DoubleStar,
            x if x == PythonTokenType::Slash as u16 => PythonTokenType::Slash,
            x if x == PythonTokenType::DoubleSlash as u16 => PythonTokenType::DoubleSlash,
            x if x == PythonTokenType::Percent as u16 => PythonTokenType::Percent,
            x if x == PythonTokenType::At as u16 => PythonTokenType::At,
            x if x == PythonTokenType::LeftShift as u16 => PythonTokenType::LeftShift,
            x if x == PythonTokenType::RightShift as u16 => PythonTokenType::RightShift,
            x if x == PythonTokenType::Ampersand as u16 => PythonTokenType::Ampersand,
            x if x == PythonTokenType::Pipe as u16 => PythonTokenType::Pipe,
            x if x == PythonTokenType::Caret as u16 => PythonTokenType::Caret,
            x if x == PythonTokenType::Tilde as u16 => PythonTokenType::Tilde,
            x if x == PythonTokenType::Less as u16 => PythonTokenType::Less,
            x if x == PythonTokenType::Greater as u16 => PythonTokenType::Greater,
            x if x == PythonTokenType::LessEqual as u16 => PythonTokenType::LessEqual,
            x if x == PythonTokenType::GreaterEqual as u16 => PythonTokenType::GreaterEqual,
            x if x == PythonTokenType::Equal as u16 => PythonTokenType::Equal,
            x if x == PythonTokenType::NotEqual as u16 => PythonTokenType::NotEqual,
            x if x == PythonTokenType::Assign as u16 => PythonTokenType::Assign,
            x if x == PythonTokenType::PlusAssign as u16 => PythonTokenType::PlusAssign,
            x if x == PythonTokenType::MinusAssign as u16 => PythonTokenType::MinusAssign,
            x if x == PythonTokenType::StarAssign as u16 => PythonTokenType::StarAssign,
            x if x == PythonTokenType::DoubleStarAssign as u16 => PythonTokenType::DoubleStarAssign,
            x if x == PythonTokenType::SlashAssign as u16 => PythonTokenType::SlashAssign,
            x if x == PythonTokenType::DoubleSlashAssign as u16 => PythonTokenType::DoubleSlashAssign,
            x if x == PythonTokenType::PercentAssign as u16 => PythonTokenType::PercentAssign,
            x if x == PythonTokenType::AtAssign as u16 => PythonTokenType::AtAssign,
            x if x == PythonTokenType::AmpersandAssign as u16 => PythonTokenType::AmpersandAssign,
            x if x == PythonTokenType::PipeAssign as u16 => PythonTokenType::PipeAssign,
            x if x == PythonTokenType::CaretAssign as u16 => PythonTokenType::CaretAssign,
            x if x == PythonTokenType::LeftShiftAssign as u16 => PythonTokenType::LeftShiftAssign,
            x if x == PythonTokenType::RightShiftAssign as u16 => PythonTokenType::RightShiftAssign,
            x if x == PythonTokenType::LeftParen as u16 => PythonTokenType::LeftParen,
            x if x == PythonTokenType::RightParen as u16 => PythonTokenType::RightParen,
            x if x == PythonTokenType::LeftBracket as u16 => PythonTokenType::LeftBracket,
            x if x == PythonTokenType::RightBracket as u16 => PythonTokenType::RightBracket,
            x if x == PythonTokenType::LeftBrace as u16 => PythonTokenType::LeftBrace,
            _ => PythonTokenType::Whitespace,
        }
    }
}
