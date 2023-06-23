#![doc = include_str!("readme.md")]
use crate::{ast::JavadocRoot, language::JavadocLanguage};
use oak_core::builder::Builder;

pub struct JavadocBuilder {
    pub config: JavadocLanguage,
}

impl Default for JavadocBuilder {
    fn default() -> Self {
        Self { config: JavadocLanguage::default() }
    }
}

impl Builder<JavadocLanguage> for JavadocBuilder {
    fn build<'a, S: oak_core::source::Source + ?Sized>(&self, _source: &S, _edits: &[oak_core::source::TextEdit], _cache: &'a mut impl oak_core::builder::BuilderCache<JavadocLanguage>) -> oak_core::builder::BuildOutput<JavadocLanguage> {
        oak_core::errors::OakDiagnostics { result: Ok(JavadocRoot { description: vec![], tags: vec![] }), diagnostics: vec![] }
    }
}
