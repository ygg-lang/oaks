#![doc = include_str!("readme.md")]
use crate::{
    ast::*,
    language::CrystalLanguage,
    parser::{CrystalParser, element_type::CrystalElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

/// AST builder for the Crystal language
#[derive(Clone)]
pub struct CrystalBuilder<'config> {
    /// Language configuration
    config: &'config CrystalLanguage,
}

impl<'config> CrystalBuilder<'config> {
    /// Creates a new Crystal builder
    pub fn new(config: &'config CrystalLanguage) -> Self {
        Self { config }
    }

    /// Builds the AST root node from the syntax tree
    pub fn build_root(&self, green: &GreenNode<CrystalLanguage>, source: &SourceText) -> Result<CrystalRoot, oak_core::OakError> {
        let red = RedNode::new(green, 0);
        let mut items = Vec::new();

        for child in red.children() {
            if let RedTree::Node(node) = child {
                if let Some(item) = self.build_item(&node, source) {
                    items.push(item);
                }
            }
        }

        Ok(CrystalRoot { items })
    }

    fn build_item(&self, node: &RedNode<CrystalLanguage>, source: &SourceText) -> Option<Item> {
        match node.green.kind {
            CrystalElementType::ClassDef => self.build_class(node, source).map(Item::Class),
            CrystalElementType::ModuleDef => self.build_module(node, source).map(Item::Module),
            CrystalElementType::MethodDef => self.build_method(node, source).map(Item::Def),
            _ if node.green.kind as u16 >= CrystalElementType::IfExpr as u16 && node.green.kind as u16 <= CrystalElementType::ParenExpr as u16 => self.build_expression(node, source).map(Item::Expression),
            _ => None,
        }
    }

    fn build_class(&self, node: &RedNode<CrystalLanguage>, source: &SourceText) -> Option<ClassDeclaration> {
        let mut name = None;
        let mut body = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    CrystalElementType::Identifier => {
                        if name.is_none() {
                            name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                    }
                    CrystalElementType::Block => {
                        for block_child in n.children() {
                            if let RedTree::Node(bn) = block_child {
                                if let Some(item) = self.build_item(&bn, source) {
                                    body.push(item);
                                }
                            }
                        }
                    }
                    _ => {
                        if let Some(item) = self.build_item(&n, source) {
                            body.push(item);
                        }
                    }
                }
            }
        }

        name.map(|name| ClassDeclaration { name, body, span: node.span() })
    }

    fn build_module(&self, node: &RedNode<CrystalLanguage>, source: &SourceText) -> Option<ModuleDeclaration> {
        let mut name = None;
        let mut body = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    CrystalElementType::Identifier => {
                        if name.is_none() {
                            name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                    }
                    CrystalElementType::Block => {
                        for block_child in n.children() {
                            if let RedTree::Node(bn) = block_child {
                                if let Some(item) = self.build_item(&bn, source) {
                                    body.push(item);
                                }
                            }
                        }
                    }
                    _ => {
                        if let Some(item) = self.build_item(&n, source) {
                            body.push(item);
                        }
                    }
                }
            }
        }

        name.map(|name| ModuleDeclaration { name, body, span: node.span() })
    }

    fn build_method(&self, node: &RedNode<CrystalLanguage>, source: &SourceText) -> Option<MethodDeclaration> {
        let mut name = None;
        let mut params = Vec::new();
        let mut body = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    CrystalElementType::Identifier => {
                        if name.is_none() {
                            name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                    }
                    CrystalElementType::ParamList => {
                        params = self.build_params(&n, source);
                    }
                    CrystalElementType::Block => {
                        for block_child in n.children() {
                            if let RedTree::Node(bn) = block_child {
                                if let Some(item) = self.build_item(&bn, source) {
                                    body.push(item);
                                }
                            }
                        }
                    }
                    _ => {
                        if let Some(item) = self.build_item(&n, source) {
                            body.push(item);
                        }
                    }
                }
            }
        }

        name.map(|name| MethodDeclaration { name, params, body, span: node.span() })
    }

    fn build_params(&self, node: &RedNode<CrystalLanguage>, source: &SourceText) -> Vec<Parameter> {
        let mut params = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == CrystalElementType::Param {
                    if let Some(param) = self.build_param(&n, source) {
                        params.push(param);
                    }
                }
            }
        }
        params
    }

    fn build_param(&self, node: &RedNode<CrystalLanguage>, source: &SourceText) -> Option<Parameter> {
        let mut name = None;
        let mut type_name = None;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    CrystalElementType::Identifier => {
                        if name.is_none() {
                            name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                        else if type_name.is_none() {
                            type_name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                    }
                    _ => {}
                }
            }
        }

        name.map(|name| Parameter { name, type_name })
    }

    fn build_expression(&self, node: &RedNode<CrystalLanguage>, source: &SourceText) -> Option<Expression> {
        match node.green.kind {
            CrystalElementType::LiteralExpr => {
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        match n.green.kind {
                            CrystalElementType::Number => return Some(Expression::Literal(Literal::Number(source.get_text_in(n.span()).to_string()))),
                            CrystalElementType::String => return Some(Expression::Literal(Literal::String(source.get_text_in(n.span()).to_string()))),
                            CrystalElementType::TrueKeyword => return Some(Expression::Literal(Literal::Boolean(true))),
                            CrystalElementType::FalseKeyword => return Some(Expression::Literal(Literal::Boolean(false))),
                            CrystalElementType::NilKeyword => return Some(Expression::Literal(Literal::Nil)),
                            _ => {}
                        }
                    }
                }
                None
            }
            CrystalElementType::CallExpr => {
                let mut name = None;
                let mut receiver = None;
                let mut args = Vec::new();

                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        match n.green.kind {
                            CrystalElementType::Identifier => {
                                if name.is_none() {
                                    name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                                }
                            }
                            _ if n.green.kind as u16 >= CrystalElementType::IfExpr as u16 && n.green.kind as u16 <= CrystalElementType::ParenExpr as u16 => {
                                if let Some(expr) = self.build_expression(&n, source) {
                                    if receiver.is_none() {
                                        receiver = Some(Box::new(expr));
                                    }
                                    else {
                                        args.push(expr);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }

                name.map(|name| Expression::Call(Call { receiver, name, args }))
            }
            _ => None,
        }
    }
}

impl<'config> Builder<CrystalLanguage> for CrystalBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<CrystalLanguage>) -> oak_core::builder::BuildOutput<CrystalLanguage> {
        let parser = CrystalParser::new(self.config);
        let mut cache = oak_core::parser::ParseSession::<CrystalLanguage>::default();
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
