use crate::{language::ClojureLanguage, parser::ClojureElementType};
use oak_core::tree::{RedLeaf, RedNode};

pub type ClojureNode<'a> = RedNode<'a, ClojureLanguage>;
pub type ClojureToken = RedLeaf<ClojureLanguage>;

#[derive(Debug, Clone, Copy)]
pub struct ClojureRoot<'a> {
    syntax: ClojureNode<'a>,
}

impl<'a> ClojureRoot<'a> {
    pub fn cast(node: ClojureNode<'a>) -> Option<Self> {
        if node.green.kind == ClojureElementType::SourceFile { Some(ClojureRoot { syntax: node }) } else { None }
    }

    pub fn syntax(&self) -> &ClojureNode<'a> {
        &self.syntax
    }
}
