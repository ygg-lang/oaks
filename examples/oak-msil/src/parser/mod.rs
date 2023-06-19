#![doc = include_str!("readme.md")]

use crate::{ast::*, kind::MsilToken};

/// MSIL 解析
#[derive(Clone, Debug)]
pub struct MsilParser {
    /// 当前解析位置
    position: usize,
    /// Token 列表
    tokens: Vec<MsilToken>,
}

impl MsilParser {
    /// 创建新的 MSIL 解析
    pub fn new() -> Self {
        Self { position: 0, tokens: Vec::new() }
    }
}

impl Default for MsilParser {
    fn default() -> Self {
        Self::new()
    }
}
