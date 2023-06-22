use crate::{ast::KotlinRoot, language::KotlinLanguage, lexer::KotlinLexer, parser::KotlinParser};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, RedNode, SourceText, TextEdit, source::Source};

pub struct KotlinBuilder<'config> {
    config: &'config KotlinLanguage,
}

impl<'config> KotlinBuilder<'config> {
    pub fn new(config: &'config KotlinLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<KotlinLanguage> for KotlinBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<KotlinLanguage>) -> OakDiagnostics<KotlinRoot> {
        let parser = KotlinParser::new(self.config);
        let lexer = KotlinLexer::new(self.config);

        let mut session = oak_core::parser::session::ParseSession::<KotlinLanguage>::default();
        lexer.lex(source, edits, &mut session);
        let parse_result = parser.parse(source, edits, &mut session);

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

impl<'config> KotlinBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: GreenNode<KotlinLanguage>, _source: &SourceText) -> Result<KotlinRoot, OakError> {
        let _red_root = RedNode::new(&green_tree, 0);
        // TODO: Map RedNode to KotlinRoot
        Ok(KotlinRoot { span: (0.._source.length()).into() })
    }
}
