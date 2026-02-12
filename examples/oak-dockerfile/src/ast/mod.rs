#![doc = include_str!("readme.md")]
use oak_core::Range;

/// Root node of the Dockerfile AST.
#[derive(Debug, PartialEq, Clone)]
pub struct DockerfileRoot {
    /// The list of instructions in the Dockerfile.
    pub instructions: Vec<Instruction>,
}

/// A Dockerfile instruction.
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    /// A `FROM` instruction.
    From {
        /// The base image.
        image: String,
        /// Optional tag for the base image.
        tag: Option<String>,
        /// The source range.
        span: Range<usize>,
    },
    /// A `RUN` instruction.
    Run {
        /// The command to run.
        command: String,
        /// The source range.
        span: Range<usize>,
    },
    /// A `COPY` instruction.
    Copy {
        /// Source path.
        src: String,
        /// Destination path.
        dest: String,
        /// The source range.
        span: Range<usize>,
    },
    /// An `ADD` instruction.
    Add {
        /// Source path.
        src: String,
        /// Destination path.
        dest: String,
        /// The source range.
        span: Range<usize>,
    },
    /// A `WORKDIR` instruction.
    Workdir {
        /// The working directory path.
        path: String,
        /// The source range.
        span: Range<usize>,
    },
    /// An `EXPOSE` instruction.
    Expose {
        /// The port to expose.
        port: String,
        /// The source range.
        span: Range<usize>,
    },
    /// An `ENV` instruction.
    Env {
        /// The environment variable key.
        key: String,
        /// The environment variable value.
        value: String,
        /// The source range.
        span: Range<usize>,
    },
    /// A `CMD` instruction.
    Cmd {
        /// The command to execute.
        command: String,
        /// The source range.
        span: Range<usize>,
    },
    /// An `ENTRYPOINT` instruction.
    Entrypoint {
        /// The entrypoint command.
        command: String,
        /// The source range.
        span: Range<usize>,
    },
    /// A `VOLUME` instruction.
    Volume {
        /// The volume path.
        path: String,
        /// The source range.
        span: Range<usize>,
    },
    /// A `USER` instruction.
    User {
        /// The username or UID.
        user: String,
        /// The source range.
        span: Range<usize>,
    },
    /// A `LABEL` instruction.
    Label {
        /// The label key.
        key: String,
        /// The label value.
        value: String,
        /// The source range.
        span: Range<usize>,
    },
    /// An `ARG` instruction.
    Arg {
        /// The argument name.
        name: String,
        /// Optional default value.
        default: Option<String>,
        /// The source range.
        span: Range<usize>,
    },
    /// An `ONBUILD` instruction.
    Onbuild {
        /// The nested instruction.
        instruction: Box<Instruction>,
        /// The source range.
        span: Range<usize>,
    },
    /// A `STOPSIGNAL` instruction.
    Stopsignal {
        /// The signal name or number.
        signal: String,
        /// The source range.
        span: Range<usize>,
    },
    /// A `HEALTHCHECK` instruction.
    Healthcheck {
        /// The health check command.
        command: String,
        /// The source range.
        span: Range<usize>,
    },
    /// A `SHELL` instruction.
    Shell {
        /// The shell command.
        shell: String,
        /// The source range.
        span: Range<usize>,
    },
}
