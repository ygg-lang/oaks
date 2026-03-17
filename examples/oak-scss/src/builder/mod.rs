use crate::{ast::ScssRoot, language::ScssLanguage};
use oak_core::{Builder, BuilderCache, Lexer, OakDiagnostics, Parser, TextEdit, builder::BuildOutput, source::Source};

/// AST builder for SCSS language
#[derive(Clone)]
pub struct ScssBuilder<'config> {
    config: &'config ScssLanguage,
}

impl<'config> ScssBuilder<'config> {
    /// Creates a new `ScssBuilder` with the given configuration.
    pub fn new(config: &'config ScssLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<ScssLanguage> for ScssBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<ScssLanguage>) -> BuildOutput<ScssLanguage> {
        let parser = crate::parser::ScssParser::new(self.config);
        let lexer = crate::lexer::ScssLexer::new(&self.config);

        let mut cache = oak_core::parser::session::ParseSession::<ScssLanguage>::default();
        lexer.lex(source, edits, &mut cache);
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(parse_tree) => {
                let ast = self.build_ast(source, &parse_tree);
                OakDiagnostics { result: Ok(ast), diagnostics: parse_result.diagnostics }
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> ScssBuilder<'config> {
    /// Builds an AST from the parse tree.
    fn build_ast<'a, S: Source + ?Sized>(&self, source: &S, _parse_tree: &oak_core::tree::GreenNode<ScssLanguage>) -> ScssRoot {
        let children = Vec::new();

        // For now, return an empty AST
        ScssRoot { span: (0..source.length()).into(), children }
    }
}
