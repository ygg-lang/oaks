#![doc = include_str!("readme.md")]
use crate::{ast::TomlRoot, language::TomlLanguage, lexer::TomlLexer, parser::TomlParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, SourceText, TextEdit, builder::BuildOutput, source::Source};
use std::{collections::BTreeMap, sync::Arc};

/// TOML AST builder
pub struct TomlBuilder<'config> {
    config: &'config TomlLanguage,
}

impl<'config> TomlBuilder<'config> {
    /// Creates a new TOML builder
    pub fn new(config: &'config TomlLanguage) -> Self {
        Self { config }
    }

    fn build_root<'a>(&self, green_tree: &GreenNode<'a, TomlLanguage>, _source: &SourceText, cache: &mut impl BuilderCache<TomlLanguage>) -> Result<TomlRoot, OakError> {
        if let Some(cached) = cache.get_typed_node::<TomlRoot>(green_tree) {
            return Ok(cached.clone());
        }

        let span = 0..green_tree.byte_length as usize;

        // Placeholder implementation for TOML root
        let items = Vec::new();

        let result = TomlRoot { span: span.into(), items };
        cache.set_typed_node(green_tree, result.clone());
        Ok(result)
    }
}

impl<'config> Builder<TomlLanguage> for TomlBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<TomlLanguage>) -> BuildOutput<TomlLanguage> {
        let parser = TomlParser::new(self.config);
        let lexer = TomlLexer::new(self.config);

        let mut session = oak_core::parser::session::ParseSession::<TomlLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(&green_tree, &source_text, cache) {
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
