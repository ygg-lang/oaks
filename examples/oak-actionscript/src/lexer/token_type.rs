use oak_core::{Token, TokenType, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// ActionScript 语言的标记
pub type ActionScriptToken = Token<ActionScriptTokenType>;

/// Represents the different types of tokens in the ActionScript language.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionScriptTokenType {
    /// Whitespace characters (spaces, tabs)
    Whitespace,
    /// Newline characters
    Newline,
    /// Comments (both single-line and multi-line)
    Comment,

    /// Identifiers (variable names, function names, etc.)
    Identifier,
    /// String literals (e.g., "hello")
    StringLiteral,
    /// Character literals (e.g., 'a')
    CharLiteral,
    /// Number literals (integer and floating-point)
    NumberLiteral,
    /// Boolean literals (true, false)
    BooleanLiteral,
    /// Null literal
    NullLiteral,

    // Keywords
    As,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Default,
    Delete,
    Do,
    Else,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    Implements,
    Import,
    In,
    Instanceof,
    Interface,
    Internal,
    Is,
    Native,
    New,
    Null,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Static,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Use,
    Var,
    Void,
    While,
    With,

    // AS3 specific keywords
    Each,
    Get,
    Set,
    Namespace,
    Include,
    Dynamic,
    Final,
    Override,

    // Built-in types
    Array,
    Boolean,
    Date,
    FunctionType,
    Number,
    ObjectType,
    RegExp,
    StringType,
    Uint,
    Vector,
    VoidType,
    Xml,
    XmlList,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    EqualEqualEqual,
    NotEqual,
    NotEqualEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    Increment,
    Decrement,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    Question,
    Colon,
    Dot,
    Arrow,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,

    // Symbols
    At,
    Hash,
    Dollar,
    Ampersand,
    Backslash,
    Quote,
    DoubleQuote,
    Backtick,

    /// End of file marker
    Eof,
}

impl TokenType for ActionScriptTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = ActionScriptTokenType::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral | Self::CharLiteral | Self::NumberLiteral | Self::BooleanLiteral | Self::NullLiteral => UniversalTokenRole::Literal,
            Self::As
            | Self::Break
            | Self::Case
            | Self::Catch
            | Self::Class
            | Self::Const
            | Self::Continue
            | Self::Default
            | Self::Delete
            | Self::Do
            | Self::Else
            | Self::Extends
            | Self::False
            | Self::Finally
            | Self::For
            | Self::Function
            | Self::If
            | Self::Implements
            | Self::Import
            | Self::In
            | Self::Instanceof
            | Self::Interface
            | Self::Internal
            | Self::Is
            | Self::Native
            | Self::New
            | Self::Null
            | Self::Package
            | Self::Private
            | Self::Protected
            | Self::Public
            | Self::Return
            | Self::Static
            | Self::Super
            | Self::Switch
            | Self::This
            | Self::Throw
            | Self::True
            | Self::Try
            | Self::Typeof
            | Self::Use
            | Self::Var
            | Self::Void
            | Self::While
            | Self::With
            | Self::Each
            | Self::Get
            | Self::Set
            | Self::Namespace
            | Self::Include
            | Self::Dynamic
            | Self::Final
            | Self::Override => UniversalTokenRole::Keyword,
            Self::Array | Self::Boolean | Self::Date | Self::FunctionType | Self::Number | Self::ObjectType | Self::RegExp | Self::StringType | Self::Uint | Self::Vector | Self::VoidType | Self::Xml | Self::XmlList => UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Equal
            | Self::EqualEqual
            | Self::EqualEqualEqual
            | Self::NotEqual
            | Self::NotEqualEqual
            | Self::LessThan
            | Self::LessEqual
            | Self::GreaterThan
            | Self::GreaterEqual
            | Self::LogicalAnd
            | Self::LogicalOr
            | Self::LogicalNot
            | Self::BitwiseAnd
            | Self::BitwiseOr
            | Self::BitwiseXor
            | Self::BitwiseNot
            | Self::LeftShift
            | Self::RightShift
            | Self::UnsignedRightShift
            | Self::Increment
            | Self::Decrement
            | Self::PlusAssign
            | Self::MinusAssign
            | Self::StarAssign
            | Self::SlashAssign
            | Self::PercentAssign
            | Self::LeftShiftAssign
            | Self::RightShiftAssign
            | Self::UnsignedRightShiftAssign
            | Self::BitwiseAndAssign
            | Self::BitwiseOrAssign
            | Self::BitwiseXorAssign
            | Self::Question
            | Self::Colon
            | Self::Dot
            | Self::Arrow => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Comma => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ActionScriptTokenType {
    /// Returns true if the token is a literal.
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::StringLiteral | Self::CharLiteral | Self::NumberLiteral | Self::BooleanLiteral | Self::NullLiteral)
    }
}
