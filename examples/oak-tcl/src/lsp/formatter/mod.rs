#![doc = include_str!("readme.md")]
//! Tcl 代码格式化器

use crate::ast::TclRoot;

/// Tcl 代码格式化器
pub struct TclFormatter {
    /// 缩进级别
    pub indent_level: usize,
    /// 缩进字符串
    pub indent_str: String,
}

impl TclFormatter {
    /// 创建一个新的 Tcl 格式化器
    pub fn new() -> Self {
        Self { indent_level: 0, indent_str: "    ".to_string() }
    }

    /// 格式化给定的 Tcl 源代码字符串
    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }

    /// 格式化 Tcl AST 根节点
    pub fn format_ast(&self, _root: &TclRoot) -> String {
        String::new()
    }
}

impl Default for TclFormatter {
    fn default() -> Self {
        Self::new()
    }
}
