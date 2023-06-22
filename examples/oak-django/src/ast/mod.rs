use core::range::Range;
use serde::{Deserialize, Serialize};

/// Django 模板根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DjangoRoot {
    pub elements: Vec<DjangoElement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Django 模板元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DjangoElement {
    /// HTML 文本
    HtmlText {
        content: String,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// Django 变量 {{ variable }}
    Variable {
        name: String,
        filters: Vec<String>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// Django 标签 {% tag %}
    Tag {
        name: String,
        args: Vec<String>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// Django 注释 {# comment #}
    Comment {
        content: String,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}
