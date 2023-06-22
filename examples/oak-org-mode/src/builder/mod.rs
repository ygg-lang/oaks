use crate::language::OrgModeLanguage;
use oak_core::{Builder, BuilderCache, OakDiagnostics, TextEdit, source::Source};

pub struct OrgModeBuilder {}

impl OrgModeBuilder {
    pub fn new(_config: &OrgModeLanguage) -> Self {
        Self {}
    }
}

impl Builder<OrgModeLanguage> for OrgModeBuilder {
    fn build<'a, S: Source + ?Sized>(&self, _source: &S, _edits: &[TextEdit], _cache: &'a mut impl BuilderCache<OrgModeLanguage>) -> OakDiagnostics<()> {
        OakDiagnostics { result: Ok(()), diagnostics: Vec::new() }
    }
}
