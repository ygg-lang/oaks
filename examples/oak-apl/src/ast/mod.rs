#![doc = include_str!("readme.md")]
/// Root node of the APL syntax tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AplRoot {
    /// Items in this APL file.
    pub items: Vec<AplItem>,
}

impl AplRoot {
    /// Creates a new APL root node.
    pub fn new(items: Vec<AplItem>) -> Self {
        Self { items }
    }

    /// Gets all top-level items in this APL file.
    pub fn items(&self) -> &[AplItem] {
        &self.items
    }
}

/// Item in an APL file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AplItem {
    /// Assignment.
    Assignment(AplAssignment),
    /// Expression.
    Expression(String),
}

/// APL assignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AplAssignment {
    /// Variable name.
    pub name: String,
    /// Expression.
    pub expression: String,
}

impl AplAssignment {
    /// Creates a new assignment.
    pub fn new(name: String, expression: String) -> Self {
        Self { name, expression }
    }
}
