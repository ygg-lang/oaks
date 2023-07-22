#![doc = include_str!("readme.md")]

mod build_expr;
mod build_root;
mod build_stmt;
mod utils;

use crate::{MatlabLanguage, MatlabParser};
use oak_core::{Builder, BuilderCache, OakDiagnostics, Parser, TextEdit, builder::BuildOutput, source::Source};

/// Extract source text for a span.
pub(crate) fn text(source: &(impl Source + ?Sized), range: oak_core::Range<usize>) -> String {
    source.get_text_in(range).to_string()
}

/// Builds an owned [`MatlabRoot`] from the parser CST.
#[derive(Clone)]
pub struct MatlabBuilder<'config> {
    /// Language configuration.
    config: &'config MatlabLanguage,
}

impl<'config> MatlabBuilder<'config> {
    /// Creates a new builder.
    pub fn new(config: &'config MatlabLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<MatlabLanguage> for MatlabBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<MatlabLanguage>) -> BuildOutput<MatlabLanguage> {
        let parser = MatlabParser::new(self.config);
        let mut parse_cache = oak_core::parser::session::ParseSession::<MatlabLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => match self.build_root(green_tree, source) {
                Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                Err(build_error) => {
                    let mut diagnostics = parse_result.diagnostics;
                    diagnostics.push(build_error.clone());
                    OakDiagnostics { result: Err(build_error), diagnostics }
                }
            },
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
