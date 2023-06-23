#![doc = include_str!("readme.md")]
use crate::language::PascalLanguage;
use oak_core::{Builder, BuilderCache, TextEdit, source::Source};

pub struct PascalBuilder;

impl PascalBuilder {
    pub fn new(_language: &PascalLanguage) -> Self {
        Self
    }
}

impl Builder<PascalLanguage> for PascalBuilder {
    fn build<'s, S: Source + ?Sized>(&self, _text: &S, _edits: &[TextEdit], _cache: &'s mut impl BuilderCache<PascalLanguage>) -> oak_core::builder::BuildOutput<PascalLanguage> {
        todo!()
    }
}
