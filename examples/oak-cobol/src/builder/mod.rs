use crate::{ast::CobolRoot, language::CobolLanguage};
use oak_core::{
    GreenNode, OakDiagnostics, Range, SourceText,
    builder::{BuildOutput, Builder, BuilderCache},
    source::{Source, TextEdit},
};

/// COBOL builder.
pub struct CobolBuilder;

impl CobolBuilder {
    /// Create a new COBOL builder.
    pub fn new() -> Self {
        Self
    }
}

impl Builder<CobolLanguage> for CobolBuilder {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<CobolLanguage>) -> BuildOutput<CobolLanguage> {
        let parser = crate::parser::CobolParser::new();
        let lexer = crate::lexer::CobolLexer::new();

        let mut session = oak_core::parser::session::ParseSession::<CobolLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut session);

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

impl CobolBuilder {
    pub(crate) fn build_root(&self, green_tree: GreenNode<CobolLanguage>, _source: &SourceText) -> Result<CobolRoot, oak_core::OakError> {
        let span = Range { start: 0, end: green_tree.byte_length as usize };
        Ok(CobolRoot { span })
    }
}
