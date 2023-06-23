#![doc = include_str!("readme.md")]
use crate::language::StylusLanguage;
use oak_core::{RedNode, TextEdit};

pub struct StylusFormatter;

impl StylusFormatter {
    pub fn new(_config: &StylusLanguage) -> Self {
        Self
    }
}

impl StylusFormatter {
    pub fn format(&self, _root: &RedNode<StylusLanguage>) -> Vec<TextEdit> {
        vec![]
    }
}
