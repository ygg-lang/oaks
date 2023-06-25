#![doc = include_str!("readme.md")]
//! Objective-C AST definitions.

/// Root node of the Objective-C syntax tree.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectiveCRoot {
    /// All top-level items in the source file.
    pub items: Vec<ObjectiveCItem>,
}

/// Objective-C top-level item.
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectiveCItem {
    /// Interface definition (@interface).
    Interface,
    /// Implementation definition (@implementation).
    Implementation,
    /// Protocol definition (@protocol).
    Protocol,
    /// Function definition.
    Function,
    /// Variable declaration.
    Variable,
    /// Import statement (#import/#include).
    Import,
}

impl Default for ObjectiveCRoot {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}
