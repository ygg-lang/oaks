#![doc = include_str!("readme.md")]
use crate::language::NixLanguage;
use oak_core::tree::RedNode;

pub struct NixExpr<'a> {
    pub node: RedNode<'a, NixLanguage>,
}

impl<'a> NixExpr<'a> {
    pub fn new(node: RedNode<'a, NixLanguage>) -> Self {
        Self { node }
    }
}

pub struct NixRoot<'a> {
    pub node: RedNode<'a, NixLanguage>,
}

impl<'a> NixRoot<'a> {
    pub fn new(node: RedNode<'a, NixLanguage>) -> Self {
        Self { node }
    }
}
