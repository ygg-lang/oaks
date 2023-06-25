use crate::{ast::*, language::DelphiLanguage, parser::DelphiParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, SourceText, TextEdit, source::Source};

/// AST builder for the Delphi language
#[derive(Clone)]
pub struct DelphiBuilder {
    config: DelphiLanguage,
}

impl DelphiBuilder {
    /// Creates a new `DelphiBuilder`
    pub fn new(config: DelphiLanguage) -> Self {
        Self { config }
    }
}

impl Builder<DelphiLanguage> for DelphiBuilder {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<DelphiLanguage>) -> oak_core::builder::BuildOutput<DelphiLanguage> {
        let parser = DelphiParser::new(&self.config);

        let mut parse_cache = oak_core::parser::session::ParseSession::<DelphiLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
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

impl DelphiBuilder {
    fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, DelphiLanguage>, _source: &SourceText) -> Result<DelphiRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        // Simple implementation
        Ok(DelphiRoot { items: Vec::new(), span: red_root.span() })
    }
}
