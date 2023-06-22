#![doc = include_str!("readme.md")]

use crate::ast::{ActionScriptItem, ActionScriptRoot};

/// ActionScript Code Formatter
pub struct ActionScriptFormatter {
    /// 缩进级别
    _indent_level: usize,
    /// 缩进字符串
    _indent_str: String,
}

impl ActionScriptFormatter {
    /// Create a new ActionScript formatter
    pub fn new() -> Self {
        Self { _indent_level: 0, _indent_str: "    ".to_string() }
    }

    /// Format the given ActionScript source code string
    pub fn format(&self, source: &str) -> String {
        // Basic implementation for now
        source.to_string()
    }

    /// Format ActionScript AST root node
    pub fn format_ast(&self, root: &ActionScriptRoot) -> String {
        let mut result = String::new();

        for (i, item) in root.items.iter().enumerate() {
            if i > 0 {
                result.push_str("\n\n");
            }
            result.push_str(&self.format_item(item));
        }

        result
    }

    /// Format top-level items
    fn format_item(&self, item: &ActionScriptItem) -> String {
        match item {
            ActionScriptItem::Class => "class {} // TODO".to_string(),
            ActionScriptItem::Interface => "interface {} // TODO".to_string(),
            ActionScriptItem::Function => "function() {} // TODO".to_string(),
            ActionScriptItem::Variable => "var x; // TODO".to_string(),
            ActionScriptItem::Package => "package {} // TODO".to_string(),
            ActionScriptItem::Import => "import {} // TODO".to_string(),
        }
    }

    fn _get_indent(&self) -> String {
        self._indent_str.repeat(self._indent_level)
    }
}

impl Default for ActionScriptFormatter {
    fn default() -> Self {
        Self::new()
    }
}
