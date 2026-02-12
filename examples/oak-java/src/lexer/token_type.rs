use oak_core::{Token, TokenType, UniversalTokenRole};

/// A token in the Java language.
pub type JavaToken = Token<JavaTokenType>;

/// Java token type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JavaTokenType {
    /// Whitespace.
    Whitespace,
    /// Line comment.
    LineComment,
    /// Block comment.
    BlockComment,
    /// Identifier.
    Identifier,
    /// String literal.
    StringLiteral,
    /// Character literal.
    CharacterLiteral,
    /// Integer literal.
    IntegerLiteral,
    /// Floating point literal.
    FloatingPointLiteral,
    /// Boolean literal.
    BooleanLiteral,
    /// Null literal.
    NullLiteral,

    // Keywords
    /// `abstract` keyword.
    Abstract,
    /// `assert` keyword.
    Assert,
    /// `boolean` keyword.
    Boolean,
    /// `break` keyword.
    Break,
    /// `byte` keyword.
    Byte,
    /// `case` keyword.
    Case,
    /// `catch` keyword.
    Catch,
    /// `char` keyword.
    Char,
    /// `class` keyword.
    Class,
    /// `const` keyword.
    Const,
    /// `continue` keyword.
    Continue,
    /// `default` keyword.
    Default,
    /// `do` keyword.
    Do,
    /// `double` keyword.
    Double,
    /// `else` keyword.
    Else,
    /// `enum` keyword.
    Enum,
    /// `extends` keyword.
    Extends,
    /// `final` keyword.
    Final,
    /// `finally` keyword.
    Finally,
    /// `float` keyword.
    Float,
    /// `for` keyword.
    For,
    /// `if` keyword.
    If,
    /// `goto` keyword.
    Goto,
    /// `implements` keyword.
    Implements,
    /// `import` keyword.
    Import,
    /// `instanceof` keyword.
    Instanceof,
    /// `int` keyword.
    Int,
    /// `interface` keyword.
    Interface,
    /// `long` keyword.
    Long,
    /// `native` keyword.
    Native,
    /// `new` keyword.
    New,
    /// `package` keyword.
    Package,
    /// `private` keyword.
    Private,
    /// `protected` keyword.
    Protected,
    /// `public` keyword.
    Public,
    /// `record` keyword.
    Record,
    /// `return` keyword.
    Return,
    /// `short` keyword.
    Short,
    /// `static` keyword.
    Static,
    /// `strictfp` keyword.
    Strictfp,
    /// `struct` keyword.
    Struct,
    /// `super` keyword.
    Super,
    /// `switch` keyword.
    Switch,
    /// `synchronized` keyword.
    Synchronized,
    /// `this` keyword.
    This,
    /// `throw` keyword.
    Throw,
    /// `throws` keyword.
    Throws,
    /// `transient` keyword.
    Transient,
    /// `try` keyword.
    Try,
    /// `void` keyword.
    Void,
    /// `volatile` keyword.
    Volatile,
    /// `while` keyword.
    While,

    // Operators and Delimiters
    /// `+` operator.
    Plus,
    /// `++` operator.
    PlusPlus,
    /// `+=` operator.
    PlusEquals,
    /// `-` operator.
    Minus,
    /// `--` operator.
    MinusMinus,
    /// `-=` operator.
    MinusEquals,
    /// `*` operator.
    Asterisk,
    /// `*=` operator.
    AsteriskEquals,
    /// `/` operator.
    Slash,
    /// `/=` operator.
    SlashEquals,
    /// `%` operator.
    Percent,
    /// `%=` operator.
    PercentEquals,
    /// `=` operator.
    Assign,
    /// `==` operator.
    Equals,
    /// `!` operator.
    Bang,
    /// `!=` operator.
    BangEquals,
    /// `<` operator.
    LessThan,
    /// `<=` operator.
    LessThanEquals,
    /// `<<` operator.
    LeftShift,
    /// `<<=` operator.
    LeftShiftEquals,
    /// `>` operator.
    GreaterThan,
    /// `>=` operator.
    GreaterThanEquals,
    /// `>>` operator.
    RightShift,
    /// `>>=` operator.
    RightShiftEquals,
    /// `>>>` operator.
    UnsignedRightShift,
    /// `>>>=` operator.
    UnsignedRightShiftEquals,
    /// `&` operator.
    Ampersand,
    /// `&&` operator.
    AmpersandAmpersand,
    /// `&=` operator.
    AmpersandEquals,
    /// `|` operator.
    Pipe,
    /// `||` operator.
    PipePipe,
    /// `|=` operator.
    PipeEquals,
    /// `^` operator.
    Caret,
    /// `^=` operator.
    CaretEquals,
    /// `~` operator.
    Tilde,
    /// `?` operator.
    Question,
    /// `:` delimiter.
    Colon,
    /// `;` delimiter.
    Semicolon,
    /// `,` delimiter.
    Comma,
    /// `.` delimiter.
    Dot,
    /// `...` delimiter.
    Ellipsis,
    /// `(` delimiter.
    LeftParen,
    /// `)` delimiter.
    RightParen,
    /// `{` delimiter.
    LeftBrace,
    /// `}` delimiter.
    RightBrace,
    /// `[` delimiter.
    LeftBracket,
    /// `]` delimiter.
    RightBracket,
    /// `@` delimiter.
    At,
    /// `::` delimiter.
    DoubleColon,

    /// Error token.
    Error,
    /// End of file token.
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
    /// Returns true if the token type is a Java keyword.
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
