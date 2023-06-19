use core::range::Range;

/// Scala 根节点
#[derive(Debug, Clone)]
pub struct ScalaRoot {
    span: Range<usize>,
}
