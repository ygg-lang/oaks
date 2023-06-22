use crate::{ast::JavaRoot, language::JavaLanguage, parser::JavaParser};
use oak_core::{
    Parser,
    builder::{Builder, BuilderCache},
    source::{Source, TextEdit},
};

pub struct JavaBuilder {
    language: JavaLanguage,
}

impl JavaBuilder {
    pub fn new(language: JavaLanguage) -> Self {
        Self { language }
    }
}

impl Builder<JavaLanguage> for JavaBuilder {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<JavaLanguage>) -> oak_core::builder::BuildOutput<JavaLanguage> {
        let parser = JavaParser::new(&self.language);
        let mut cache = oak_core::parser::session::ParseSession::<JavaLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(_green_tree) => oak_core::errors::OakDiagnostics { result: Ok(JavaRoot { items: vec![] }), diagnostics: parse_result.diagnostics },
            Err(e) => oak_core::errors::OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
