use core::range::Range;

/// Sass AST 根节点
#[derive(Debug, Clone)]
pub struct SassRoot {
    span: Range<usize>,
}
