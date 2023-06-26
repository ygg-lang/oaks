use oak_core::UniversalTokenRole;

/// Token types for the Mojo language lexer.
///
/// This enum represents all possible token types in Mojo,
/// including keywords, identifiers, literals, operators, and delimiters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MojoTokenType {
    /// Function keyword `fn`.
    Fn,
    /// Struct keyword `struct`.
    Struct,
    /// Variable keyword `var`.
    Var,
    /// Let keyword `let`.
    Let,
    /// If keyword `if`.
    If,
    /// Else keyword `else`.
    Else,
    /// While keyword `while`.
    While,
    /// For keyword `for`.
    For,
    /// In keyword `in`.
    In,
    /// Return keyword `return`.
    Return,
    /// Break keyword `break`.
    Break,
    /// Continue keyword `continue`.
    Continue,
    /// Import keyword `import`.
    Import,
    /// From keyword `from`.
    From,
    /// Boolean literal `True`.
    True,
    /// Boolean literal `False`.
    False,
    /// None literal.
    None,

    /// Identifier token.
    Identifier,
    /// Integer literal token.
    Integer,
    /// Float literal token.
    Float,
    /// String literal token.
    String,

    /// Plus operator `+`.
    Plus,
    /// Minus operator `-`.
    Minus,
    /// Star operator `*`.
    Star,
    /// Slash operator `/`.
    Slash,
    /// Percent operator `%`.
    Percent,
    /// Assignment operator `=`.
    Equal,
    /// Equality operator `==`.
    EqualEqual,
    /// Inequality operator `!=`.
    NotEqual,
    /// Less than operator `<`.
    Less,
    /// Less than or equal operator `<=`.
    LessEqual,
    /// Greater than operator `>`.
    Greater,
    /// Greater than or equal operator `>=`.
    GreaterEqual,
    /// Logical and operator `and`.
    And,
    /// Logical or operator `or`.
    Or,
    /// Logical not operator `not`.
    Not,
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Comma `,`.
    Comma,
    /// Dot `.`.
    Dot,
    /// Colon `:`.
    Colon,
    /// Semicolon `;`.
    Semicolon,
    /// Arrow operator `->`.
    Arrow,

    /// Whitespace token.
    Whitespace,
    /// Newline token.
    Newline,
    /// Comment token.
    Comment,
    /// Indent token for significant whitespace.
    Indent,
    /// Dedent token for significant whitespace.
    Dedent,

    /// End of stream marker.
    EndOfStream,
    /// Error token.
    Error,
}

impl oak_core::TokenType for MojoTokenType {
    const END_OF_STREAM: Self = MojoTokenType::EndOfStream;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            MojoTokenType::Fn
            | MojoTokenType::Struct
            | MojoTokenType::Var
            | MojoTokenType::Let
            | MojoTokenType::If
            | MojoTokenType::Else
            | MojoTokenType::While
            | MojoTokenType::For
            | MojoTokenType::In
            | MojoTokenType::Return
            | MojoTokenType::Break
            | MojoTokenType::Continue
            | MojoTokenType::Import
            | MojoTokenType::From
            | MojoTokenType::True
            | MojoTokenType::False
            | MojoTokenType::None => UniversalTokenRole::Keyword,

            MojoTokenType::Identifier => UniversalTokenRole::Name,

            MojoTokenType::Integer | MojoTokenType::Float | MojoTokenType::String => UniversalTokenRole::Literal,

            MojoTokenType::Plus
            | MojoTokenType::Minus
            | MojoTokenType::Star
            | MojoTokenType::Slash
            | MojoTokenType::Percent
            | MojoTokenType::Equal
            | MojoTokenType::EqualEqual
            | MojoTokenType::NotEqual
            | MojoTokenType::Less
            | MojoTokenType::LessEqual
            | MojoTokenType::Greater
            | MojoTokenType::GreaterEqual
            | MojoTokenType::And
            | MojoTokenType::Or
            | MojoTokenType::Not => UniversalTokenRole::Operator,

            MojoTokenType::LeftParen
            | MojoTokenType::RightParen
            | MojoTokenType::LeftBracket
            | MojoTokenType::RightBracket
            | MojoTokenType::LeftBrace
            | MojoTokenType::RightBrace
            | MojoTokenType::Comma
            | MojoTokenType::Dot
            | MojoTokenType::Colon
            | MojoTokenType::Semicolon
            | MojoTokenType::Arrow => UniversalTokenRole::Punctuation,

            MojoTokenType::Whitespace | MojoTokenType::Newline | MojoTokenType::Comment | MojoTokenType::Indent | MojoTokenType::Dedent => UniversalTokenRole::Whitespace,

            MojoTokenType::EndOfStream => UniversalTokenRole::Eof,
            MojoTokenType::Error => UniversalTokenRole::Error,
        }
    }
}
