use core::range::Range;

/// JSON 值节
#[derive(Debug, Clone)]
pub struct JsonRoot {
    span: Range<usize>,
}
