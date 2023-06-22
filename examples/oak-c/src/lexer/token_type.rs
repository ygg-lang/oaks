use oak_core::{TokenType, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum CTokenType {
    /// Left parenthesis: `(`
    LeftParen,
    /// Right parenthesis: `)`
    RightParen,
    /// Left bracket: `[`
    LeftBracket,
    /// Right bracket: `]`
    RightBracket,
    /// Left brace: `{`
    LeftBrace,
    /// Right brace: `}`
    RightBrace,
    /// Comma: `,`
    Comma,
    /// Semicolon: `;`
    Semicolon,
    /// Colon: `:`
    Colon,
    /// Dot: `.`
    Dot,
    /// Question mark: `?`
    Question,
    /// Addition operator: `+`
    Plus,
    /// Subtraction operator: `-`
    Minus,
    /// Multiplication operator: `*`
    Star,
    /// Division operator: `/`
    Slash,
    /// Modulo operator: `%`
    Percent,
    /// Assignment operator: `=`
    Assign,
    /// Addition assignment: `+=`
    PlusAssign,
    /// Subtraction assignment: `-=`
    MinusAssign,
    /// Multiplication assignment: `*=`
    StarAssign,
    /// Division assignment: `/=`
    SlashAssign,
    /// Modulo assignment: `%=`
    PercentAssign,
    /// Equality comparison: `==`
    Equal,
    /// Inequality comparison: `!=`
    NotEqual,
    /// Less than: `<`
    Less,
    /// Greater than: `>`
    Greater,
    /// Less than or equal: `<=`
    LessEqual,
    /// Greater than or equal: `>=`
    GreaterEqual,
    /// Logical AND: `&&`
    LogicalAnd,
    /// Logical OR: `||`
    LogicalOr,
    /// Logical NOT: `!`
    LogicalNot,
    /// Bitwise AND: `&`
    BitAnd,
    /// Bitwise OR: `|`
    BitOr,
    /// Bitwise XOR: `^`
    BitXor,
    /// Bitwise NOT: `~`
    BitNot,
    /// Left shift: `<<`
    LeftShift,
    /// Right shift: `>>`
    RightShift,
    /// Bitwise AND assignment: `&=`
    AndAssign,
    /// Bitwise OR assignment: `|=`
    OrAssign,
    /// Bitwise XOR assignment: `^=`
    XorAssign,
    /// Left shift assignment: `<<=`
    LeftShiftAssign,
    /// Right shift assignment: `>>=`
    RightShiftAssign,
    /// Increment: `++`
    Increment,
    /// Decrement: `--`
    Decrement,
    /// Arrow operator: `->`
    Arrow,
    /// `auto` keyword
    Auto,
    /// `register` keyword
    Register,
    /// `static` keyword
    Static,
    /// `extern` keyword
    Extern,
    /// `typedef` keyword
    Typedef,
    /// `void` keyword
    Void,
    /// `char` keyword
    Char,
    /// `short` keyword
    Short,
    /// `int` keyword
    Int,
    /// `long` keyword
    Long,
    /// `float` keyword
    Float,
    /// `double` keyword
    Double,
    /// `signed` keyword
    Signed,
    /// `unsigned` keyword
    Unsigned,
    /// `struct` keyword
    Struct,
    /// `union` keyword
    Union,
    /// `enum` keyword
    Enum,
    /// `const` keyword
    Const,
    /// `volatile` keyword
    Volatile,
    /// `restrict` keyword
    Restrict,
    /// `if` keyword
    If,
    /// `else` keyword
    Else,
    /// `switch` keyword
    Switch,
    /// `case` keyword
    Case,
    /// `default` keyword
    Default,
    /// `for` keyword
    For,
    /// `while` keyword
    While,
    /// `do` keyword
    Do,
    /// `break` keyword
    Break,
    /// `continue` keyword
    Continue,
    /// `goto` keyword
    Goto,
    /// `return` keyword
    Return,
    /// `sizeof` keyword
    Sizeof,
    /// `inline` keyword
    Inline,
    /// `_Bool` keyword
    Bool,
    /// `_Complex` keyword
    Complex,
    /// `_Imaginary` keyword
    Imaginary,
    /// `_Alignas` keyword
    Alignas,
    /// `_Alignof` keyword
    Alignof,
    /// `_Atomic` keyword
    Atomic,
    /// `_Static_assert` keyword
    StaticAssert,
    /// `_Thread_local` keyword
    ThreadLocal,
    /// `_Generic` keyword
    Generic,
    /// `_Noreturn` keyword
    Noreturn,
    /// Integer literal
    IntegerLiteral,
    /// Floating-point literal
    FloatLiteral,
    /// Character literal
    CharLiteral,
    /// String literal
    StringLiteral,
    /// Identifier
    Identifier,
    /// Whitespace
    Whitespace,
    /// Comments
    Comment,
    /// Preprocessor directives
    PreprocessorDirective,
    /// Text
    Text,
    /// Error token
    Error,
    /// End of file marker
    Eof,
}

impl CTokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Auto
                | Self::Register
                | Self::Static
                | Self::Extern
                | Self::Typedef
                | Self::Void
                | Self::Char
                | Self::Short
                | Self::Int
                | Self::Long
                | Self::Float
                | Self::Double
                | Self::Signed
                | Self::Unsigned
                | Self::Struct
                | Self::Union
                | Self::Enum
                | Self::Const
                | Self::Volatile
                | Self::Restrict
                | Self::If
                | Self::Else
                | Self::Switch
                | Self::Case
                | Self::Default
                | Self::For
                | Self::While
                | Self::Do
                | Self::Break
                | Self::Continue
                | Self::Goto
                | Self::Return
                | Self::Sizeof
                | Self::Inline
                | Self::Bool
                | Self::Complex
                | Self::Imaginary
                | Self::Alignas
                | Self::Alignof
                | Self::Atomic
                | Self::StaticAssert
                | Self::ThreadLocal
                | Self::Generic
                | Self::Noreturn
        )
    }
}

impl TokenType for CTokenType {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            _ if self.is_keyword() => Keyword,
            Self::Identifier => Name,
            Self::IntegerLiteral | Self::FloatLiteral | Self::CharLiteral | Self::StringLiteral => Literal,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Comma | Self::Semicolon | Self::Colon | Self::Dot | Self::Question => Punctuation,
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
            | Self::Arrow => Operator,
            Self::Comment => Comment,
            Self::Whitespace => Whitespace,
            Self::Error => Error,
            Self::Eof => Eof,
            _ => None,
        }
    }
}
