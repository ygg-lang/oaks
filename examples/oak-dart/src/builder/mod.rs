use crate::{ast::*, kind::DartSyntaxKind, language::DartLanguage, parser::DartParser};
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

    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, DartLanguage>, _source: &SourceText) -> Result<DartRoot, oak_core::OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let items = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    DartSyntaxKind::ClassDeclaration => {
                        // TODO: Implement build_class
                    }
                    DartSyntaxKind::FunctionDeclaration => {
                        // TODO: Implement build_function
                    }
                    _ => {}
                }
            }
        }
        Ok(DartRoot { items })
    }
}

impl<'config> Builder<DartLanguage> for DartBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<DartLanguage>) -> oak_core::builder::BuildOutput<DartLanguage> {
        let parser = DartParser::new(self.config);
        let mut parse_session = oak_core::parser::session::ParseSession::<DartLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_session);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
                match self.build_root(green_tree, &source_text) {
                    Ok(ast_root) => oak_core::OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(e) => oak_core::OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
                }
            }
            Err(e) => oak_core::OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
