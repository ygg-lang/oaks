use oak_core::{TokenType, UniversalTokenRole};

/// Token types for Pascal.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PascalTokenType {
    /// Whitespace characters.
    Whitespace,
    /// Newline characters.
    Newline,

    /// Comments.
    Comment,

    /// `program` keyword.
    Program,
    /// `begin` keyword.
    Begin,
    /// `end` keyword.
    End,
    /// `var` keyword.
    Var,
    /// `const` keyword.
    Const,
    /// `type` keyword.
    Type,
    /// `function` keyword.
    Function,
    /// `procedure` keyword.
    Procedure,
    /// `if` keyword.
    If,
    /// `then` keyword.
    Then,
    /// `else` keyword.
    Else,
    /// `while` keyword.
    While,
    /// `do` keyword.
    Do,
    /// `for` keyword.
    For,
    /// `to` keyword.
    To,
    /// `downto` keyword.
    Downto,
    /// `repeat` keyword.
    Repeat,
    /// `until` keyword.
    Until,
    /// `case` keyword.
    Case,
    /// `of` keyword.
    Of,
    /// `with` keyword.
    With,
    /// `record` keyword.
    Record,
    /// `array` keyword.
    Array,
    /// `set` keyword.
    Set,
    /// `file` keyword.
    File,
    /// `packed` keyword.
    Packed,
    /// `nil` keyword.
    Nil,
    /// `true` keyword.
    True,
    /// `false` keyword.
    False,
    /// `and` keyword.
    And,
    /// `or` keyword.
    Or,
    /// `not` keyword.
    Not,
    /// `div` keyword.
    Div,
    /// `mod` keyword.
    Mod,
    /// `in` keyword.
    In,

    /// An identifier.
    Identifier,
    /// An integer literal.
    IntegerLiteral,
    /// A real number literal.
    RealLiteral,
    /// A string literal.
    StringLiteral,
    /// A character literal.
    CharLiteral,

    /// Plus operator `+`.
    Plus,
    /// Minus operator `-`.
    Minus,
    /// Multiply operator `*`.
    Multiply,
    /// Divide operator `/`.
    Divide,
    /// Assignment operator `:=`.
    Assign,
    /// Equality operator `=`.
    Equal,
    /// Inequality operator `<>`.
    NotEqual,
    /// Less than operator `<`.
    Less,
    /// Less than or equal operator `<=`.
    LessEqual,
    /// Greater than operator `>`.
    Greater,
    /// Greater than or equal operator `>=`.
    GreaterEqual,

    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Semicolon `;`.
    Semicolon,
    /// Comma `,`.
    Comma,
    /// Dot `.`.
    Dot,
    /// Colon `:`.
    Colon,
    /// Range operator `..`.
    Range,
    /// Caret operator `^`.
    Caret,

    /// Root node (Internal use).
    Root,
    /// Program block (Internal use).
    ProgramBlock,
    /// Variable section (Internal use).
    VarSection,
    /// Constant section (Internal use).
    ConstSection,
    /// Type section (Internal use).
    TypeSection,
    /// Procedure definition (Internal use).
    ProcedureDef,
    /// Function definition (Internal use).
    FunctionDef,
    /// Compound statement (Internal use).
    CompoundStmt,
    /// Expression (Internal use).
    Expression,

    /// Error token.
    Error,
    /// End of file.
    Eof,
}

/// Represents a token in a Pascal source file.
pub type PascalToken = Token<PascalTokenType>;

impl TokenType for PascalTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            Self::Whitespace | Self::Newline => Whitespace,
            Self::Comment => Comment,
            Self::Identifier => Name,
            Self::IntegerLiteral | Self::RealLiteral | Self::StringLiteral | Self::CharLiteral => Literal,
            Self::Program
            | Self::Begin
            | Self::End
            | Self::Var
            | Self::Const
            | Self::Type
            | Self::Function
            | Self::Procedure
            | Self::If
            | Self::Then
            | Self::Else
            | Self::While
            | Self::Do
            | Self::For
            | Self::To
            | Self::Downto
            | Self::Repeat
            | Self::Until
            | Self::Case
            | Self::Of
            | Self::With
            | Self::Record
            | Self::Array
            | Self::Set
            | Self::File
            | Self::Packed
            | Self::Nil
            | Self::True
            | Self::False
            | Self::And
            | Self::Or
            | Self::Not
            | Self::Div
            | Self::Mod
            | Self::In => UniversalTokenRole::Keyword,
            Self::Plus | Self::Minus | Self::Multiply | Self::Divide | Self::Assign | Self::Equal | Self::NotEqual | Self::Less | Self::LessEqual | Self::Greater | Self::GreaterEqual | Self::Caret => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Comma | Self::Dot | Self::Colon | Self::Range => UniversalTokenRole::Punctuation,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}
