#![doc = include_str!("readme.md")]
use crate::{
    ast::*,
    language::YamlLanguage,
    lexer::token_type::YamlTokenType,
    parser::{YamlParser, element_type::YamlElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, Source, SourceText, TextEdit, builder::BuildOutput};
use std::{collections::BTreeMap, sync::Arc};

/// AST builder for YAML.
pub struct YamlBuilder<'config> {
    /// Language configuration.
    #[allow(dead_code)]
    config: &'config YamlLanguage,
}

impl<'config> YamlBuilder<'config> {
    /// Create a new instance of the YAML builder.
    pub fn new(config: &'config YamlLanguage) -> Self {
        Self { config }
    }

    fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, YamlLanguage>, source: &SourceText, cache: &mut impl BuilderCache<YamlLanguage>) -> Result<YamlRoot, OakError> {
        if let Some(cached) = cache.get_typed_node::<YamlRoot>(green_tree) {
            return Ok(cached.clone());
        }

        let mut items = Vec::new();
        let mut current_offset = 0;

        for child in green_tree.children {
            match child {
                oak_core::GreenTree::Node(n) => {
                    if n.kind == YamlElementType::Document {
                        items.push(self.build_value(n, current_offset, source, cache)?);
                    }
                    current_offset += n.byte_length as usize;
                }
                oak_core::GreenTree::Leaf(l) => {
                    current_offset += l.length as usize;
                }
            }
        }

        // If no documents were found, but we have a root node, try to build a value from the root itself
        if items.is_empty() && green_tree.kind == YamlElementType::Root {
            // For now, just treat the whole root as one value if it's not empty
            if green_tree.byte_length > 0 {
                // This is a fallback for the placeholder parser
                let span = oak_core::Range::from(0..green_tree.byte_length as usize);
                let text = source.get_text_in(span.clone()).to_string();
                items.push(YamlValueNode::Scalar(YamlScalar { value: text, span }));
            }
        }

        let result = YamlRoot { span: (0..green_tree.byte_length as usize).into(), items };
        cache.set_typed_node(green_tree, result.clone());
        Ok(result)
    }

    fn build_value<'a>(&self, node: &GreenNode<'a, YamlLanguage>, offset: usize, source: &SourceText, _cache: &mut impl BuilderCache<YamlLanguage>) -> Result<YamlValueNode, OakError> {
        let span: oak_core::Range<usize> = (offset..offset + node.byte_length as usize).into();

        // Placeholder implementation for YAML values
        let text = source.get_text_in(span.clone()).to_string();

        Ok(YamlValueNode::Scalar(YamlScalar { value: text, span }))
    }
}

impl<'config> Builder<YamlLanguage> for YamlBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<YamlLanguage>) -> BuildOutput<YamlLanguage> {
        let parser = YamlParser::new(self.config);
        let mut session = oak_core::parser::session::ParseSession::<YamlLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text, cache) {
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
