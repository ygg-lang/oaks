use core::range::Range;
use serde::{Deserialize, Serialize};

/// Kotlin root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KotlinRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
