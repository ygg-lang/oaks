use core::range::Range;
use serde::{Deserialize, Serialize};

/// Stylus 文档根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylusRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub items: Vec<StylusItem>,
}

/// Stylus 顶级项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StylusItem {
    Rule(StylusRule),
    Comment(StylusComment),
}

/// Stylus 规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylusRule {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub selector: String,
    pub properties: Vec<StylusProperty>,
}

/// Stylus 注释
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylusComment {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub text: String,
}

/// Stylus 属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylusProperty {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub name: String,
    pub value: String,
}

impl StylusRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span, items: Vec::new() }
    }
}
