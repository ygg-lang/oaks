use crate::{
    ast::*,
    language::DartLanguage,
    lexer::token_type::DartTokenType,
    parser::{DartParser, element_type::DartElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// AST builder for the Dart language.
#[derive(Clone)]
pub struct DartBuilder<'config> {
    config: &'config DartLanguage,
}

impl<'config> DartBuilder<'config> {
    /// Creates a new DartBuilder with the given language configuration.
    pub fn new(config: &'config DartLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, DartLanguage>, source: &SourceText) -> Result<DartRoot, oak_core::OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut items = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    DartElementType::ClassDeclaration => {
                        if let Some(item) = self.build_class(&n, source) {
                            items.push(Item::Class(item))
                        }
                    }
                    DartElementType::FunctionDeclaration => {
                        if let Some(item) = self.build_function(&n, source) {
                            items.push(Item::Function(item))
                        }
                    }
                    DartElementType::VariableDeclaration => {
                        if let Some(item) = self.build_variable(&n, source) {
                            items.push(Item::Variable(item))
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(DartRoot { items })
    }

    fn build_class(&self, node: &RedNode<DartLanguage>, source: &SourceText) -> Option<ClassDeclaration> {
        let name = self.find_identifier(node, source)?;
        let mut body = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    DartElementType::FunctionDeclaration => {
                        if let Some(f) = self.build_function(&n, source) {
                            body.push(Item::Function(f));
                        }
                    }
                    DartElementType::VariableDeclaration => {
                        if let Some(v) = self.build_variable(&n, source) {
                            body.push(Item::Variable(v));
                        }
                    }
                    _ => {}
                }
            }
        }
        Some(ClassDeclaration { name, body, span: node.span().into() })
    }

    fn build_function(&self, node: &RedNode<DartLanguage>, source: &SourceText) -> Option<FunctionDeclaration> {
        let name = self.find_identifier(node, source)?;
        let mut parameters = Vec::new();
        let mut body = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    DartElementType::VariableDeclaration => {
                        // Dart functions might have variable declarations in parameters or body
                        // This is a simplified approach
                    }
                    _ => {}
                }
            }
        }

        Some(FunctionDeclaration { name, parameters, body, span: node.span().into() })
    }

    fn build_variable(&self, node: &RedNode<DartLanguage>, source: &SourceText) -> Option<VariableDeclaration> {
        let name = self.find_identifier(node, source)?;
        let mut type_annotation = None;
        let mut value = None;

        let mut found_colon = false;
        for child in node.children() {
            match child {
                RedTree::Leaf(t) if t.kind == DartTokenType::Colon => found_colon = true,
                RedTree::Leaf(t) if t.kind == DartTokenType::Identifier && found_colon && type_annotation.is_none() => {
                    type_annotation = Some(Identifier { name: source.get_text_in(t.span.into()).to_string(), span: t.span.into() });
                }
                RedTree::Node(n) if n.green.kind == DartElementType::IntegerLiteral || n.green.kind == DartElementType::StringLiteral => {
                    value = Some(Expression::Literal(source.get_text_in(n.span().into()).to_string()));
                }
                _ => {}
            }
        }

        Some(VariableDeclaration { name, type_annotation, value, span: node.span().into() })
    }

    fn find_identifier(&self, node: &RedNode<DartLanguage>, source: &SourceText) -> Option<Identifier> {
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == DartTokenType::Identifier {
                        let range = t.span;
                        let name = source.get_text_in(range.into()).to_string();
                        if name.is_empty() {
                            continue;
                        }
                        return Some(Identifier { name, span: range.into() });
                    }
                }
                RedTree::Node(n) => {
                    if n.green.kind == DartElementType::Identifier {
                        let range = n.span();
                        let name = source.get_text_in(range.into()).to_string();
                        if name.is_empty() {
                            continue;
                        }
                        return Some(Identifier { name, span: range.into() });
                    }
                }
            }
        }
        None
    }
}

impl<'config> Builder<DartLanguage> for DartBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<DartLanguage>) -> oak_core::builder::BuildOutput<DartLanguage> {
        let parser = DartParser::new(self.config);
        let mut parse_session = oak_core::parser::session::ParseSession::<DartLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_session);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
                    Ok(ast_root) => oak_core::OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(e) => oak_core::OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
                }
            }
            Err(e) => oak_core::OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
