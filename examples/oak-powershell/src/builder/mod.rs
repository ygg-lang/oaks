//! PowerShell AST builder

use crate::{ast::*, language::PowerShellLanguage, parser::PowerShellParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, RedNode, SourceText, TextEdit, source::Source};

pub struct PowerShellBuilder<'config> {
    config: &'config PowerShellLanguage,
}

impl<'config> PowerShellBuilder<'config> {
    pub fn new(config: &'config PowerShellLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, PowerShellLanguage>, _source: &SourceText) -> Result<PowerShellRoot, oak_core::OakError> {
        let _red_root = RedNode::new(green_tree, 0);
        // Minimal implementation
        Ok(PowerShellRoot { items: Vec::new() })
    }
}

impl<'config> Builder<PowerShellLanguage> for PowerShellBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<PowerShellLanguage>) -> OakDiagnostics<PowerShellRoot> {
        let parser = PowerShellParser::new(self.config);

        let mut parse_cache = oak_core::parser::session::ParseSession::<PowerShellLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
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
