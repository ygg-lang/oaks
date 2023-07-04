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
                match token {
            crate::lexer::token_type::DHallTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::DHallTokenType::Newline => Self::Newline,
            crate::lexer::token_type::DHallTokenType::Comment => Self::Comment,
            crate::lexer::token_type::DHallTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::DHallTokenType::Number => Self::Number,
            crate::lexer::token_type::DHallTokenType::String => Self::String,
            crate::lexer::token_type::DHallTokenType::If => Self::If,
            crate::lexer::token_type::DHallTokenType::Then => Self::Then,
            crate::lexer::token_type::DHallTokenType::Else => Self::Else,
            crate::lexer::token_type::DHallTokenType::Let => Self::Let,
            crate::lexer::token_type::DHallTokenType::In => Self::In,
            crate::lexer::token_type::DHallTokenType::Using => Self::Using,
            crate::lexer::token_type::DHallTokenType::As => Self::As,
            crate::lexer::token_type::DHallTokenType::Merge => Self::Merge,
            crate::lexer::token_type::DHallTokenType::Some => Self::Some,
            crate::lexer::token_type::DHallTokenType::None => Self::None,
            crate::lexer::token_type::DHallTokenType::NaN => Self::NaN,
            crate::lexer::token_type::DHallTokenType::Infinity => Self::Infinity,
            crate::lexer::token_type::DHallTokenType::Type => Self::Type,
            crate::lexer::token_type::DHallTokenType::Kind => Self::Kind,
            crate::lexer::token_type::DHallTokenType::Sort => Self::Sort,
            crate::lexer::token_type::DHallTokenType::Bool => Self::Bool,
            crate::lexer::token_type::DHallTokenType::Natural => Self::Natural,
            crate::lexer::token_type::DHallTokenType::Integer => Self::Integer,
            crate::lexer::token_type::DHallTokenType::Double => Self::Double,
            crate::lexer::token_type::DHallTokenType::Text => Self::Text,
            crate::lexer::token_type::DHallTokenType::List => Self::List,
            crate::lexer::token_type::DHallTokenType::Optional => Self::Optional,
            crate::lexer::token_type::DHallTokenType::True => Self::True,
            crate::lexer::token_type::DHallTokenType::False => Self::False,
            crate::lexer::token_type::DHallTokenType::With => Self::With,
            crate::lexer::token_type::DHallTokenType::Forall => Self::Forall,
            crate::lexer::token_type::DHallTokenType::Assert => Self::Assert,
            crate::lexer::token_type::DHallTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::DHallTokenType::FatArrow => Self::FatArrow,
            crate::lexer::token_type::DHallTokenType::EqualEqual => Self::EqualEqual,
            crate::lexer::token_type::DHallTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::DHallTokenType::And => Self::And,
            crate::lexer::token_type::DHallTokenType::Or => Self::Or,
            crate::lexer::token_type::DHallTokenType::Append => Self::Append,
            crate::lexer::token_type::DHallTokenType::Combine => Self::Combine,
            crate::lexer::token_type::DHallTokenType::CombineTypes => Self::CombineTypes,
            crate::lexer::token_type::DHallTokenType::Prefer => Self::Prefer,
            crate::lexer::token_type::DHallTokenType::Lambda => Self::Lambda,
            crate::lexer::token_type::DHallTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::DHallTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::DHallTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::DHallTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::DHallTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::DHallTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::DHallTokenType::Comma => Self::Comma,
            crate::lexer::token_type::DHallTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::DHallTokenType::Dot => Self::Dot,
            crate::lexer::token_type::DHallTokenType::Colon => Self::Colon,
            crate::lexer::token_type::DHallTokenType::Equal => Self::Equal,
            crate::lexer::token_type::DHallTokenType::Less => Self::Less,
            crate::lexer::token_type::DHallTokenType::Greater => Self::Greater,
            crate::lexer::token_type::DHallTokenType::Plus => Self::Plus,
            crate::lexer::token_type::DHallTokenType::Minus => Self::Minus,
            crate::lexer::token_type::DHallTokenType::Star => Self::Star,
            crate::lexer::token_type::DHallTokenType::Slash => Self::Slash,
            crate::lexer::token_type::DHallTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::DHallTokenType::At => Self::At,
            crate::lexer::token_type::DHallTokenType::Hash => Self::Hash,
            crate::lexer::token_type::DHallTokenType::Question => Self::Question,
            crate::lexer::token_type::DHallTokenType::Error => Self::Error,
            crate::lexer::token_type::DHallTokenType::Eof => Self::Eof,
            crate::lexer::token_type::DHallTokenType::Root => Self::Root,
            crate::lexer::token_type::DHallTokenType::SourceFile => Self::SourceFile,
            _ => Self::Error,
        }
    }
}
