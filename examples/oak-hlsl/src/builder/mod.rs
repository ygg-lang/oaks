use crate::{ast::*, language::HlslLanguage, parser::HlslParser};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// HLSL 语言的 AST 构建器
pub struct HlslBuilder<'config> {
    config: &'config HlslLanguage,
}

impl<'config> HlslBuilder<'config> {
    pub fn new(config: &'config HlslLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<HlslLanguage> for HlslBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<HlslLanguage>) -> oak_core::builder::BuildOutput<HlslLanguage> {
        let parser = HlslParser::new(self.config);
        let lexer = crate::lexer::HlslLexer::new(&self.config);

        // For now, we don't have a proper incremental builder implementation here
        // We just run the lexer and parser
        let mut lexer_cache = oak_core::parser::session::ParseSession::<HlslLanguage>::default();
        let _lex_output = lexer.lex(text, edits, &mut lexer_cache);

        let mut parser_cache = oak_core::parser::session::ParseSession::<HlslLanguage>::default();
        let parse_result = parser.parse(text, edits, &mut parser_cache);

        let OakDiagnostics { result, diagnostics } = parse_result;

        match result {
            Ok(green_tree) => {
                let source_text = SourceText::new(text.get_text_in((0..text.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics },
        }
    }
}

impl<'config> HlslBuilder<'config> {
    fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, HlslLanguage>, _source: &SourceText) -> Result<HlslRoot, OakError> {
        let red_root = RedNode::<HlslLanguage>::new(green_tree, 0);
        let declarations = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(_n) = child {
                // HLSL parser logic would go here
                // For now, just a placeholder
            }
        }

        Ok(HlslRoot { declarations })
    }
}
