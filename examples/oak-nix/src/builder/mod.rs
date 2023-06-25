use crate::{ast::*, language::NixLanguage, parser::element_type::NixElementType};
use oak_core::{Builder, BuilderCache, GreenNode, GreenTree, OakDiagnostics, OakError, Range, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// Builder for Nix language structures.
#[derive(Clone)]
pub struct NixBuilder<'config> {
    config: &'config NixLanguage,
}

impl<'config> NixBuilder<'config> {
    /// Creates a new `NixBuilder` with the given configuration.
    pub fn new(config: &'config NixLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<NixLanguage> for NixBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<NixLanguage>) -> BuildOutput<NixLanguage> {
        let parser = crate::parser::NixParser::new(self.config);
        let lexer = crate::lexer::NixLexer::new(&self.config);

        let mut session = oak_core::parser::session::ParseSession::<NixLanguage>::default();
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

impl<'config> NixBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: GreenNode<NixLanguage>, source: &SourceText) -> Result<NixRoot, OakError> {
        let expr = self.build_expr(&green_tree, 0, source)?;
        Ok(NixRoot { expr })
    }

    fn build_expr(&self, node: &GreenNode<NixLanguage>, offset: usize, source: &SourceText) -> Result<NixExpr, OakError> {
        let span = Range { start: offset, end: offset + node.byte_length as usize };

        match node.kind {
            NixElementType::Number => {
                let text = source.get_text_in(span.clone());
                let val = text.parse::<f64>().unwrap_or(0.0);
                Ok(NixExpr::Literal(NixLiteral::Number(val, span)))
            }
            NixElementType::Boolean | NixElementType::True | NixElementType::False => {
                let text = source.get_text_in(span.clone());
                let val = text == "true";
                Ok(NixExpr::Literal(NixLiteral::Boolean(val, span)))
            }
            NixElementType::Null => Ok(NixExpr::Literal(NixLiteral::Null(span))),
            NixElementType::Identifier => {
                let name = source.get_text_in(span.clone()).to_string();
                Ok(NixExpr::Identifier(NixIdentifier { name, span }))
            }
            NixElementType::String => {
                let text = source.get_text_in(span.clone()).to_string();
                Ok(NixExpr::Literal(NixLiteral::String(text, span)))
            }
            _ => {
                // For other types, try to find a child node that is an expression
                for child in node.children {
                    if let GreenTree::Node(n) = child {
                        return self.build_expr(n, offset, source);
                    }
                }
                // Fallback to identifier if nothing else matches (this is very basic)
                let name = source.get_text_in(span.clone()).to_string();
                Ok(NixExpr::Identifier(NixIdentifier { name, span }))
            }
        }
    }
}
