use crate::{ast::*, language::MatlabLanguage, parser::MatlabParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Source, SourceText, TextEdit};

#[derive(Clone)]
pub struct MatlabBuilder<'config> {
    config: &'config MatlabLanguage,
}

impl<'config> MatlabBuilder<'config> {
    pub fn new(config: &'config MatlabLanguage) -> Self {
        Self { config }
    }

    pub fn build_root(&self, green_tree: &GreenNode<MatlabLanguage>, source: &SourceText) -> Result<MatlabRoot, oak_core::OakError> {
        let red_root = oak_core::tree::RedNode::new(green_tree, 0);
        let mut items = Vec::new();
        for child in red_root.children() {
            if let oak_core::tree::RedTree::Node(node) = child {
                if let Some(item) = self.build_item(&node, source) {
                    items.push(item);
                }
            }
        }
        Ok(MatlabRoot { items })
    }

    pub fn build_item(&self, node: &oak_core::RedNode<MatlabLanguage>, source: &SourceText) -> Option<Item> {
        use crate::kind::MatlabSyntaxKind::*;
        let kind = node.green.kind;

        match kind {
            FunctionDef => {
                let mut name = "anonymous".to_string();
                for child in node.children() {
                    if let oak_core::RedTree::Node(n) = child {
                        if n.green.kind == Identifier {
                            name = source.get_text_in(n.span()).to_string();
                            break;
                        }
                    }
                }
                Some(Item::Function(crate::ast::Function { name, inputs: Vec::new(), outputs: Vec::new(), body: Vec::new(), span: node.span() }))
            }
            ClassDef => {
                let mut name = "Unknown".to_string();
                for child in node.children() {
                    if let oak_core::RedTree::Node(n) = child {
                        if n.green.kind == Identifier {
                            name = source.get_text_in(n.span()).to_string();
                            break;
                        }
                    }
                }
                Some(Item::Class(crate::ast::Class { name, superclasses: Vec::new(), properties: Vec::new(), methods: Vec::new(), span: node.span() }))
            }
            Expression | Block | Statement => Some(Item::Statement(crate::ast::Statement::Expression { value: source.get_text_in(node.span()).to_string(), span: node.span() })),
            _ => None,
        }
    }
}

impl<'config> Builder<MatlabLanguage> for MatlabBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<MatlabLanguage>) -> oak_core::builder::BuildOutput<MatlabLanguage> {
        let parser = MatlabParser::new(self.config);
        let lexer = crate::lexer::MatlabLexer::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<MatlabLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
                OakDiagnostics { result: self.build_root(&green_tree, &source_text), diagnostics: parse_result.diagnostics }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
