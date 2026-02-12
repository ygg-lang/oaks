use crate::{language::CobolLanguage, parser::CobolElementType};
use oak_core::tree::{RedLeaf, RedNode};

pub type CobolNode<'a> = RedNode<'a, CobolLanguage>;
pub type CobolToken = RedLeaf<CobolLanguage>;

/// COBOL root node.
#[derive(Debug, Clone, Copy)]
pub struct CobolRoot<'a> {
    syntax: CobolNode<'a>,
}

impl<'a> CobolRoot<'a> {
    pub fn cast(node: CobolNode<'a>) -> Option<Self> {
        if node.green.kind == CobolElementType::Root { Some(CobolRoot { syntax: node }) } else { None }
    }

    pub fn syntax(&self) -> &CobolNode<'a> {
        &self.syntax
    }
}
