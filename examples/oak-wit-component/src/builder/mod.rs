use crate::{
    ast::*,
    language::WitLanguage,
    lexer::token_type::WitTokenType,
    parser::{WitParser, element_type::WitElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, Source, SourceText, TextEdit, builder::BuildOutput};

/// AST builder for the WIT Component format.
#[derive(Clone, Copy)]
pub struct WitBuilder<'config> {
    /// Language configuration
    config: &'config WitLanguage,
}

impl<'config> WitBuilder<'config> {
    /// Creates a new `WitBuilder` with the given configuration.
    pub const fn new(config: &'config WitLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<WitLanguage> for WitBuilder<'config> {
    fn build<'a, S: oak_core::source::Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<WitLanguage>) -> BuildOutput<WitLanguage> {
        let parser = WitParser::new(self.config);
        let mut session = oak_core::parser::session::ParseSession::<WitLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut session);

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

impl<'config> WitBuilder<'config> {
    fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, WitLanguage>, source: &SourceText) -> Result<WitRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut items = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    WitElementType::PackageKw => {
                        items.push(WitItem::Package(self.build_package(n, source)?));
                    }
                    WitElementType::WorldKw => {
                        items.push(WitItem::World(self.build_world(n, source)?));
                    }
                    WitElementType::InterfaceKw => {
                        items.push(WitItem::Interface(self.build_interface(n, source)?));
                    }
                    _ => {}
                }
            }
        }

        Ok(WitRoot { items })
    }

    fn build_package(&self, node: RedNode<WitLanguage>, source: &SourceText) -> Result<WitPackage, OakError> {
        let mut name = String::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == WitTokenType::Identifier {
                    name = source.get_text_in(t.span.clone().into()).to_string();
                }
            }
        }

        Ok(WitPackage { name })
    }

    fn build_world(&self, node: RedNode<WitLanguage>, source: &SourceText) -> Result<WitWorld, OakError> {
        let mut name = String::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == WitTokenType::Identifier {
                        name = source.get_text_in(t.span.clone().into()).to_string();
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    WitElementType::ImportKw => {
                        items.push(WitWorldItem::Import(self.build_import(n, source)?));
                    }
                    WitElementType::ExportKw => {
                        items.push(WitWorldItem::Export(self.build_export(n, source)?));
                    }
                    WitElementType::IncludeKw => {
                        items.push(WitWorldItem::Include(self.build_include(n, source)?));
                    }
                    _ => {}
                },
            }
        }

        Ok(WitWorld { name, items })
    }

    fn build_interface(&self, node: RedNode<WitLanguage>, source: &SourceText) -> Result<WitInterface, OakError> {
        let mut name = String::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == WitTokenType::Identifier {
                        name = source.get_text_in(t.span.clone().into()).to_string();
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    WitElementType::TypeKw => {
                        items.push(WitInterfaceItem::Type(self.build_type(n, source)?));
                    }
                    WitElementType::FuncKw => {
                        items.push(WitInterfaceItem::Func(self.build_func(n, source)?));
                    }
                    _ => {}
                },
            }
        }

        Ok(WitInterface { name, items })
    }

    fn build_import(&self, node: RedNode<WitLanguage>, source: &SourceText) -> Result<WitImport, OakError> {
        let mut name = String::new();
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == WitTokenType::Identifier {
                    name = source.get_text_in(t.span.clone().into()).to_string();
                }
            }
        }
        Ok(WitImport { name })
    }

    fn build_export(&self, node: RedNode<WitLanguage>, source: &SourceText) -> Result<WitExport, OakError> {
        let mut name = String::new();
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == WitTokenType::Identifier {
                    name = source.get_text_in(t.span.clone().into()).to_string();
                }
            }
        }
        Ok(WitExport { name })
    }

    fn build_include(&self, node: RedNode<WitLanguage>, source: &SourceText) -> Result<WitInclude, OakError> {
        let mut name = String::new();
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == WitTokenType::Identifier {
                    name = source.get_text_in(t.span.clone().into()).to_string();
                }
            }
        }
        Ok(WitInclude { name })
    }

    fn build_type(&self, node: RedNode<WitLanguage>, source: &SourceText) -> Result<WitType, OakError> {
        let mut name = String::new();
        let mut kind = WitTypeKind::Bool; // Default

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == WitTokenType::Identifier {
                    name = source.get_text_in(t.span.clone().into()).to_string();
                }
                else if let Some(t_kind) = self.parse_type_kind(&source.get_text_in(t.span.clone().into())) {
                    kind = t_kind;
                }
            }
        }

        Ok(WitType { name, kind })
    }

    fn build_func(&self, node: RedNode<WitLanguage>, source: &SourceText) -> Result<WitFunc, OakError> {
        let mut name = String::new();
        let mut params = Vec::new();
        let mut result = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == WitTokenType::Identifier {
                        name = source.get_text_in(t.span.clone().into()).to_string();
                    }
                }
                _ => {}
            }
        }

        Ok(WitFunc { name, params, result })
    }

    fn parse_type_kind(&self, text: &str) -> Option<WitTypeKind> {
        match text {
            "bool" => Some(WitTypeKind::Bool),
            "u32" => Some(WitTypeKind::U32),
            "string" => Some(WitTypeKind::String),
            _ => None,
        }
    }
}
