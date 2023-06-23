#![doc = include_str!("readme.md")]
use crate::language::DotLanguage;
use oak_core::{Builder, BuilderCache, Source, TextEdit, builder::BuildOutput};

pub struct DotBuilder {}

impl DotBuilder {
    pub fn new(_lang: &DotLanguage) -> Self {
        Self {}
    }
}

impl Builder<DotLanguage> for DotBuilder {
    fn build<'a, S: Source + ?Sized>(&self, _text: &S, _edits: &[TextEdit], _cache: &'a mut impl BuilderCache<DotLanguage>) -> BuildOutput<DotLanguage> {
        todo!()
    }
}
