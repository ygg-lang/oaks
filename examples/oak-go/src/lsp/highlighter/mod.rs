#![doc = include_str!("readme.md")]
//! Go 语言语法高亮器

use oak_highlight::HighlightSegment;

use crate::ast::GoRoot;

/// Go 语言语法高亮器
pub struct GoHighlighter;

impl GoHighlighter {
    pub fn highlight<'a>(&self, _root: &'a GoRoot) -> Vec<HighlightSegment<'_>> {
        vec![]
    }
}
