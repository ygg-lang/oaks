use core::range::Range;

/// SQL 根节点
#[derive(Debug, Clone)]
pub struct SqlRoot {
    span: Range<usize>,
}
