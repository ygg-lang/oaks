#![doc = include_str!("readme.md")]
//! JavaScript 格式化器
use crate::{ast::JavaScriptRoot, language::JavaScriptLanguage};
use oak_core::{BuilderCache, OakDiagnostics, source::Source};
use oak_pretty_print::FormatConfig;

pub struct JavaScriptFormatter;

impl JavaScriptFormatter {
    pub fn format(&self, _root: &JavaScriptRoot, _text: impl Source, _config: &FormatConfig, _cache: impl BuilderCache<JavaScriptLanguage>) -> OakDiagnostics<String> {
        OakDiagnostics { result: Ok(String::new()), diagnostics: vec![] }
    }
}
