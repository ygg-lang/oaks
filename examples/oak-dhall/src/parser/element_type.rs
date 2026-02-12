use oak_core::{ElementType, UniversalElementRole};

/// Element types for DHall AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum DHallElementType {
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    /// Identifier.
    Identifier,
    /// Number.
    Number,
    /// String literal.
    String,

    // Keywords
    /// `if`
    If,
    /// `then`
    Then,
    /// `else`
    Else,
    /// `let`
    Let,
    /// `in`
    In,
    /// `using`
    Using,
    /// `as`
    As,
    /// `merge`
    Merge,
    /// `Some`
    Some,
    /// `None`
    None,
    /// `NaN`
    NaN,
    /// `Infinity`
    Infinity,
    /// `Type`
    Type,
    /// `Kind`
    Kind,
    /// `Sort`
    Sort,
    /// `Bool`
    Bool,
    /// `Natural`
    Natural,
    /// `Integer`
    Integer,
    /// `Double`
    Double,
    /// `Text`
    Text,
    /// `List`
    List,
    /// `Optional`
    Optional,
    /// `True`
    True,
    /// `False`
    False,
    /// `with`
    With,
    /// `forall` or `∀`
    Forall,
    /// `assert`
    Assert,

    // Operators
    /// `->` or `→`
    Arrow,
    /// `=>` or `⇒`
    FatArrow,
    /// `==`
    EqualEqual,
    /// `!=`
    NotEqual,
    /// `&&`
    And,
    /// `||`
    Or,
    /// `++`
    Append,
    /// `//`
    Combine,
    /// `//\\`
    CombineTypes,
    /// `///`
    Prefer,
    /// `\` or `λ`
    Lambda,

    // Punctuation
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `,`
    Comma,
    /// `;`
    Semicolon,
    /// `.`
    Dot,
    /// `:`
    Colon,
    /// `=`
    Equal,
    /// `<`
    Less,
    /// `>`
    Greater,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `|`
    Pipe,
    /// `@`
    At,
    /// `#`
    Hash,
    /// `?`
    Question,
    /// Error element.
    Error,
    /// End of file.
    Eof,

    // Special
    /// Root node.
    Root,
    /// Source file node.
    SourceFile,
}

impl DHallElementType {
    /// Returns `true` if the element type is a keyword.
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

impl ElementType for DHallElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::DHallTokenType> for DHallElementType {
    fn from(token: crate::lexer::token_type::DHallTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
