use oak_core::{TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
/// Token types for the C++ language.
pub enum CppTokenType {
    // Trivia tokens
    /// Whitespace characters (spaces, tabs)
    Whitespace,
    /// Newline characters
    Newline,
    /// Comments (both single-line and multi-line)
    Comment,

    // Literals
    /// String literals (e.g., "hello")
    StringLiteral,
    /// Character literals (e.g., 'a')
    CharacterLiteral,
    /// Integer literals (e.g., 42, 0xFF)
    IntegerLiteral,
    /// Floating-point literals (e.g., 3.14, 2.5e10)
    FloatLiteral,
    /// Boolean literals (true, false)
    BooleanLiteral,

    // Identifiers and keywords
    /// Identifiers (variable names, function names, etc.)
    Identifier,
    /// Keywords (language reserved words)
    Keyword,

    // Operators
    /// Plus operator: +
    Plus,
    /// Minus operator: -
    Minus,
    /// Multiplication operator: *
    Star,
    /// Division operator: /
    Slash,
    /// Modulo operator: %
    Percent,
    /// Assignment operator: =
    Assign,
    /// Plus-assignment operator: +=
    PlusAssign,
    /// Minus-assignment operator: -=
    MinusAssign,
    /// Multiply-assignment operator: *=
    StarAssign,
    /// Divide-assignment operator: /=
    SlashAssign,
    /// Modulo-assignment operator: %=
    PercentAssign,
    /// Equality operator: ==
    Equal,
    /// Inequality operator: !=
    NotEqual,
    /// Less-than operator: <
    Less,
    /// Greater-than operator: >
    Greater,
    /// Less-than-or-equal operator: <=
    LessEqual,
    /// Greater-than-or-equal operator: >=
    GreaterEqual,
    /// Logical AND operator: &&
    LogicalAnd,
    /// Logical OR operator: ||
    LogicalOr,
    /// Logical NOT operator: !
    LogicalNot,
    /// Bitwise AND operator: &
    BitAnd,
    /// Bitwise OR operator: |
    BitOr,
    /// Bitwise XOR operator: ^
    BitXor,
    /// Bitwise NOT operator: ~
    BitNot,
    /// Left shift operator: <<
    LeftShift,
    /// Right shift operator: >>
    RightShift,
    /// AND-assignment operator: &=
    AndAssign,
    /// OR-assignment operator: |=
    OrAssign,
    /// XOR-assignment operator: ^=
    XorAssign,
    /// Left shift-assignment operator: <<=
    LeftShiftAssign,
    /// Right shift-assignment operator: >>=
    RightShiftAssign,
    /// Increment operator: ++
    Increment,
    /// Decrement operator: --
    Decrement,
    /// Arrow operator: ->
    Arrow,
    /// Scope resolution operator: ::
    Scope,
    /// Question mark: ?
    Question,
    /// Colon: :
    Colon,
    /// Semicolon: ;
    Semicolon,
    /// Comma: ,
    Comma,
    /// Dot operator: .
    Dot,
    /// Pointer-to-member operator: .*
    DotStar,
    /// Pointer-to-member arrow operator: ->*
    ArrowStar,

    // Punctuation
    /// Left parenthesis: (
    LeftParen,
    /// Right parenthesis: )
    RightParen,
    /// Left bracket: [
    LeftBracket,
    /// Right bracket: ]
    RightBracket,
    /// Left brace: {
    LeftBrace,
    /// Right brace: }
    RightBrace,

    // Preprocessor
    /// Preprocessor hash: #
    Hash,
    /// Double hash: ##
    DoubleHash,
    /// Preprocessor directive
    Preprocessor,

    // Special
    /// End of file
    EndOfFile,
    /// Unknown token
    Unknown,
    /// Error token
    Error,
}

impl TokenType for CppTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::EndOfFile;

    fn role(&self) -> UniversalTokenRole {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::StringLiteral | Self::CharacterLiteral | Self::IntegerLiteral | Self::FloatLiteral => UniversalTokenRole::Literal,
            Self::BooleanLiteral => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Keyword => UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Assign
            | Self::PlusAssign
            | Self::MinusAssign
            | Self::StarAssign
            | Self::SlashAssign
            | Self::PercentAssign
            | Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::LogicalAnd
            | Self::LogicalOr
            | Self::LogicalNot
            | Self::BitAnd
            | Self::BitOr
            | Self::BitXor
            | Self::BitNot
            | Self::LeftShift
            | Self::RightShift
            | Self::AndAssign
            | Self::OrAssign
            | Self::XorAssign
            | Self::LeftShiftAssign
            | Self::RightShiftAssign
            | Self::Increment
            | Self::Decrement
            | Self::Arrow
            | Self::Scope
            | Self::Question
            | Self::Colon
            | Self::Dot
            | Self::DotStar
            | Self::ArrowStar => UniversalTokenRole::Operator,
            Self::Semicolon | Self::Comma => UniversalTokenRole::Punctuation,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace => UniversalTokenRole::Punctuation,
            Self::Hash | Self::DoubleHash | Self::Preprocessor => UniversalTokenRole::Operator,
            Self::EndOfFile => UniversalTokenRole::Eof,
            Self::Unknown | Self::Error => UniversalTokenRole::Error,
        }
    }
}
