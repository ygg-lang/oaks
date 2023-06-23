#![doc = include_str!("readme.md")]
use crate::{ast::JavaScriptRoot, language::JavaScriptLanguage};
use oak_core::{
    Builder, BuilderCache,
    builder::BuildOutput,
    source::{Source, TextEdit},
}

pub struct JavaScriptBuilder;

impl JavaScriptBuilder {
    pub fn new(_language: JavaScriptLanguage) -> Self {
        Self
    }
}

impl Builder<JavaScriptLanguage> for JavaScriptBuilder {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], _cache: &'a mut impl BuilderCache<JavaScriptLanguage>) -> BuildOutput<JavaScriptLanguage> {
        oak_core::errors::OakDiagnostics { result: Ok(JavaScriptRoot { span: (0..source.length()).into() }), diagnostics: vec![] }
    }
}