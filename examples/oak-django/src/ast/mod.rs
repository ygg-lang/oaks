#![doc = include_str!("readme.md")]
use core::range::Range;

/// Django template root node.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DjangoRoot {
    /// Child elements in the template.
    pub elements: Vec<DjangoElement>,
    /// Source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Django template element.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DjangoElement {
    /// HTML text content outside Django tags.
    HtmlText {
        /// Raw HTML content.
        content: String,
        /// Source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Django variable expression `{{ variable }}`.
    Variable {
        /// Variable name.
        name: String,
        /// Filter chain applied to the variable.
        filters: Vec<String>,
        /// Source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Django tag `{% tag %}`.
    Tag {
        /// Tag name.
        name: String,
        /// Tag arguments.
        args: Vec<String>,
        /// Source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Django comment `{# comment #}`.
    Comment {
        /// Comment content.
        content: String,
        /// Source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}
