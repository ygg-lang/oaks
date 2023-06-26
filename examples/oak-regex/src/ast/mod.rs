use crate::{language::RegexLanguage, parser::element_type::RegexElementType};

/// Regex root node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Root node of a regex AST.
pub struct RegexRoot;

impl RegexRoot {
    pub fn new() -> Self {
        Self
    }
}
