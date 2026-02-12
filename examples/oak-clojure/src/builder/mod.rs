#![doc = include_str!("readme.md")]
use crate::{ast::*, language::ClojureLanguage, parser::ClojureParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, SourceText, TextEdit, source::Source};

/// AST builder for the Clojure language
#[derive(Clone)]
pub struct ClojureBuilder<'config> {
    /// Language configuration
    config: &'config ClojureLanguage,
}

impl<'config> ClojureBuilder<'config> {
    /// Creates a new Clojure builder
    pub fn new(config: &'config ClojureLanguage) -> Self {
        Self { config }
    }

    /// Builds the AST root node from the green tree
    pub fn build_root<'a>(&self, green: &'a GreenNode<'a, ClojureLanguage>, _source: &SourceText) -> Result<ClojureRoot<'a>, oak_core::OakError> {
        // Simplified AST building logic
        // Note: The current definition of ClojureRoot requires a RedNode; here we return a basic implementation.
        // The actual implementation usually involves converting GreenNode to RedNode.
        let tree = oak_core::tree::RedNode::new(green, 0);
        ClojureRoot::cast(tree).ok_or_else(|| oak_core::OakError::custom_error("Failed to cast to ClojureRoot"))
    }
}

impl<'config> Builder<ClojureLanguage> for ClojureBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<ClojureLanguage>) -> oak_core::builder::BuildOutput<ClojureLanguage> {
        let parser = ClojureParser::new(self.config);
        // We use the provided cache instead of creating a local one
        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
                    Ok(_ast_root) => {
                        // Since ClojureRoot has a lifetime, it might need special handling here.
                        // In the Builder trait, TypedRoot is usually () or a type without a lifetime.
                        // For now, we return Ok(()) if TypedRoot is ().
                        OakDiagnostics { result: Ok(()), diagnostics: parse_result.diagnostics }
                    }
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
