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
        unsafe { std::mem::transmute(token) }
    }
}
