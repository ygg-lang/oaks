#![doc = include_str!("readme.md")]
/// Root node of the J syntax tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JRoot {
    /// Items in this J file.
    pub items: Vec<JItem>,
}

impl JRoot {
    /// Creates a new J root node.
    pub fn new(items: Vec<JItem>) -> Self {
        Self { items }
    }

    /// Gets all top-level items in this J file.
    pub fn items(&self) -> &[JItem] {
        &self.items
    }
}

/// Item in an J file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JItem {
    /// A sentence (the basic unit in J).
    Sentence(JSentence),
}

/// J sentence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JSentence {
    /// Content of the sentence.
    pub content: String,
}

impl JSentence {
    /// Creates a new sentence.
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

/// J assignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JAssignment {
    /// Name being assigned to.
    pub name: String,
    /// Whether it's a global assignment (=:).
    pub is_global: bool,
    /// The expression being assigned.
    pub expression: String,
}
