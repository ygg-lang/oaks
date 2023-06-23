use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type DockerfileToken = Token<DockerfileTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DockerfileTokenType {
    Identifier,
    String,
    Number,
    Whitespace,
    Newline,
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
    As,
    None,
    Interval,
    Timeout,
    StartPeriod,
    Retries,
    Equal,
    Equals,
    Colon,
    Comma,
    Semicolon,
    Dollar,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Comment,
    Path,
    Root,
    Statement,
    Error,
    Eof,
}

impl DockerfileTokenType {
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

impl TokenType for DockerfileTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
