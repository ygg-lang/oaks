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
                match token {
            crate::lexer::token_type::GraphQLTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::GraphQLTokenType::IntLiteral => Self::IntLiteral,
            crate::lexer::token_type::GraphQLTokenType::FloatLiteral => Self::FloatLiteral,
            crate::lexer::token_type::GraphQLTokenType::BooleanLiteral => Self::BooleanLiteral,
            crate::lexer::token_type::GraphQLTokenType::NullLiteral => Self::NullLiteral,
            crate::lexer::token_type::GraphQLTokenType::Name => Self::Name,
            crate::lexer::token_type::GraphQLTokenType::QueryKeyword => Self::QueryKeyword,
            crate::lexer::token_type::GraphQLTokenType::MutationKeyword => Self::MutationKeyword,
            crate::lexer::token_type::GraphQLTokenType::SubscriptionKeyword => Self::SubscriptionKeyword,
            crate::lexer::token_type::GraphQLTokenType::FragmentKeyword => Self::FragmentKeyword,
            crate::lexer::token_type::GraphQLTokenType::OnKeyword => Self::OnKeyword,
            crate::lexer::token_type::GraphQLTokenType::TypeKeyword => Self::TypeKeyword,
            crate::lexer::token_type::GraphQLTokenType::InterfaceKeyword => Self::InterfaceKeyword,
            crate::lexer::token_type::GraphQLTokenType::UnionKeyword => Self::UnionKeyword,
            crate::lexer::token_type::GraphQLTokenType::ScalarKeyword => Self::ScalarKeyword,
            crate::lexer::token_type::GraphQLTokenType::EnumKeyword => Self::EnumKeyword,
            crate::lexer::token_type::GraphQLTokenType::InputKeyword => Self::InputKeyword,
            crate::lexer::token_type::GraphQLTokenType::ExtendKeyword => Self::ExtendKeyword,
            crate::lexer::token_type::GraphQLTokenType::SchemaKeyword => Self::SchemaKeyword,
            crate::lexer::token_type::GraphQLTokenType::DirectiveKeyword => Self::DirectiveKeyword,
            crate::lexer::token_type::GraphQLTokenType::ImplementsKeyword => Self::ImplementsKeyword,
            crate::lexer::token_type::GraphQLTokenType::RepeatsKeyword => Self::RepeatsKeyword,
            crate::lexer::token_type::GraphQLTokenType::Spread => Self::Spread,
            crate::lexer::token_type::GraphQLTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::GraphQLTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::GraphQLTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::GraphQLTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::GraphQLTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::GraphQLTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::GraphQLTokenType::Comma => Self::Comma,
            crate::lexer::token_type::GraphQLTokenType::Colon => Self::Colon,
            crate::lexer::token_type::GraphQLTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::GraphQLTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::GraphQLTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::GraphQLTokenType::Equals => Self::Equals,
            crate::lexer::token_type::GraphQLTokenType::Exclamation => Self::Exclamation,
            crate::lexer::token_type::GraphQLTokenType::At => Self::At,
            crate::lexer::token_type::GraphQLTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::GraphQLTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::GraphQLTokenType::Comment => Self::Comment,
            crate::lexer::token_type::GraphQLTokenType::SourceFile => Self::SourceFile,
            crate::lexer::token_type::GraphQLTokenType::Newline => Self::Newline,
            crate::lexer::token_type::GraphQLTokenType::Eof => Self::Eof,
            crate::lexer::token_type::GraphQLTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
