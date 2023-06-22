use oak_core::{Token, TokenType, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// Rust 语言的标记
pub type RustToken = Token<RustTokenType>;

/// Represents the different types of tokens in the Rust language.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum RustTokenType {
    // Keywords
    As,
    Break,
    Const,
    Continue,
    Crate,
    Else,
    Enum,
    Extern,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Match,
    Mod,
    Move,
    Mut,
    Pub,
    Ref,
    Return,
    SelfLower,
    SelfUpper,
    Static,
    Struct,
    Super,
    Trait,
    True,
    Type,
    Unsafe,
    Use,
    Where,
    While,

    // Reserved keywords
    Abstract,
    Become,
    Box,
    Do,
    Final,
    Macro,
    Override,
    Priv,
    Typeof,
    Unsized,
    Virtual,
    Yield,

    // Weak keywords
    Async,
    Await,
    Dyn,
    Try,
    Union,
    Raw,

    // Literals
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    ByteLiteral,
    ByteStringLiteral,
    RawStringLiteral,
    BoolLiteral,

    // Identifiers
    Identifier,
    Lifetime,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    DotDot,
    DotDotDot,
    DotDotEq,
    Colon,
    DoubleColon,
    PathSep,
    Question,
    At,
    Hash,
    Dollar,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Ampersand,
    Pipe,
    Tilde,
    Bang,
    Eq,
    Lt,
    Gt,
    LessThan,
    GreaterThan,
    EqEq,
    Ne,
    Le,
    Ge,
    LessEq,
    GreaterEq,
    AndAnd,
    OrOr,
    LeftShift,
    RightShift,
    Shl,
    Shr,

    // Assignment operators
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    CaretEq,
    AndEq,
    OrEq,
    ShlEq,
    ShrEq,
    LeftShiftEq,
    RightShiftEq,

    // Assignment aliases
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    AmpAssign,
    PipeAssign,
    CaretAssign,
    ShlAssign,
    ShrAssign,

    // Special syntax
    Arrow,
    FatArrow,

    // Whitespace and comments
    Space,
    Newline,
    Whitespace,
    LineComment,
    BlockComment,
    DocComment,

    // Special tokens
    Error,
    PlusPlus,
    MinusMinus,
    Eof,
}

impl RustTokenType {
    /// 是否为关键字
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::As
                | Self::Break
                | Self::Const
                | Self::Continue
                | Self::Crate
                | Self::Else
                | Self::Enum
                | Self::Extern
                | Self::False
                | Self::Fn
                | Self::For
                | Self::If
                | Self::Impl
                | Self::In
                | Self::Let
                | Self::Loop
                | Self::Match
                | Self::Mod
                | Self::Move
                | Self::Mut
                | Self::Pub
                | Self::Ref
                | Self::Return
                | Self::SelfLower
                | Self::SelfUpper
                | Self::Static
                | Self::Struct
                | Self::Super
                | Self::Trait
                | Self::True
                | Self::Type
                | Self::Unsafe
                | Self::Use
                | Self::Where
                | Self::While
                | Self::Abstract
                | Self::Become
                | Self::Box
                | Self::Do
                | Self::Final
                | Self::Macro
                | Self::Override
                | Self::Priv
                | Self::Typeof
                | Self::Unsized
                | Self::Virtual
                | Self::Yield
                | Self::Async
                | Self::Await
                | Self::Dyn
                | Self::Try
                | Self::Union
                | Self::Raw
        )
    }

    /// 是否为字面量
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::ByteLiteral | Self::ByteStringLiteral | Self::RawStringLiteral | Self::BoolLiteral | Self::True | Self::False)
    }

    /// 是否为被忽略的标记（如空白或注释）
    pub fn is_ignored(&self) -> bool {
        self.is_whitespace() || self.is_comment()
    }
}

impl TokenType for RustTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment | Self::DocComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Space | Self::Newline)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            // Keywords
            _ if self.is_keyword() => Keyword,

            // Identifiers
            Self::Identifier | Self::Lifetime => Name,

            // Literals
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::RawStringLiteral | Self::ByteStringLiteral | Self::CharLiteral | Self::ByteLiteral | Self::BoolLiteral => Literal,

            // Punctuation
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
            | Self::DotDotEq
            | Self::Colon
            | Self::DoubleColon
            | Self::PathSep
            | Self::Arrow
            | Self::FatArrow
            | Self::Question
            | Self::At
            | Self::Hash
            | Self::Dollar => Punctuation,

            // Operators
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Caret
            | Self::Ampersand
            | Self::Pipe
            | Self::Tilde
            | Self::Bang
            | Self::Eq
            | Self::Ne
            | Self::Lt
            | Self::Gt
            | Self::Le
            | Self::Ge
            | Self::LessThan
            | Self::GreaterThan
            | Self::LessEq
            | Self::GreaterEq
            | Self::EqEq
            | Self::AndAnd
            | Self::OrOr
            | Self::Shl
            | Self::Shr
            | Self::LeftShift
            | Self::RightShift
            | Self::ShlEq
            | Self::ShrEq
            | Self::PlusEq
            | Self::MinusEq
            | Self::StarEq
            | Self::SlashEq
            | Self::PercentEq
            | Self::CaretEq
            | Self::AndEq
            | Self::OrEq
            | Self::LeftShiftEq
            | Self::RightShiftEq
            | Self::PlusPlus
            | Self::MinusMinus
            | Self::Assign
            | Self::PlusAssign
            | Self::MinusAssign
            | Self::StarAssign
            | Self::SlashAssign
            | Self::PercentAssign
            | Self::AmpAssign
            | Self::PipeAssign
            | Self::CaretAssign
            | Self::ShlAssign
            | Self::ShrAssign => Operator,

            // Comments
            Self::LineComment | Self::BlockComment | Self::DocComment => Comment,

            // Whitespace
            Self::Whitespace | Self::Space | Self::Newline => Whitespace,

            // Special
            Self::Error => Error,
            Self::Eof => Eof,
            _ => None,
        }
    }
}

impl std::fmt::Debug for RustTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Keywords
            Self::Fn => write!(f, "Keyword(Fn)"),
            Self::Let => write!(f, "Keyword(Let)"),
            Self::Mut => write!(f, "Keyword(Mut)"),
            Self::Const => write!(f, "Keyword(Const)"),
            Self::Static => write!(f, "Keyword(Static)"),
            Self::If => write!(f, "Keyword(If)"),
            Self::Else => write!(f, "Keyword(Else)"),
            Self::Match => write!(f, "Keyword(Match)"),
            Self::For => write!(f, "Keyword(For)"),
            Self::While => write!(f, "Keyword(While)"),
            Self::Loop => write!(f, "Keyword(Loop)"),
            Self::Break => write!(f, "Keyword(Break)"),
            Self::Continue => write!(f, "Keyword(Continue)"),
            Self::Return => write!(f, "Keyword(Return)"),
            Self::Struct => write!(f, "Keyword(Struct)"),
            Self::Enum => write!(f, "Keyword(Enum)"),
            Self::Union => write!(f, "Keyword(Union)"),
            Self::Trait => write!(f, "Keyword(Trait)"),
            Self::Impl => write!(f, "Keyword(Impl)"),
            Self::Mod => write!(f, "Keyword(Mod)"),
            Self::Use => write!(f, "Keyword(Use)"),
            Self::Pub => write!(f, "Keyword(Pub)"),
            Self::Crate => write!(f, "Keyword(Crate)"),
            Self::Super => write!(f, "Keyword(Super)"),
            Self::SelfLower => write!(f, "Keyword(Self)"),
            Self::SelfUpper => write!(f, "Keyword(SelfType)"),
            Self::Extern => write!(f, "Keyword(Extern)"),
            Self::Unsafe => write!(f, "Keyword(Unsafe)"),
            Self::Async => write!(f, "Keyword(Async)"),
            Self::Await => write!(f, "Keyword(Await)"),
            Self::Move => write!(f, "Keyword(Move)"),
            Self::Box => write!(f, "Keyword(Box)"),
            Self::Ref => write!(f, "Keyword(Ref)"),
            Self::In => write!(f, "Keyword(In)"),
            Self::Where => write!(f, "Keyword(Where)"),
            Self::As => write!(f, "Keyword(As)"),
            Self::Type => write!(f, "Keyword(Type)"),
            Self::Dyn => write!(f, "Keyword(Dyn)"),
            Self::True => write!(f, "Keyword(True)"),
            Self::False => write!(f, "Keyword(False)"),
            Self::Abstract => write!(f, "Keyword(Abstract)"),
            Self::Become => write!(f, "Keyword(Become)"),
            Self::Do => write!(f, "Keyword(Do)"),
            Self::Final => write!(f, "Keyword(Final)"),
            Self::Macro => write!(f, "Keyword(Macro)"),
            Self::Override => write!(f, "Keyword(Override)"),
            Self::Priv => write!(f, "Keyword(Priv)"),
            Self::Typeof => write!(f, "Keyword(Typeof)"),
            Self::Unsized => write!(f, "Keyword(Unsized)"),
            Self::Virtual => write!(f, "Keyword(Virtual)"),
            Self::Yield => write!(f, "Keyword(Yield)"),
            Self::Try => write!(f, "Keyword(Try)"),
            Self::Raw => write!(f, "Keyword(Raw)"),

            // Identifiers
            Self::Identifier => write!(f, "Identifier"),
            Self::Lifetime => write!(f, "Lifetime"),

            // Literals
            Self::IntegerLiteral => write!(f, "IntegerLiteral"),
            Self::FloatLiteral => write!(f, "FloatLiteral"),
            Self::StringLiteral => write!(f, "StringLiteral"),
            Self::RawStringLiteral => write!(f, "RawStringLiteral"),
            Self::ByteStringLiteral => write!(f, "ByteStringLiteral"),
            Self::CharLiteral => write!(f, "CharLiteral"),
            Self::ByteLiteral => write!(f, "ByteLiteral"),
            Self::BoolLiteral => write!(f, "BoolLiteral"),

            // Punctuation
            Self::LeftParen => write!(f, "LeftParen"),
            Self::RightParen => write!(f, "RightParen"),
            Self::LeftBrace => write!(f, "LeftBrace"),
            Self::RightBrace => write!(f, "RightBrace"),
            Self::LeftBracket => write!(f, "LeftBracket"),
            Self::RightBracket => write!(f, "RightBracket"),
            Self::Semicolon => write!(f, "Semicolon"),
            Self::Comma => write!(f, "Comma"),
            Self::Dot => write!(f, "Dot"),
            Self::DotDot => write!(f, "DotDot"),
            Self::DotDotDot => write!(f, "DotDotDot"),
            Self::DotDotEq => write!(f, "DotDotEq"),
            Self::Colon => write!(f, "Colon"),
            Self::DoubleColon => write!(f, "DoubleColon"),
            Self::PathSep => write!(f, "PathSep"),
            Self::Arrow => write!(f, "Arrow"),
            Self::FatArrow => write!(f, "FatArrow"),
            Self::Question => write!(f, "Question"),
            Self::At => write!(f, "At"),
            Self::Hash => write!(f, "Hash"),
            Self::Dollar => write!(f, "Dollar"),

            // Operators
            Self::Plus => write!(f, "Plus"),
            Self::Minus => write!(f, "Minus"),
            Self::Star => write!(f, "Star"),
            Self::Slash => write!(f, "Slash"),
            Self::Percent => write!(f, "Percent"),
            Self::Caret => write!(f, "Caret"),
            Self::Ampersand => write!(f, "Ampersand"),
            Self::Pipe => write!(f, "Pipe"),
            Self::Tilde => write!(f, "Tilde"),
            Self::Bang => write!(f, "Bang"),
            Self::Eq => write!(f, "Eq"),
            Self::Ne => write!(f, "Ne"),
            Self::Lt => write!(f, "Lt"),
            Self::Gt => write!(f, "Gt"),
            Self::Le => write!(f, "Le"),
            Self::Ge => write!(f, "Ge"),
            Self::LessThan => write!(f, "LessThan"),
            Self::GreaterThan => write!(f, "GreaterThan"),
            Self::LessEq => write!(f, "LessEq"),
            Self::GreaterEq => write!(f, "GreaterEq"),
            Self::EqEq => write!(f, "EqEq"),
            Self::AndAnd => write!(f, "AndAnd"),
            Self::OrOr => write!(f, "OrOr"),
            Self::Shl => write!(f, "Shl"),
            Self::Shr => write!(f, "Shr"),
            Self::LeftShift => write!(f, "LeftShift"),
            Self::RightShift => write!(f, "RightShift"),
            Self::ShlEq => write!(f, "ShlEq"),
            Self::ShrEq => write!(f, "ShrEq"),
            Self::PlusEq => write!(f, "PlusEq"),
            Self::MinusEq => write!(f, "MinusEq"),
            Self::StarEq => write!(f, "StarEq"),
            Self::SlashEq => write!(f, "SlashEq"),
            Self::PercentEq => write!(f, "PercentEq"),
            Self::CaretEq => write!(f, "CaretEq"),
            Self::AndEq => write!(f, "AndEq"),
            Self::OrEq => write!(f, "OrEq"),
            Self::LeftShiftEq => write!(f, "LeftShiftEq"),
            Self::RightShiftEq => write!(f, "RightShiftEq"),
            Self::PlusPlus => write!(f, "PlusPlus"),
            Self::MinusMinus => write!(f, "MinusMinus"),

            // Assignment aliases
            Self::Assign => write!(f, "Assign"),
            Self::PlusAssign => write!(f, "PlusAssign"),
            Self::MinusAssign => write!(f, "MinusAssign"),
            Self::StarAssign => write!(f, "StarAssign"),
            Self::SlashAssign => write!(f, "SlashAssign"),
            Self::PercentAssign => write!(f, "PercentAssign"),
            Self::AmpAssign => write!(f, "AmpAssign"),
            Self::PipeAssign => write!(f, "PipeAssign"),
            Self::CaretAssign => write!(f, "CaretAssign"),
            Self::ShlAssign => write!(f, "ShlAssign"),
            Self::ShrAssign => write!(f, "ShrAssign"),

            // Comments
            Self::LineComment => write!(f, "LineComment"),
            Self::BlockComment => write!(f, "BlockComment"),
            Self::DocComment => write!(f, "DocComment"),

            // Whitespace
            Self::Whitespace => write!(f, "Whitespace"),
            Self::Space => write!(f, "Space"),
            Self::Newline => write!(f, "Newline"),

            // Special tokens
            Self::Error => write!(f, "Error"),
            Self::Eof => write!(f, "Eof"),
        }
    }
}
