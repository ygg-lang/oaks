use core::range::Range;
use serde::{Deserialize, Serialize};

/// HTML 属性
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// HTML 节点
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HtmlNode {
    Element(Element),
    Text(Text),
    Comment(String),
}

/// HTML 元素
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Element {
    pub tag_name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<HtmlNode>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// HTML 文本
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Text {
    pub content: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// HTML 文档根节点
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HtmlDocument {
    pub nodes: Vec<HtmlNode>,
}
