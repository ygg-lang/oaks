use oak_core::SyntaxKind;

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
    Newline,
    Eof,
    Error,
}

impl SyntaxKind for GraphQLSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Newline)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Eof | Self::Error)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Eof | Self::Error)
    }
}
