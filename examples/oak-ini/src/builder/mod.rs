use crate::{IniParser, ast::*, language::IniLanguage, lexer::token_type::IniTokenType, parser::element_type::IniElementType};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, source::Source};

pub struct IniBuilder<'config> {
    config: &'config IniLanguage,
}

impl<'config> IniBuilder<'config> {
    pub fn new(config: &'config IniLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<IniLanguage> for IniBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<IniLanguage>) -> OakDiagnostics<IniRoot> {
        let parser = IniParser::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<IniLanguage>::default();
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

impl<'config> IniBuilder<'config> {
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, IniLanguage>, source: &SourceText) -> Result<IniRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut sections = Vec::new();
        let mut properties = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == IniElementType::Table {
                    sections.push(self.build_section(n, source)?);
                }
                else if n.green.kind == IniElementType::KeyValue {
                    properties.push(self.build_property(n, source)?);
                }
            }
        }
        Ok(IniRoot { sections, properties })
    }

    fn build_section(&self, node: RedNode<IniLanguage>, source: &SourceText) -> Result<Section, OakError> {
        let span = node.span();
        let mut name = String::new();
        let mut properties = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) if t.kind == IniTokenType::Identifier => name = source.get_text_in(t.span.clone().into()).to_string(),
                RedTree::Node(n) if n.green.kind == IniElementType::KeyValue => properties.push(self.build_property(n, source)?),
                _ => {}
            }
        }
        Ok(Section { name, properties, span })
    }

    fn build_property(&self, node: RedNode<IniLanguage>, source: &SourceText) -> Result<Property, OakError> {
        let span = node.span();
        let mut key = String::new();
        let mut value = String::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == IniElementType::Key {
                    key = source.get_text_in(n.span().into()).to_string();
                }
                else if n.green.kind == IniElementType::Value {
                    value = source.get_text_in(n.span().into()).to_string();
                }
            }
        }
        Ok(Property { key, value, span })
    }
}
