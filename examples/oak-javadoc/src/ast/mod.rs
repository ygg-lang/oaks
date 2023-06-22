#![doc = include_str!("readme.md")]

use serde::{Deserialize, Serialize};

/// Javadoc 根节点
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JavadocRoot {
    pub description: Vec<JavadocItem>,
    pub tags: Vec<JavadocBlockTag>,
}

/// Javadoc 内容项（文本或内联标签）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum JavadocItem {
    Text(String),
    InlineTag(JavadocInlineTag),
}

/// Javadoc 内联标签（如 {@link ...}）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JavadocInlineTag {
    pub tag: String,
    pub content: String,
}

/// Javadoc 块标签（如 @param, @return）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JavadocBlockTag {
    pub tag: String,
    pub content: Vec<JavadocItem>,
}
