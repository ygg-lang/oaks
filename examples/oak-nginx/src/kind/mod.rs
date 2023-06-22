use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl TokenType for NginxSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::CommentToken | Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::CommentToken | Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for NginxSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            Self::Root => UniversalElementRole::Root,
            Self::Directive | Self::Block => UniversalElementRole::Detail,
            _ => UniversalElementRole::None,
        }
    }
}

impl NginxSyntaxKind {
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::Directive | Self::Block | Self::Parameter | Self::Value | Self::Comment)
    }

    pub fn is_token(&self) -> bool {
        !self.is_element()
    }
}
