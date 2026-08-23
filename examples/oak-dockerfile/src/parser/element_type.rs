use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Dockerfile parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DockerfileElementType {
    /// An identifier.
    Identifier,
    /// A string literal.
    String,
    /// A numeric literal.
    Number,
    /// Whitespace.
    Whitespace,
    /// A newline character.
    Newline,
    /// `FROM` instruction.
    From,
    /// `RUN` instruction.
    Run,
    /// `CMD` instruction.
    Cmd,
    /// `LABEL` instruction.
    Label,
    /// `MAINTAINER` instruction.
    Maintainer,
    /// `EXPOSE` instruction.
    Expose,
    /// `ENV` instruction.
    Env,
    /// `ADD` instruction.
    Add,
    /// `COPY` instruction.
    Copy,
    /// `ENTRYPOINT` instruction.
    Entrypoint,
    /// `VOLUME` instruction.
    Volume,
    /// `USER` instruction.
    User,
    /// `WORKDIR` instruction.
    Workdir,
    /// `ARG` instruction.
    Arg,
    /// `ONBUILD` instruction.
    Onbuild,
    /// `STOPSIGNAL` instruction.
    Stopsignal,
    /// `HEALTHCHECK` instruction.
    Healthcheck,
    /// `SHELL` instruction.
    Shell,
    /// `AS` keyword.
    As,
    /// No-op or placeholder.
    None,
    /// `interval` keyword.
    Interval,
    /// `timeout` keyword.
    Timeout,
    /// `start-period` keyword.
    StartPeriod,
    /// `retries` keyword.
    Retries,
    /// Single equals sign `=`.
    Equal,
    /// Double equals sign `==`.
    Equals,
    /// Colon `:`.
    Colon,
    /// Comma `,`.
    Comma,
    /// Semicolon `;`.
    Semicolon,
    /// Dollar sign `$`.
    Dollar,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// A comment.
    Comment,
    /// A file path.
    Path,
    /// The root of the Dockerfile.
    Root,
    /// A statement or instruction.
    Statement,
    /// An error element.
    Error,
    /// End of stream.
    Eof,
}

impl DockerfileElementType {
    /// Returns `true` if this element type is a Dockerfile instruction.
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

    /// Returns `true` if this element type is a trivia element (whitespace, newline, or comment).
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
                match token {
            crate::lexer::token_type::DockerfileTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::DockerfileTokenType::String => Self::String,
            crate::lexer::token_type::DockerfileTokenType::Number => Self::Number,
            crate::lexer::token_type::DockerfileTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::DockerfileTokenType::Newline => Self::Newline,
            crate::lexer::token_type::DockerfileTokenType::From => Self::From,
            crate::lexer::token_type::DockerfileTokenType::Run => Self::Run,
            crate::lexer::token_type::DockerfileTokenType::Cmd => Self::Cmd,
            crate::lexer::token_type::DockerfileTokenType::Label => Self::Label,
            crate::lexer::token_type::DockerfileTokenType::Maintainer => Self::Maintainer,
            crate::lexer::token_type::DockerfileTokenType::Expose => Self::Expose,
            crate::lexer::token_type::DockerfileTokenType::Env => Self::Env,
            crate::lexer::token_type::DockerfileTokenType::Add => Self::Add,
            crate::lexer::token_type::DockerfileTokenType::Copy => Self::Copy,
            crate::lexer::token_type::DockerfileTokenType::Entrypoint => Self::Entrypoint,
            crate::lexer::token_type::DockerfileTokenType::Volume => Self::Volume,
            crate::lexer::token_type::DockerfileTokenType::User => Self::User,
            crate::lexer::token_type::DockerfileTokenType::Workdir => Self::Workdir,
            crate::lexer::token_type::DockerfileTokenType::Arg => Self::Arg,
            crate::lexer::token_type::DockerfileTokenType::Onbuild => Self::Onbuild,
            crate::lexer::token_type::DockerfileTokenType::Stopsignal => Self::Stopsignal,
            crate::lexer::token_type::DockerfileTokenType::Healthcheck => Self::Healthcheck,
            crate::lexer::token_type::DockerfileTokenType::Shell => Self::Shell,
            crate::lexer::token_type::DockerfileTokenType::As => Self::As,
            crate::lexer::token_type::DockerfileTokenType::None => Self::None,
            crate::lexer::token_type::DockerfileTokenType::Interval => Self::Interval,
            crate::lexer::token_type::DockerfileTokenType::Timeout => Self::Timeout,
            crate::lexer::token_type::DockerfileTokenType::StartPeriod => Self::StartPeriod,
            crate::lexer::token_type::DockerfileTokenType::Retries => Self::Retries,
            crate::lexer::token_type::DockerfileTokenType::Equal => Self::Equal,
            crate::lexer::token_type::DockerfileTokenType::Equals => Self::Equals,
            crate::lexer::token_type::DockerfileTokenType::Colon => Self::Colon,
            crate::lexer::token_type::DockerfileTokenType::Comma => Self::Comma,
            crate::lexer::token_type::DockerfileTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::DockerfileTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::DockerfileTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::DockerfileTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::DockerfileTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::DockerfileTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::DockerfileTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::DockerfileTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::DockerfileTokenType::Comment => Self::Comment,
            crate::lexer::token_type::DockerfileTokenType::Path => Self::Path,
            crate::lexer::token_type::DockerfileTokenType::Root => Self::Root,
            crate::lexer::token_type::DockerfileTokenType::Statement => Self::Statement,
            crate::lexer::token_type::DockerfileTokenType::Error => Self::Error,
            crate::lexer::token_type::DockerfileTokenType::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}
