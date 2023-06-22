use crate::{ast::TypstRoot, language::TypstLanguage, parser::TypstParser};
use oak_core::{
    Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode,
    source::{Source, TextEdit},
};

/// Typst 语言的 AST 构建器
#[derive(Clone)]
pub struct TypstBuilder<'config> {
    config: &'config TypstLanguage,
}

impl<'config> TypstBuilder<'config> {
    pub fn new(config: &'config TypstLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<TypstLanguage> for TypstBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<TypstLanguage>) -> OakDiagnostics<TypstRoot> {
        let parser = TypstParser::new(self.config);

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

impl<'config> TypstBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: &GreenNode<TypstLanguage>) -> Result<TypstRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        Ok(TypstRoot::new(red_root.span()))
    }
}
