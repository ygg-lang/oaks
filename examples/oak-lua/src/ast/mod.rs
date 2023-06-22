use core::range::Range;
use serde::{Deserialize, Serialize};

/// Lua 根节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
