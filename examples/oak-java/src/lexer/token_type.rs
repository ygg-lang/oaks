use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type JavaToken = Token<JavaTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JavaTokenType {
    Whitespace,
    LineComment,
    BlockComment,
    Identifier,
    StringLiteral,
    CharacterLiteral,
    IntegerLiteral,
    FloatingPointLiteral,
    BooleanLiteral,
    NullLiteral,

    // Keywords
    Abstract,
    Assert,
    Boolean,
    Break,
    Byte,
    Case,
    Catch,
    Char,
    Class,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extends,
    Final,
    Finally,
    Float,
    For,
    If,
    Goto,
    Implements,
    Import,
    Instanceof,
    Int,
    Interface,
    Long,
    Native,
    New,
    Package,
    Private,
    Protected,
    Public,
    Record,
    Return,
    Short,
    Static,
    Strictfp,
    Struct,
    Super,
    Switch,
    Synchronized,
    This,
    Throw,
    Throws,
    Transient,
    Try,
    Void,
    Volatile,
    While,

    // Operators and Delimiters
    Plus,
    PlusPlus,
    PlusEquals,
    Minus,
    MinusMinus,
    MinusEquals,
    Asterisk,
    AsteriskEquals,
    Slash,
    SlashEquals,
    Percent,
    PercentEquals,
    Assign,
    Equals,
    Bang,
    BangEquals,
    LessThan,
    LessThanEquals,
    LeftShift,
    LeftShiftEquals,
    GreaterThan,
    GreaterThanEquals,
    RightShift,
    RightShiftEquals,
    UnsignedRightShift,
    UnsignedRightShiftEquals,
    Ampersand,
    AmpersandAmpersand,
    AmpersandEquals,
    Pipe,
    PipePipe,
    PipeEquals,
    Caret,
    CaretEquals,
    Tilde,
    Question,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Ellipsis,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    At,
    DoubleColon,

    Error,
    EndOfFile,
}

impl TokenType for JavaTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::EndOfFile;

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            Self::Whitespace => Whitespace,
            Self::LineComment | Self::BlockComment => Comment,
            Self::Identifier => Name,
            Self::StringLiteral | Self::CharacterLiteral | Self::IntegerLiteral | Self::FloatingPointLiteral | Self::BooleanLiteral | Self::NullLiteral => Literal,
            _ if self.is_keyword() => Keyword,
            Self::Plus
            | Self::PlusPlus
            | Self::PlusEquals
            | Self::Minus
            | Self::MinusMinus
            | Self::MinusEquals
            | Self::Asterisk
            | Self::AsteriskEquals
            | Self::Slash
            | Self::SlashEquals
            | Self::Percent
            | Self::PercentEquals
            | Self::Assign
            | Self::Equals
            | Self::Bang
            | Self::BangEquals
            | Self::LessThan
            | Self::LessThanEquals
            | Self::LeftShift
            | Self::LeftShiftEquals
            | Self::GreaterThan
            | Self::GreaterThanEquals
            | Self::RightShift
            | Self::RightShiftEquals
            | Self::UnsignedRightShift
            | Self::UnsignedRightShiftEquals
            | Self::Ampersand
            | Self::AmpersandAmpersand
            | Self::AmpersandEquals
            | Self::Pipe
            | Self::PipePipe
            | Self::PipeEquals
            | Self::Caret
            | Self::CaretEquals
            | Self::Tilde => Operator,
            Self::Question | Self::Colon | Self::Semicolon | Self::Comma | Self::Dot | Self::Ellipsis | Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::At | Self::DoubleColon => {
                Punctuation
            }
            Self::Error => Error,
            Self::EndOfFile => Eof,
            _ if self.is_keyword() => Keyword,
            _ => None,
        }
    }
}

impl JavaTokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Abstract
                | Self::Assert
                | Self::Boolean
                | Self::Break
                | Self::Byte
                | Self::Case
                | Self::Catch
                | Self::Char
                | Self::Class
                | Self::Const
                | Self::Continue
                | Self::Default
                | Self::Do
                | Self::Double
                | Self::Else
                | Self::Enum
                | Self::Extends
                | Self::Final
                | Self::Finally
                | Self::Float
                | Self::For
                | Self::If
                | Self::Goto
                | Self::Implements
                | Self::Import
                | Self::Instanceof
                | Self::Int
                | Self::Interface
                | Self::Long
                | Self::Native
                | Self::New
                | Self::Package
                | Self::Private
                | Self::Protected
                | Self::Public
                | Self::Record
                | Self::Return
                | Self::Short
                | Self::Static
                | Self::Strictfp
                | Self::Struct
                | Self::Super
                | Self::Switch
                | Self::Synchronized
                | Self::This
                | Self::Throw
                | Self::Throws
                | Self::Transient
                | Self::Try
                | Self::Void
                | Self::Volatile
                | Self::While
        )
    }
}
