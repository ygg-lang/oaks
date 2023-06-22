use oak_core::{TokenType, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
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
    /// Dot operator: .
    Dot,
    /// Ternary operator: ?
    Question,
    /// Colon operator: :
    Colon,
    /// Scope resolution operator: ::
    Scope,

    // Delimiters
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
    /// Comma: ,
    Comma,
    /// Semicolon: ;
    Semicolon,

    // Preprocessor
    /// Preprocessor directives (e.g., #include, #define)
    Preprocessor,

    Text,
    Error,
    Eof,
}

impl TokenType for CppTokenType {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

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
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
