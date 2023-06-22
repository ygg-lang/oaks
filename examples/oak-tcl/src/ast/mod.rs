use core::range::Range;
use serde::{Deserialize, Serialize};

/// Tcl 抽象语法树根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TclRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub items: Vec<TclItem>,
}

/// Tcl 顶级项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TclItem {
    Command(TclCommand),
    Comment(TclComment),
}

/// Tcl 命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TclCommand {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub words: Vec<TclWord>,
}

/// Tcl 词 (命令的参数或命令名)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TclWord {
    Simple(String),
    Variable(String),
    Script(TclRoot),
    Braced(String),
}

/// Tcl 注释
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TclComment {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub text: String,
}

impl TclRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span, items: Vec::new() }
    }
}
