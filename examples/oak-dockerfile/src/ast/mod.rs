use oak_core::Range;

/// Dockerfile AST root type
#[derive(Debug, PartialEq, Clone)]
pub struct DockerfileRoot {
    pub instructions: Vec<Instruction>,
}

/// Dockerfile instruction
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    From { image: String, tag: Option<String>, span: Range<usize> },
    Run { command: String, span: Range<usize> },
    Copy { src: String, dest: String, span: Range<usize> },
    Add { src: String, dest: String, span: Range<usize> },
    Workdir { path: String, span: Range<usize> },
    Expose { port: String, span: Range<usize> },
    Env { key: String, value: String, span: Range<usize> },
    Cmd { command: String, span: Range<usize> },
    Entrypoint { command: String, span: Range<usize> },
    Volume { path: String, span: Range<usize> },
    User { user: String, span: Range<usize> },
    Label { key: String, value: String, span: Range<usize> },
    Arg { name: String, default: Option<String>, span: Range<usize> },
    Onbuild { instruction: Box<Instruction>, span: Range<usize> },
    Stopsignal { signal: String, span: Range<usize> },
    Healthcheck { command: String, span: Range<usize> },
    Shell { shell: String, span: Range<usize> },
}
