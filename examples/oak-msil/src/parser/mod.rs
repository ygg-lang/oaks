#![doc = include_str!("readme.md")]

use crate::{ast::*, errors::MsilResult, kind::MsilToken};
use alloc::vec::Vec;

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

    /// 解析 MSIL 源代码文
    pub fn parse(&mut self, tokens: Vec<MsilToken>) -> MsilResult<MsilRoot> {
        self.tokens = tokens;
        self.position = 0;

        // 简化的解析实现
        Ok(MsilRoot { classes: Vec::new() })
    }
}

impl Default for MsilParser {
    fn default() -> Self {
        Self::new()
    }
}
