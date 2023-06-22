use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

pub type DHallToken = Token<DHallSyntaxKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DHallSyntaxKind {
    /// Whitespace characters
    Whitespace,
    /// Newline character
    Newline,
    /// Comment text
    Comment,
    /// Identifier name
    Identifier,
    /// Number literal
    Number,
    /// String literal
    String,

    // Keywords
    /// The 'if' keyword
    If,
    /// The 'then' keyword
    Then,
    /// The 'else' keyword
    Else,
    /// The 'let' keyword
    Let,
    /// The 'in' keyword
    In,
    /// The 'using' keyword
    Using,
    /// The 'as' keyword
    As,
    /// The 'merge' keyword
    Merge,
    /// The 'Some' keyword
    Some,
    /// The 'None' keyword
    None,
    /// The 'NaN' keyword
    NaN,
    /// The 'Infinity' keyword
    Infinity,
    /// The 'Type' keyword
    Type,
    /// The 'Kind' keyword
    Kind,
    /// The 'Sort' keyword
    Sort,
    /// The 'Bool' keyword
    Bool,
    /// The 'Natural' keyword
    Natural,
    /// The 'Integer' keyword
    Integer,
    /// The 'Double' keyword
    Double,
    /// The 'Text' keyword
    Text,
    /// The 'List' keyword
    List,
    /// The 'Optional' keyword
    Optional,
    /// The 'True' keyword
    True,
    /// The 'False' keyword
    False,
    /// The 'with' keyword
    With,
    /// The 'forall' keyword
    Forall,
    /// The 'assert' keyword
    Assert,

    // Operators
    /// Arrow operator (-> or →)
    Arrow,
    /// Fat arrow operator (=>)
    FatArrow,
    /// Equality operator (== or ≡)
    EqualEqual,
    /// Inequality operator (!=)
    NotEqual,
    /// Logical AND (&& or ∧)
    And,
    /// Logical OR (|| or ∨)
    Or,
    /// List concatenation (++)
    Append,
    /// Record combination (// or ⫽)
    Combine,
    /// Type combination (/\ or ⩓)
    CombineTypes,
    /// Record preference (//\)
    Prefer,
    /// Lambda operator (\ or λ)
    Lambda,

    // Punctuation
    /// Left parenthesis (()
    LeftParen,
    /// Right parenthesis ())
    RightParen,
    /// Left brace ({)
    LeftBrace,
    /// Right brace (})
    RightBrace,
    /// Left bracket ([)
    LeftBracket,
    /// Right bracket (])
    RightBracket,
    /// Comma (,)
    Comma,
    /// Semicolon (;)
    Semicolon,
    /// Dot (.)
    Dot,
    /// Colon (:)
    Colon,
    /// Assignment operator (=)
    Equal,
    /// Less than operator (<)
    Less,
    /// Greater than operator (>)
    Greater,
    /// Plus operator (+)
    Plus,
    /// Minus operator (-)
    Minus,
    /// Multiplication operator (*)
    Star,
    /// Division operator (/)
    Slash,
    /// Pipe operator (|)
    Pipe,
    /// At symbol (@)
    At,
    /// Hash symbol (#)
    Hash,
    /// Question mark (?)
    Question,

    /// Error token
    Error,
    /// End of file marker
    Eof,

    // Special
    Root,
    SourceFile,
}

impl TokenType for DHallSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number | Self::String => UniversalTokenRole::Literal,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Equal | Self::Arrow | Self::FatArrow | Self::EqualEqual | Self::NotEqual | Self::And | Self::Or | Self::Append | Self::Combine | Self::CombineTypes | Self::Prefer | Self::Lambda => {
                UniversalTokenRole::Operator
            }
            _ => UniversalTokenRole::Punctuation,
        }
    }
}

impl DHallSyntaxKind {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::If
                | Self::Then
                | Self::Else
                | Self::Let
                | Self::In
                | Self::Using
                | Self::As
                | Self::Merge
                | Self::Some
                | Self::None
                | Self::NaN
                | Self::Infinity
                | Self::Type
                | Self::Kind
                | Self::Sort
                | Self::Bool
                | Self::Natural
                | Self::Integer
                | Self::Double
                | Self::Text
                | Self::List
                | Self::Optional
                | Self::True
                | Self::False
                | Self::With
                | Self::Forall
                | Self::Assert
        )
    }
}

impl ElementType for DHallSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
