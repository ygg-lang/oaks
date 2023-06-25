use oak_core::{Token, TokenType, UniversalTokenRole};

/// Represents a Nix token.
pub type NixToken = Token<NixTokenType>;

impl NixTokenType {
    /// Returns true if the token represents a structural element.
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::Set | Self::List | Self::Lambda | Self::LetIn | Self::IfThenElse | Self::AttrPath | Self::Binding)
    }

    /// Returns true if the token is a terminal token.
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    /// Returns true if the token is a language keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(self, Self::Let | Self::In | Self::If | Self::Then | Self::Else | Self::With | Self::Inherit | Self::Rec | Self::Import | Self::Assert | Self::Or | Self::And | Self::Not)
    }

    /// Returns true if the token is an operator.
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::Concatenation
                | Self::Update
                | Self::Implication
                | Self::Equal
                | Self::NotEqual
                | Self::Less
                | Self::Greater
                | Self::LessEqual
                | Self::GreaterEqual
                | Self::LogicalAnd
                | Self::LogicalOr
                | Self::Assign
                | Self::Question
        )
    }

    /// Returns true if the token is a punctuation mark.
    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Colon | Self::Comma | Self::Dot | Self::At | Self::Dollar | Self::Hash)
    }
}

impl TokenType for NixTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::String | Self::Number | Self::Boolean | Self::True | Self::False | Self::Null => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            _ if self.is_operator() => UniversalTokenRole::Operator,
            _ if self.is_punctuation() => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }
}

/// Token types for the Nix language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NixTokenType {
    /// Whitespace characters.
    Whitespace,
    /// Newline characters.
    Newline,
    /// Source code comments.
    Comment,
    /// String literal.
    String,
    /// Numeric literal.
    Number,
    /// Boolean literal.
    Boolean,
    /// The `true` keyword.
    True,
    /// The `false` keyword.
    False,
    /// The `null` keyword.
    Null,
    /// A name identifier.
    Identifier,

    /// The `let` keyword.
    Let,
    /// The `in` keyword.
    In,
    /// The `if` keyword.
    If,
    /// The `then` keyword.
    Then,
    /// The `else` keyword.
    Else,
    /// The `with` keyword.
    With,
    /// The `inherit` keyword.
    Inherit,
    /// The `rec` keyword.
    Rec,
    /// The `import` keyword.
    Import,
    /// The `assert` keyword.
    Assert,
    /// The `or` keyword.
    Or,
    /// The `and` keyword.
    And,
    /// The `not` keyword.
    Not,

    /// Addition operator: `+`.
    Plus,
    /// Subtraction operator: `-`.
    Minus,
    /// Multiplication operator: `*`.
    Star,
    /// Division operator: `/`.
    Slash,
    /// Modulo operator: `%`.
    Percent,
    /// Concatenation operator: `++`.
    Concatenation,
    /// Update operator: `//`.
    Update,
    /// Implication operator: `->`.
    Implication,
    /// Equality comparison: `==`.
    Equal,
    /// Inequality comparison: `!=`.
    NotEqual,
    /// Less than: `<`.
    Less,
    /// Greater than: `>`.
    Greater,
    /// Less than or equal: `<=`.
    LessEqual,
    /// Greater than or equal: `>=`.
    GreaterEqual,
    /// Logical AND: `&&`.
    LogicalAnd,
    /// Logical OR: `||`.
    LogicalOr,
    /// Assignment: `=`.
    Assign,
    /// Question mark: `?`.
    Question,

    /// Left parenthesis: `(`.
    LeftParen,
    /// Right parenthesis: `)`.
    RightParen,
    /// Left brace: `{`.
    LeftBrace,
    /// Right brace: `}`.
    RightBrace,
    /// Left bracket: `[`.
    LeftBracket,
    /// Right bracket: `]`.
    RightBracket,
    /// Semicolon: `;`.
    Semicolon,
    /// Colon: `:`.
    Colon,
    /// Comma: `,`.
    Comma,
    /// Dot: `.`.
    Dot,
    /// At symbol: `@`.
    At,
    /// Dollar sign: `$`.
    Dollar,
    /// Hash sign: `#`.
    Hash,

    /// The root of a Nix document.
    Root,
    /// A set definition.
    Set,
    /// A list definition.
    List,
    /// A lambda function.
    Lambda,
    /// A let-in expression.
    LetIn,
    /// An if-then-else expression.
    IfThenElse,
    /// An attribute path.
    AttrPath,
    /// A binding.
    Binding,

    /// An error token.
    Error,
    /// End of stream.
    Eof,
}
