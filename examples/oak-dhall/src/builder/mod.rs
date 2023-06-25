use crate::parser::DHallParser;
#[doc = include_str!("../readme.md")]
use crate::{ast::*, language::DHallLanguage};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// DHall AST builder.
#[derive(Clone)]
pub struct DHallBuilder<'config> {
    config: &'config DHallLanguage,
}

impl<'config> DHallBuilder<'config> {
    /// Creates a new `DHallBuilder`.
    pub fn new(config: &'config DHallLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<DHallLanguage> for DHallBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<DHallLanguage>) -> BuildOutput<DHallLanguage> {
        let parser = DHallParser::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<DHallLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

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

impl<'config> DHallBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: GreenNode<DHallLanguage>, source: &SourceText) -> Result<DHallRoot, OakError> {
        let mut expressions = Vec::new();
        let mut current_offset = 0;

        for child in green_tree.children {
            match child {
                oak_core::GreenTree::Node(n) => {
                    expressions.push(self.build_expr(n, current_offset, source)?);
                    current_offset += n.byte_length as usize;
                }
                oak_core::GreenTree::Leaf(l) => {
                    current_offset += l.length as usize;
                }
            }
        }

        Ok(DHallRoot { expressions })
    }

    fn build_expr(&self, node: &GreenNode<DHallLanguage>, offset: usize, source: &SourceText) -> Result<DHallExpr, OakError> {
        let span = core::range::Range { start: offset, end: offset + node.byte_length as usize };

        match node.kind {
            crate::parser::element_type::DHallElementType::Identifier => {
                let name = source.get_text_in(span.clone()).to_string();
                Ok(DHallExpr::Identifier { name, span })
            }
            crate::parser::element_type::DHallElementType::Number | crate::parser::element_type::DHallElementType::String | crate::parser::element_type::DHallElementType::True | crate::parser::element_type::DHallElementType::False => {
                let value = source.get_text_in(span.clone()).to_string();
                Ok(DHallExpr::Literal { value, span })
            }
            _ => {
                // For other types, try to find a child node that is an expression
                for child in node.children {
                    if let oak_core::GreenTree::Node(n) = child {
                        return self.build_expr(n, offset, source);
                    }
                }
                // Fallback
                let name = source.get_text_in(span.clone()).to_string();
                Ok(DHallExpr::Identifier { name, span })
            }
        }
    }
}
