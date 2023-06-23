#![doc = include_str!("readme.md")]
use crate::language::GsglLanguage;
use oak_core::{Builder, BuilderCache, Source, TextEdit, builder::BuildOutput};

pub struct GsglBuilder {}

impl GsglBuilder {
    pub fn new(_lang: &GsglLanguage) -> Self {
        Self {}
    }
}

impl Builder<GsglLanguage> for GsglBuilder {
    fn build<'a, S: Source + ?Sized>(&self, _text: &S, _edits: &[TextEdit], _cache: &'a mut impl BuilderCache<GsglLanguage>) -> BuildOutput<GsglLanguage> {
        todo!()
    }
}
