#![doc = include_str!("readme.md")]
use crate::ast::DjangoRoot;

pub struct DjangoFormatter {
    indent_str: String,
}

impl DjangoFormatter {
    pub fn new() -> Self {
        Self { indent_str: "    ".to_string() }
    }

    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }

    pub fn format_ast(&self, _root: &DjangoRoot) -> String {
        String::new()
    }
}
