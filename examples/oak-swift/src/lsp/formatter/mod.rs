#![doc = include_str!("readme.md")]
use crate::language::SwiftLanguage;
use oak_core::RedNode;

pub struct SwiftFormatter<'config> {
    _config: &'config SwiftLanguage,
}

impl<'config> SwiftFormatter<'config> {
    pub fn new(config: &'config SwiftLanguage) -> Self {
        Self { _config: config }
    }

    pub fn format(&self, _root: &RedNode<SwiftLanguage>) -> String {
        String::new()
    }
}
