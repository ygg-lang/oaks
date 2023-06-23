#![doc = include_str!("readme.md")]
use core::range::Range;

/// SCSS root node
#[derive(Debug, Clone)]
pub struct ScssRoot {
    _span: Range<usize>,
}
