use core::range::Range;
use serde::{Deserialize, Serialize};

/// Typst AST 根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypstRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub items: Vec<TypstItem>,
}

impl TypstRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span, items: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypstItem {
    Text(String),
    Space,
    Parbreak,
    Heading(TypstHeading),
    Strong(TypstRoot),
    Emphasis(TypstRoot),
    Math(TypstRoot),
    Quote(TypstRoot),
    ListItem(TypstRoot),
    EnumItem(TypstRoot),
    Link(TypstLink),
    Raw(String),
    Block(TypstRoot),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypstHeading {
    pub level: usize,
    pub content: TypstRoot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypstLink {
    pub url: String,
    pub content: Option<TypstRoot>,
}
