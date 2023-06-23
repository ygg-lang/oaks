use crate::{
    ast::*,
    language::DartLanguage,
    lexer::token_type::DartTokenType,
    parser::{DartParser, element_type::DartElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// Dart 语言的 AST 构建器
#[derive(Clone)]
pub struct DartBuilder<'config> {
    config: &'config DartLanguage,
}

impl<'config> DartBuilder<'config> {
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
        Some(ClassDeclaration { name, span: node.span().into() })
    }

    fn build_function(&self, node: &RedNode<DartLanguage>, source: &SourceText) -> Option<FunctionDeclaration> {
        let name = self.find_identifier(node, source)?;
        Some(FunctionDeclaration { name, span: node.span().into() })
    }

    fn build_variable(&self, node: &RedNode<DartLanguage>, source: &SourceText) -> Option<VariableDeclaration> {
        let name = self.find_identifier(node, source)?;
        Some(VariableDeclaration { name, span: node.span().into() })
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
