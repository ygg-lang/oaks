use crate::{IdlLanguage, IdlParser, ast::*};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, SourceText, TextEdit, source::Source};

pub struct IdlBuilder<'config> {
    config: &'config IdlLanguage,
}

impl<'config> IdlBuilder<'config> {
    pub fn new(config: &'config IdlLanguage) -> Self {
        Self { config }
    }

    pub fn build_root(&self, green_tree: GreenNode<IdlLanguage>, _source: &SourceText) -> Result<IdlRoot, OakError> {
        let _red_root = RedNode::new(&green_tree, 0);
        // Simplified for now
        Ok(IdlRoot { items: Vec::new() })
    }
}

impl<'config> Builder<IdlLanguage> for IdlBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<IdlLanguage>) -> OakDiagnostics<IdlRoot> {
        let parser = IdlParser::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<IdlLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree.clone(), &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
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
