use oak_core::{TokenType, UniversalTokenRole};

// pub type DockerfileToken = Token<DockerfileTokenType>;

/// Token types for the Dockerfile lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DockerfileTokenType {
    /// An identifier (e.g., variable name, image name).
    Identifier,
    /// A string literal.
    String,
    /// A numeric literal.
    Number,
    /// Whitespace (spaces, tabs).
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
    /// `MAINTAINER` instruction (deprecated).
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
    /// `AS` keyword (used in `FROM`).
    As,
    /// No-op or placeholder.
    None,
    /// `interval` keyword (in `HEALTHCHECK`).
    Interval,
    /// `timeout` keyword (in `HEALTHCHECK`).
    Timeout,
    /// `start-period` keyword (in `HEALTHCHECK`).
    StartPeriod,
    /// `retries` keyword (in `HEALTHCHECK`).
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
    /// A comment starting with `#`.
    Comment,
    /// A file path.
    Path,
    /// The root of the Dockerfile.
    Root,
    /// A statement or instruction.
    Statement,
    /// An error token.
    Error,
    /// End of stream.
    Eof,
}

impl DockerfileTokenType {
    /// Returns `true` if this token type is a Dockerfile instruction.
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

    /// Returns `true` if this token type is a trivia token (whitespace, newline, or comment).
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
