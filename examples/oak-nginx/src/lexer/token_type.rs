use oak_core::{Token, TokenType, UniversalTokenRole};

/// Represents a token in an Nginx configuration file.
pub type NginxToken = Token<NginxTokenType>;

/// Token types for Nginx configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NginxTokenType {
    /// The root of the configuration.
    Root,
    /// A configuration directive (e.g., `worker_processes 1;`).
    Directive,
    /// A configuration block (e.g., `server { ... }`).
    Block,
    /// A parameter within a directive.
    Parameter,
    /// A value within a directive or parameter.
    Value,
    /// A comment.
    Comment,

    /// The `server` keyword.
    ServerKeyword, // server
    /// The `location` keyword.
    LocationKeyword, // location
    /// The `upstream` keyword.
    UpstreamKeyword, // upstream
    /// The `http` keyword.
    HttpKeyword, // http
    /// The `events` keyword.
    EventsKeyword, // events
    /// The `listen` keyword.
    ListenKeyword, // listen
    /// The `server_name` keyword.
    ServerNameKeyword, // server_name
    /// The `root` keyword.
    RootKeyword, // root
    /// The `index` keyword.
    IndexKeyword, // index
    /// The `proxy_pass` keyword.
    ProxyPassKeyword, // proxy_pass

    /// Left brace `{`.
    LeftBrace, // {
    /// Right brace `}`.
    RightBrace, // }
    /// Semicolon `;`.
    Semicolon, // ;

    /// An identifier.
    Identifier,
    /// A string literal.
    String,
    /// A numeric literal.
    Number,
    /// A file path.
    Path,
    /// A URL.
    Url,

    /// Whitespace characters.
    Whitespace,
    /// Newline characters.
    Newline,
    /// A comment token.
    CommentToken,
    /// End of stream.
    Eof,
    /// An error token.
    Error,
}

impl NginxTokenType {
    /// Returns true if the token type represents a structural element.
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::Directive | Self::Block | Self::Parameter | Self::Value | Self::Comment)
    }

    /// Returns true if the token type represents a lexical token.
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
