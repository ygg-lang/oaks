#![doc = include_str!("readme.md")]
use core::range::Range;

/// Django template root node.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DjangoRoot {
    pub elements: Vec<DjangoElement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Django template element.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DjangoElement {
    /// HTML text.
    HtmlText {
        content: String,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Django variable {{ variable }}.
    Variable {
        name: String,
        filters: Vec<String>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Django tag {% tag %}.
    Tag {
        name: String,
        args: Vec<String>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Django comment {# comment #}.
    Comment {
        content: String,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}
