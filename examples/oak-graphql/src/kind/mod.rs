use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};

/// GraphQL 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphQLSyntaxKind {
    // 字面量
    StringLiteral,
    IntLiteral,
    FloatLiteral,
    BooleanLiteral,
    NullLiteral,

    // 标识符和名称
    Name,

    // 关键字
    QueryKeyword,
    MutationKeyword,
    SubscriptionKeyword,
    FragmentKeyword,
    OnKeyword,
    TypeKeyword,
    InterfaceKeyword,
    UnionKeyword,
    ScalarKeyword,
    EnumKeyword,
    InputKeyword,
    ExtendKeyword,
    SchemaKeyword,
    DirectiveKeyword,
    ImplementsKeyword,
    RepeatsKeyword,

    // 操作符
    Spread, // ...

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Colon,        // :
    Semicolon,    // ;
    Pipe,         // |
    Ampersand,    // &
    Equals,       // =
    Exclamation,  // !
    At,           // @
    Dollar,       // $

    // 空白和注释
    Whitespace,
    Comment,

    // 特殊
    SourceFile,
    Newline,
    Eof,
    Error,
}

impl GraphQLSyntaxKind {
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

impl TokenType for GraphQLSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Name => UniversalTokenRole::Name,
            Self::StringLiteral | Self::IntLiteral | Self::FloatLiteral | Self::BooleanLiteral | Self::NullLiteral => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Spread | Self::Equals | Self::Exclamation | Self::At | Self::Dollar | Self::Pipe | Self::Ampersand => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Comma | Self::Colon | Self::Semicolon => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for GraphQLSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}
