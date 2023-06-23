#![doc = include_str!("readme.md")]
//! Go 语言格式化器

use oak_pretty_print::Document;

/// 格式化器 trait
#[allow(dead_code)]
pub trait Formatter {
    /// 格式化给定的 AST
    fn format(&self, code: &str) -> Document<'_>;
}

/// Go 语言格式化器
pub struct GoFormatter;

impl GoFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Formatter for GoFormatter {
    fn format(&self, _text: &str) -> Document<'_> {
        Document::Nil
    }
}
