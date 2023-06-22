pub type DockerfileToken = Token<DockerfileSyntaxKind>;

/// Dockerfile 语法种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum DockerfileSyntaxKind {
    // 基本 kind
    Identifier,
    String,
    Number,
    Whitespace,
    Newline,

    // Dockerfile 指令
    From,
    Run,
    Cmd,
    Label,
    Maintainer,
    Expose,
    Env,
    Add,
    Copy,
    Entrypoint,
    Volume,
    User,
    Workdir,
    Arg,
    Onbuild,
    Stopsignal,
    Healthcheck,
    Shell,

    // 特殊关键
    As,
    None,
    Interval,
    Timeout,
    StartPeriod,
    Retries,

    // 操作符和分隔符
    Equal,        // = (保持向后兼容)
    Equals,       // = (新名称)
    Colon,        // :
    Comma,        // ,
    Semicolon,    // ;
    Dollar,       // $
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    LeftParen,    // (
    RightParen,   // )

    // 注释
    Comment,

    // 路径和文件名
    Path,

    // 特殊
    Root,
    Statement,
    Error,
    Eof,
}

impl DockerfileSyntaxKind {
    pub fn is_instruction(&self) -> bool {
        matches!(
            self,
            Self::From
                | Self::Run
                | Self::Cmd
                | Self::Label
                | Self::Maintainer
                | Self::Expose
                | Self::Env
                | Self::Add
                | Self::Copy
                | Self::Entrypoint
                | Self::Volume
                | Self::User
                | Self::Workdir
                | Self::Arg
                | Self::Onbuild
                | Self::Stopsignal
                | Self::Healthcheck
                | Self::Shell
        )
    }

    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};

impl TokenType for DockerfileSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            Self::From
            | Self::Run
            | Self::Cmd
            | Self::Label
            | Self::Maintainer
            | Self::Expose
            | Self::Env
            | Self::Add
            | Self::Copy
            | Self::Entrypoint
            | Self::Volume
            | Self::User
            | Self::Workdir
            | Self::Arg
            | Self::Onbuild
            | Self::Stopsignal
            | Self::Healthcheck
            | Self::Shell => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::String => UniversalTokenRole::Literal,
            Self::Number => UniversalTokenRole::Literal,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for DockerfileSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Statement => UniversalElementRole::Statement,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
