use oak_core::{Token, TokenRole, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type NginxToken = Token<NginxTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NginxTokenType {
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

impl NginxTokenType {
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::Directive | Self::Block | Self::Parameter | Self::Value | Self::Comment)
    }

    pub fn is_token(&self) -> bool {
        !self.is_element()
    }
}

impl TokenType for NginxTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::ServerKeyword | Self::LocationKeyword | Self::UpstreamKeyword | Self::HttpKeyword | Self::EventsKeyword | Self::ListenKeyword | Self::ServerNameKeyword | Self::RootKeyword | Self::IndexKeyword | Self::ProxyPassKeyword => {
                UniversalTokenRole::Keyword
            }

            Self::LeftBrace | Self::RightBrace | Self::Semicolon => UniversalTokenRole::Punctuation,

            Self::Identifier => UniversalTokenRole::Name,
            Self::String | Self::Number | Self::Path | Self::Url => UniversalTokenRole::Literal,

            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::CommentToken => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
