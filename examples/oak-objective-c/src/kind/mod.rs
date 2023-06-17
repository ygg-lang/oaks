use oak_core::SyntaxKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ObjectiveCLanguageSyntaxKind {
    // 节点类型
    Root,
    InterfaceDeclaration,
    ImplementationDeclaration,
    MethodDeclaration,
    PropertyDeclaration,
    ProtocolDeclaration,
    CategoryDeclaration,
    ClassExtension,

    // 关键字
    InterfaceKeyword,
    ImplementationKeyword,
    EndKeyword,
    PropertyKeyword,
    SynthesizeKeyword,
    DynamicKeyword,
    ProtocolKeyword,
    CategoryKeyword,
    ImportKeyword,
    IncludeKeyword,
    IfKeyword,
    ElseKeyword,
    ForKeyword,
    WhileKeyword,
    DoKeyword,
    SwitchKeyword,
    CaseKeyword,
    DefaultKeyword,
    BreakKeyword,
    ContinueKeyword,
    ReturnKeyword,
    VoidKeyword,
    IntKeyword,
    FloatKeyword,
    DoubleKeyword,
    CharKeyword,
    BoolKeyword,
    IdKeyword,
    SelfKeyword,
    SuperKeyword,
    NilKeyword,
    YesKeyword,
    NoKeyword,

    // 符号
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    Question,
    At,

    // 字面量
    Identifier,
    Number,
    String,
    Character,

    // 其他
    Whitespace,
    Newline,
    CommentToken,
    Error,
    Eof,
}

impl SyntaxKind for ObjectiveCLanguageSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::CommentToken)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::CommentToken)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(
            self,
            Self::Root
                | Self::InterfaceDeclaration
                | Self::ImplementationDeclaration
                | Self::MethodDeclaration
                | Self::PropertyDeclaration
                | Self::ProtocolDeclaration
                | Self::CategoryDeclaration
                | Self::ClassExtension
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            Self::Root
                | Self::InterfaceDeclaration
                | Self::ImplementationDeclaration
                | Self::MethodDeclaration
                | Self::PropertyDeclaration
                | Self::ProtocolDeclaration
                | Self::CategoryDeclaration
                | Self::ClassExtension
        )
    }
}
