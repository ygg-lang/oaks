use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DockerfileElementType {
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

impl DockerfileElementType {
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

impl ElementType for DockerfileElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,

            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::DockerfileTokenType> for DockerfileElementType {
    fn from(token: crate::lexer::token_type::DockerfileTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
