use crate::{ast::LlirRoot, language::LLvmLanguage, parser::LlirParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, TextEdit, source::Source};

pub struct LlirBuilder<'config> {
    config: &'config LLvmLanguage,
}

impl<'config> LlirBuilder<'config> {
    pub fn new(config: &'config LLvmLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<LLvmLanguage> for LlirBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<LLvmLanguage>) -> OakDiagnostics<LlirRoot> {
        let parser = LlirParser::new(self.config);
        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(green_tree) => match self.build_root(green_tree) {
                Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                Err(build_error) => {
                    let mut diagnostics = parse_result.diagnostics;
                    diagnostics.push(build_error.clone());
                    OakDiagnostics { result: Err(build_error), diagnostics }
                }
            },
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> LlirBuilder<'config> {
    fn build_root(&self, green_tree: &GreenNode<LLvmLanguage>) -> Result<LlirRoot, OakError> {
        Ok(LlirRoot { span: (0..green_tree.text_len() as usize).into() })
    }
}
