use crate::OrgModeLanguage;
use oak_core::tree::RedNode;

pub struct OrgModeRoot<'a> {
    pub node: RedNode<'a, OrgModeLanguage>,
}

impl<'a> OrgModeRoot<'a> {
    pub fn new(node: RedNode<'a, OrgModeLanguage>) -> Self {
        Self { node }
    }
}
