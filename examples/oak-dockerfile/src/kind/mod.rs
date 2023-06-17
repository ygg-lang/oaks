/// Dockerfile 语法种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    // 操作符和分隔
    Equal,        // =
    Comma,        // ,
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )

    // 注释
    Comment,

    // 路径和文件名
    Path,

    // 特殊
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

use oak_core::SyntaxKind;

impl SyntaxKind for DockerfileSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error | Self::Eof)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Error | Self::Eof)
    }
}
