use crate::{ast::TomlRoot, language::TomlLanguage};
use oak_core::{Builder, BuilderCache, OakDiagnostics, TextEdit, source::Source};

pub struct TomlBuilder<'config> {
    _config: &'config TomlLanguage,
}

impl<'config> TomlBuilder<'config> {
    pub fn new(config: &'config TomlLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Builder<TomlLanguage> for TomlBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, _source: &S, _edits: &[TextEdit], _cache: &'a mut impl BuilderCache<TomlLanguage>) -> OakDiagnostics<TomlRoot> {
        todo!("TomlBuilder::build not implemented")
    }
}
