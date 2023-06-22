use crate::{ast::HaskellRoot, language::HaskellLanguage, parser::HaskellParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, RedNode, Source, SourceText, TextEdit};

/// Haskell 语言的 AST 构建器
#[derive(Clone)]
pub struct HaskellBuilder<'config> {
    config: &'config HaskellLanguage,
}

impl<'config> HaskellBuilder<'config> {
    pub fn new(config: &'config HaskellLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<HaskellLanguage> for HaskellBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<HaskellLanguage>) -> OakDiagnostics<HaskellRoot> {
        let parser = HaskellParser::new(self.config);
        let lexer = crate::lexer::HaskellLexer::new(self.config);

        let mut cache = oak_core::parser::session::ParseSession::<HaskellLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
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

impl<'config> HaskellBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: GreenNode<HaskellLanguage>, _source: &SourceText) -> Result<HaskellRoot, OakError> {
        let _red_root = RedNode::new(&green_tree, 0);
        let items = Vec::new();
        // Simplified for now
        Ok(HaskellRoot { module_name: None, items })
    }
}
