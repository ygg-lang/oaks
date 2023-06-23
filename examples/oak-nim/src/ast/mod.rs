#![doc = include_str!("readme.md")]
use crate::language::NimLanguage;
use oak_core::tree::RedNode;

pub struct NimModule<'a> {
    pub node: RedNode<'a, NimLanguage>,
}

impl<'a> NimModule<'a> {
    pub fn new(node: RedNode<'a, NimLanguage>) -> Self {
        Self { node }
    }
}
