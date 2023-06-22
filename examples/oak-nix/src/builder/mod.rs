use crate::language::NixLanguage;
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, SourceText, TextEdit, builder::BuildOutput, source::Source};

#[derive(Clone)]
pub struct NixBuilder<'config> {
    config: &'config NixLanguage,
}

impl<'config> NixBuilder<'config> {
    pub fn new(config: &'config NixLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<NixLanguage> for NixBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<NixLanguage>) -> BuildOutput<NixLanguage> {
        let parser = crate::parser::NixParser::new(self.config);
        let lexer = crate::lexer::NixLexer::new(&self.config);

        let mut session = oak_core::parser::session::ParseSession::<NixLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree.clone(), &source_text) {
                    Ok(_) => OakDiagnostics { result: Ok(()), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> NixBuilder<'config> {
    pub(crate) fn build_root(&self, _green_tree: GreenNode<NixLanguage>, _source: &SourceText) -> Result<(), OakError> {
        Ok(())
    }
}
