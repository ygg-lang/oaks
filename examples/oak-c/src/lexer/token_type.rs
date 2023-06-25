use oak_core::{TokenType, UniversalTokenRole};

/// Represents different types of tokens in the C language.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    /// `break` keyword
    Break,
    /// `case` keyword
    Case,
    /// `char` keyword
    Char,
    /// `const` keyword
    Const,
    /// `continue` keyword
    Continue,
    /// `default` keyword
    Default,
    /// `do` keyword
    Do,
    /// `double` keyword
    Double,
    /// `else` keyword
    Else,
    /// `enum` keyword
    Enum,
    /// `extern` keyword
    Extern,
    /// `float` keyword
    Float,
    /// `for` keyword
    For,
    /// `goto` keyword
    Goto,
    /// `if` keyword
    If,
    /// `inline` keyword
    Inline,
    /// `int` keyword
    Int,
    /// `long` keyword
    Long,
    /// `register` keyword
    Register,
    /// `restrict` keyword
    Restrict,
    /// `return` keyword
    Return,
    /// `short` keyword
    Short,
    /// `signed` keyword
    Signed,
    /// `sizeof` keyword
    Sizeof,
    /// `static` keyword
    Static,
    /// `struct` keyword
    Struct,
    /// `switch` keyword
    Switch,
    /// `typedef` keyword
    Typedef,
    /// `union` keyword
    Union,
    /// `unsigned` keyword
    Unsigned,
    /// `void` keyword
    Void,
    /// `volatile` keyword
    Volatile,
    /// `while` keyword
    While,
    /// `_Alignas` keyword
    Alignas,
    /// `_Alignof` keyword
    Alignof,
    /// `_Atomic` keyword
    Atomic,
    /// `_Bool` keyword
    Bool,
    /// `_Complex` keyword
    Complex,
    /// `_Generic` keyword
    Generic,
    /// `_Imaginary` keyword
    Imaginary,
    /// `_Noreturn` keyword
    Noreturn,
    /// `_Static_assert` keyword
    StaticAssert,
    /// `_Thread_local` keyword
    ThreadLocal,
    /// Identifier
    Identifier,
    /// String literal
    StringLiteral,
    /// Character constant
    CharConstant,
    /// Integer constant
    IntConstant,
    /// Floating-point constant
    FloatConstant,
    /// Whitespace
    Whitespace,
    /// Single-line comment
    LineComment,
    /// Multi-line comment
    BlockComment,
    /// Preprocessor directive
    Preprocessor,
    /// Raw text or unrecognized sequence
    Text,
    /// Invalid token
    #[default]
    Error,
    /// End of stream
    Eof,
}

impl CTokenType {
    /// Returns true if the token is a keyword.
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
    /// The end of stream token.
    const END_OF_STREAM: Self = Self::Eof;
    /// The token role type.
    type Role = UniversalTokenRole;

    /// Returns true if the token should be ignored during parsing.
    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::LineComment | Self::BlockComment)
    }

    /// Returns true if the token is a comment.
    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
    }

    /// Returns true if the token is whitespace.
    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    /// Returns the role of the token.
    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            _ if self.is_keyword() => Keyword,
            Self::Identifier => Name,
            Self::IntConstant | Self::FloatConstant | Self::CharConstant | Self::StringLiteral => Literal,
            Self::Preprocessor => Keyword,
            Self::Text => None,
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
            Self::LineComment | Self::BlockComment => Comment,
            Self::Whitespace => Whitespace,
            Self::Error => Error,
            Self::Eof => Eof,
            _ => None,
        }
    }
}
