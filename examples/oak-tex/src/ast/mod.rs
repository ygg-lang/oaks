use core::range::Range;
use serde::{Deserialize, Serialize};

/// TeX 抽象语法树根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TexRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub items: Vec<TexItem>,
}

/// TeX 顶级项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TexItem {
    Command(TexCommand),
    Group(TexGroup),
    Text(String),
    Comment(String),
}

/// TeX 命令 (e.g., \section, \textbf)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TexCommand {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub name: String,
    pub arguments: Vec<TexArgument>,
}

/// TeX 参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TexArgument {
    Optional(TexRoot),
    Required(TexRoot),
}

/// TeX 组 (e.g., { ... })
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TexGroup {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub content: TexRoot,
}

impl TexRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span, items: Vec::new() }
    }
}
