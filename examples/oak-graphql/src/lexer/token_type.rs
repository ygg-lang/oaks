use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

/// Represents a token in a GraphQL source file.
pub type GraphQLToken = Token<GraphQLTokenType>;

/// Token types for GraphQL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GraphQLTokenType {
    /// A string literal.
    StringLiteral,
    /// An integer literal.
    IntLiteral,
    /// A float literal.
    FloatLiteral,
    /// A boolean literal (`true` or `false`).
    BooleanLiteral,
    /// A null literal (`null`).
    NullLiteral,
    /// A name (identifier).
    Name,
    /// The `query` keyword.
    QueryKeyword,
    /// The `mutation` keyword.
    MutationKeyword,
    /// The `subscription` keyword.
    SubscriptionKeyword,
    /// The `fragment` keyword.
    FragmentKeyword,
    /// The `on` keyword.
    OnKeyword,
    /// The `type` keyword.
    TypeKeyword,
    /// The `interface` keyword.
    InterfaceKeyword,
    /// The `union` keyword.
    UnionKeyword,
    /// The `scalar` keyword.
    ScalarKeyword,
    /// The `enum` keyword.
    EnumKeyword,
    /// The `input` keyword.
    InputKeyword,
    /// The `extend` keyword.
    ExtendKeyword,
    /// The `schema` keyword.
    SchemaKeyword,
    /// The `directive` keyword.
    DirectiveKeyword,
    /// The `implements` keyword.
    ImplementsKeyword,
    /// The `repeats` keyword.
    RepeatsKeyword,
    /// The spread operator `...`.
    Spread,
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
    /// Colon `:`.
    Colon,
    /// Semicolon `;`.
    Semicolon,
    /// Pipe symbol `|`.
    Pipe,
    /// Ampersand symbol `&`.
    Ampersand,
    /// Equals symbol `=`.
    Equals,
    /// Exclamation mark `!`.
    Exclamation,
    /// At symbol `@`.
    At,
    /// Dollar sign `$`.
    Dollar,
    /// Whitespace.
    Whitespace,
    /// A comment.
    Comment,
    /// The root of the source file.
    SourceFile,
    /// A newline.
    Newline,
    /// End of file.
    Eof,
    /// An error token.
    Error,
}

impl GraphQLTokenType {
    /// Returns true if the token type is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::QueryKeyword
                | Self::MutationKeyword
                | Self::SubscriptionKeyword
                | Self::FragmentKeyword
                | Self::OnKeyword
                | Self::TypeKeyword
                | Self::InterfaceKeyword
                | Self::UnionKeyword
                | Self::ScalarKeyword
                | Self::EnumKeyword
                | Self::InputKeyword
                | Self::ExtendKeyword
                | Self::SchemaKeyword
                | Self::DirectiveKeyword
                | Self::ImplementsKeyword
                | Self::RepeatsKeyword
        )
    }
}

impl TokenType for GraphQLTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
