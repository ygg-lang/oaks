use crate::{ClojureLanguage, ClojureSyntaxKind};
use oak_core::tree::{RedLeaf, RedNode};

pub type ClojureNode = RedNode<ClojureSyntaxKind>;
pub type ClojureToken = RedLeaf<ClojureSyntaxKind>;

#[derive(Debug, Clone)]
pub struct ClojureRoot {
    syntax: ClojureNode,
}

impl ClojureRoot {
    pub fn cast(node: ClojureNode) -> Option<Self> {
        if node.green.kind == ClojureSyntaxKind::SourceFile { Some(ClojureRoot { syntax: node }) } else { None }
    }

    pub fn syntax(&self) -> &ClojureNode {
        &self.syntax
    }
}
