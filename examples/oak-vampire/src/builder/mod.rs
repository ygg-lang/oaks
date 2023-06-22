use crate::{ast::*, language::VampireLanguage, parser::VampireParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, TextEdit, source::Source};

/// Vampire 语言的 AST 构建器
#[derive(Clone)]
pub struct VampireBuilder<'config> {
    config: &'config VampireLanguage,
}

impl<'config> VampireBuilder<'config> {
    pub fn new(config: &'config VampireLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<VampireLanguage> for VampireBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<VampireLanguage>) -> oak_core::builder::BuildOutput<VampireLanguage> {
        let parser = VampireParser::new(self.config);

        let parse_result = parser.parse(text, edits, cache);

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

impl<'config> VampireBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: &GreenNode<VampireLanguage>) -> Result<VampireRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let span = red_root.span();
        Ok(VampireRoot { span: span.into(), formulas: Vec::new() })
    }
}
