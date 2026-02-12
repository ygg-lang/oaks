use oak_core::{ElementType, Parser, UniversalElementRole};

/// Element types for GraphQL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GraphQLElementType {
    /// A string literal.
    StringLiteral,
    /// An integer literal.
    IntLiteral,
    /// A float literal.
    FloatLiteral,
    /// A boolean literal.
    BooleanLiteral,
    /// A null literal.
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
    /// The `repeatable` keyword.
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
    /// Pipe `|`.
    Pipe,
    /// Ampersand `&`.
    Ampersand,
    /// Equals `=`.
    Equals,
    /// Exclamation `!`.
    Exclamation,
    /// At symbol `@`.
    At,
    /// Dollar sign `$`.
    Dollar,
    /// Whitespace.
    Whitespace,
    /// A comment.
    Comment,
    /// The root source file.
    SourceFile,
    /// A newline.
    Newline,
    /// End of file.
    Eof,
    /// An error element.
    Error,
}

impl GraphQLElementType {
    /// Returns true if the element type is a keyword.
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

impl ElementType for GraphQLElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::GraphQLTokenType> for GraphQLElementType {
    fn from(token: crate::lexer::token_type::GraphQLTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
