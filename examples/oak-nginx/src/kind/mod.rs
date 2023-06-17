use oak_core::SyntaxKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NginxSyntaxKind {
    // 节点种类
    Root,
    Directive,
    Block,
    Parameter,
    Value,
    Comment,

    // 词法种类 - 关键字
    ServerKeyword,     // server
    LocationKeyword,   // location
    UpstreamKeyword,   // upstream
    HttpKeyword,       // http
    EventsKeyword,     // events
    ListenKeyword,     // listen
    ServerNameKeyword, // server_name
    RootKeyword,       // root
    IndexKeyword,      // index
    ProxyPassKeyword,  // proxy_pass

    // 词法种类 - 符号
    LeftBrace,  // {
    RightBrace, // }
    Semicolon,  // ;

    // 词法种类 - 字面量
    Identifier,
    String,
    Number,
    Path,
    Url,

    // 词法种类 - 其他
    Whitespace,
    Newline,
    CommentToken,
    Eof,
    Error,
}

impl SyntaxKind for NginxSyntaxKind {
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
        !matches!(self, Self::Root | Self::Directive | Self::Block | Self::Parameter | Self::Value | Self::Comment)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Root | Self::Directive | Self::Block | Self::Parameter | Self::Value | Self::Comment)
    }
}
