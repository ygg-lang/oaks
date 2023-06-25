use crate::{D2ElementType, D2Parser, D2TokenType, ast::*, language::D2Language};
use oak_core::{
    Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree,
    builder::BuildOutput,
    source::{Source, SourceText, TextEdit},
};

/// AST builder for the D2 language.
#[derive(Clone, Copy)]
pub struct D2Builder<'config> {
    /// Language configuration.
    config: &'config D2Language,
}

impl<'config> D2Builder<'config> {
    /// Creates a new `D2Builder` with the given language configuration.
    pub fn new(config: &'config D2Language) -> Self {
        Self { config }
    }
}

impl<'config> Builder<D2Language> for D2Builder<'config> {
    /// Builds the D2 AST from the green tree.
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<D2Language>) -> BuildOutput<D2Language> {
        let parser = D2Parser::new(self.config);

        let mut cache = oak_core::parser::session::ParseSession::<D2Language>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

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

impl<'config> D2Builder<'config> {
    /// Builds the AST root from the green tree.
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, D2Language>, source: &SourceText) -> Result<D2Root, OakError> {
        let root_node = RedNode::new(green_tree, 0);
        let mut elements = Vec::new();

        for child in root_node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    D2ElementType::Shape => elements.push(D2Element::Shape(self.build_shape(n, source)?)),
                    D2ElementType::Connection => elements.push(D2Element::Connection(self.build_connection(n, source)?)),
                    _ => {}
                }
            }
        }

        Ok(D2Root { elements, span: root_node.span() })
    }

    fn build_shape(&self, node: RedNode<D2Language>, source: &SourceText) -> Result<Shape, OakError> {
        let mut id = String::new();
        let mut label = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    D2TokenType::Id => id = text(source, t.span.clone()),
                    D2TokenType::Label => label = Some(text(source, t.span.clone())),
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(Shape { id, label })
    }

    fn build_connection(&self, node: RedNode<D2Language>, source: &SourceText) -> Result<Connection, OakError> {
        let mut from = String::new();
        let mut to = String::new();

        let mut ids = Vec::new();
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == D2TokenType::Id {
                    ids.push(text(source, t.span.clone()));
                }
            }
        }

        if ids.len() >= 2 {
            from = ids[0].clone();
            to = ids[1].clone();
        }

        Ok(Connection { from, to, span: node.span() })
    }
}

fn text(source: &SourceText, span: core::range::Range<usize>) -> String {
    source.get_text_in(span).to_string()
}
