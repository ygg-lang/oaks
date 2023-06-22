use crate::{ast::*, language::JasmLanguage, parser::JasmParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, TextEdit, source::Source};

/// JASM 语言的 AST 构建器
#[derive(Clone)]
pub struct JasmBuilder<'config> {
    config: &'config JasmLanguage,
}

impl<'config> JasmBuilder<'config> {
    pub fn new(config: &'config JasmLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<JasmLanguage> for JasmBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<JasmLanguage>) -> OakDiagnostics<JasmRoot> {
        let parser = JasmParser::new(self.config);

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

impl<'config> JasmBuilder<'config> {
    fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, JasmLanguage>) -> Result<JasmRoot, OakError> {
        let _red_root = RedNode::new(green_tree, 0);
        // Simplified for example
        Ok(JasmRoot { class: JasmClass { modifiers: vec![], name: String::new(), version: None, methods: vec![], fields: vec![], source_file: None } })
    }
}
