use crate::{ast::LeanRoot, language::LeanLanguage};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, RedNode, TextEdit, source::Source};

pub struct LeanBuilder<'config> {
    config: &'config LeanLanguage,
}

impl<'config> LeanBuilder<'config> {
    pub fn new(config: &'config LeanLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<LeanLanguage> for LeanBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<LeanLanguage>) -> OakDiagnostics<LeanRoot> {
        let parser = crate::parser::LeanParser::new(self.config);
        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(green_tree) => {
                let ast_root = self.build_root(green_tree);
                OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> LeanBuilder<'config> {
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, LeanLanguage>) -> LeanRoot {
        LeanRoot::new(RedNode::new(green_tree, 0).span())
    }
}
