use crate::{ast::*, language::MermaidLanguage, parser::MermaidParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Source, SourceText, TextEdit};

/// A builder for Mermaid Abstract Syntax Trees.
#[derive(Clone)]
pub struct MermaidBuilder<'config> {
    config: &'config MermaidLanguage,
}

impl<'config> MermaidBuilder<'config> {
    /// Creates a new Mermaid builder with the given configuration.
    pub fn new(config: &'config MermaidLanguage) -> Self {
        Self { config }
    }

    /// Builds a Mermaid root from a green tree and source text.
    pub fn build_root(&self, green_tree: &GreenNode<MermaidLanguage>, _source: &SourceText) -> Result<MermaidRoot, oak_core::OakError> {
        // Simplified implementation, actual logic needs to recursively build AST based on GreenTree node types
        Ok(MermaidRoot { diagrams: Vec::new(), span: (0..green_tree.byte_length as usize).into() })
    }
}

impl<'config> Builder<MermaidLanguage> for MermaidBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<MermaidLanguage>) -> oak_core::builder::BuildOutput<MermaidLanguage> {
        let parser = MermaidParser::new(self.config);
        let lexer = crate::lexer::MermaidLexer::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<MermaidLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                OakDiagnostics { result: self.build_root(&green_tree, &source_text), diagnostics: parse_result.diagnostics }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
