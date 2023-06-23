use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A token in the Rust programming language.
pub type RustToken = Token<RustTokenType>;

/// Rust token types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RustTokenType {
    /// `as`
    As,
    /// `break`
    Break,
    /// `const`
    Const,
    /// `continue`
    Continue,
    /// `crate`
    Crate,
    /// `else`
    Else,
    /// `enum`
    Enum,
    /// `extern`
    Extern,
    /// `false`
    False,
    /// `fn`
    Fn,
    /// `for`
    For,
    /// `if`
    If,
    /// `impl`
    Impl,
    /// `in`
    In,
    /// `let`
    Let,
    /// `loop`
    Loop,
    /// `match`
    Match,
    /// `mod`
    Mod,
    /// `move`
    Move,
    /// `mut`
    Mut,
    /// `pub`
    Pub,
    /// `ref`
    Ref,
    /// `return`
    Return,
    /// `self`
    SelfLower,
    /// `Self`
    SelfUpper,
    /// `static`
    Static,
    /// `struct`
    Struct,
    /// `super`
    Super,
    /// `trait`
    Trait,
    /// `true`
    True,
    /// `type`
    Type,
    /// `unsafe`
    Unsafe,
    /// `use`
    Use,
    /// `where`
    Where,
    /// `while`
    While,
    /// `abstract`
    Abstract,
    /// `become`
    Become,
    /// `box`
    Box,
    /// `do`
    Do,
    /// `final`
    Final,
    /// `macro`
    Macro,
    /// `override`
    Override,
    /// `priv`
    Priv,
    /// `typeof`
    Typeof,
    /// `unsized`
    Unsized,
    /// `virtual`
    Virtual,
    /// `yield`
    Yield,
    /// `async`
    Async,
    /// `await`
    Await,
    /// `dyn`
    Dyn,
    /// `try`
    Try,
    /// `union`
    Union,
    /// `raw`
    Raw,
    /// Integer literal
    IntegerLiteral,
    /// Float literal
    FloatLiteral,
    /// String literal
    StringLiteral,
    /// Char literal
    CharLiteral,
    /// Byte literal
    ByteLiteral,
    /// Byte string literal
    ByteStringLiteral,
    /// Raw string literal
    RawStringLiteral,
    /// Bool literal
    BoolLiteral,
    /// Identifier
    Identifier,
    /// Lifetime
    Lifetime,
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `;`
    Semicolon,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `..`
    DotDot,
    /// `...`
    DotDotDot,
    /// `..=`
    DotDotEq,
    /// `:`
    Colon,
    /// `::`
    DoubleColon,
    /// Path separator
    PathSep,
    /// `?`
    Question,
    /// `@`
    At,
    /// `#`
    Hash,
    /// `$`
    Dollar,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `^`
    Caret,
    /// `&`
    Ampersand,
    /// `|`
    Pipe,
    /// `~`
    Tilde,
    /// `!`
    Bang,
    /// `=`
    Eq,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `<`
    LessThan,
    /// `>`
    GreaterThan,
    /// `==`
    EqEq,
    /// `!=`
    Ne,
    /// `<=`
    Le,
    /// `>=`
    Ge,
    /// `<=`
    LessEq,
    /// `>=`
    GreaterEq,
    /// `&&`
    AndAnd,
    /// `||`
    OrOr,
    /// `<<`
    LeftShift,
    /// `>>`
    RightShift,
    /// `<<`
    Shl,
    /// `>>`
    Shr,
    /// `+=`
    PlusEq,
    /// `-=`
    MinusEq,
    /// `*=`
    StarEq,
    /// `/=`
    SlashEq,
    /// `%=`
    PercentEq,
    /// `^=`
    CaretEq,
    /// `&=`
    AndEq,
    /// `|=`
    OrEq,
    /// `<<=`
    ShlEq,
    /// `>>=`
    ShrEq,
    /// `<<=`
    LeftShiftEq,
    /// `>>=`
    RightShiftEq,
    /// `=`
    Assign,
    /// `+=`
    PlusAssign,
    /// `-=`
    MinusAssign,
    /// `*=`
    StarAssign,
    /// `/=`
    SlashAssign,
    /// `%=`
    PercentAssign,
    /// `&=`
    AmpAssign,
    /// `|=`
    PipeAssign,
    /// `^=`
    CaretAssign,
    /// `<<=`
    ShlAssign,
    /// `>>=`
    ShrAssign,
    /// `->`
    Arrow,
    /// `=>`
    FatArrow,
    /// Space
    Space,
    /// Newline
    Newline,
    /// Whitespace
    Whitespace,
    /// Line comment
    LineComment,
    /// Block comment
    BlockComment,
    /// Doc comment
    DocComment,
    /// `++`
    PlusPlus,
    /// `--`
    MinusMinus,
    /// End of stream
    Eof,
    /// Error token
    Error,
}

impl RustTokenType {
    /// Returns `true` if the token is a literal.
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::ByteLiteral | Self::ByteStringLiteral | Self::RawStringLiteral | Self::BoolLiteral | Self::True | Self::False)
    }
}

impl TokenType for RustTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment => UniversalTokenRole::Comment,
            Self::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
