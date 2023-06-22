use core::range::Range;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NginxRoot {
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
}

impl NginxRoot {
    pub fn new(range: Range<usize>) -> Self {
        Self { range }
    }
}
