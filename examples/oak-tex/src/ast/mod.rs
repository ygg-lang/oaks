#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// TeX 抽象语法树根节点
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TexRoot {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    pub items: Vec<TexItem>,
}

/// TeX 顶级项目
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TexItem {
    Command(TexCommand),
    Environment(TexEnvironment),
    Group(TexGroup),
    Math(TexMath),
    Superscript(TexSuperscript),
    Subscript(TexSubscript),
    Text {
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
        content: String,
    },
    Comment {
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
        content: String,
    },
}

/// TeX 环境 (e.g., \begin{matrix} ... \end{matrix})
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TexEnvironment {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    pub name: String,
    pub arguments: Vec<TexArgument>,
    pub content: TexRoot,
}

/// TeX 上标
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TexSuperscript {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    pub target: Option<Box<TexItem>>,
    pub content: Box<TexRoot>,
}

/// TeX 下标
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TexSubscript {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    pub target: Option<Box<TexItem>>,
    pub content: Box<TexRoot>,
}

/// TeX 数学环境 ($...$ 或 $$...$$)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TexMath {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    pub content: TexRoot,
    pub is_display: bool,
}

/// TeX 命令 (e.g., \section, \textbf)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TexCommand {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    pub name: String,
    pub arguments: Vec<TexArgument>,
}

/// TeX 参数
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TexArgument {
    Optional(TexRoot),
    Required(TexRoot),
}

/// TeX 组 (e.g., { ... })
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TexGroup {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    pub content: TexRoot,
}

impl TexRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span, items: Vec::new() }
    }
}
