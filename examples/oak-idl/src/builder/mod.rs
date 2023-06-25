use crate::{IdlLanguage, IdlParser, ast::*, parser::element_type::IdlElementType};
use oak_core::{Builder, BuilderCache, GreenNode, GreenTree, OakDiagnostics, OakError, Parser, SourceText, TextEdit, source::Source};

/// A builder for IDL language structures.
pub struct IdlBuilder<'config> {
    config: &'config IdlLanguage,
}

impl<'config> IdlBuilder<'config> {
    /// Creates a new instance of the IDL builder.
    pub fn new(config: &'config IdlLanguage) -> Self {
        Self { config }
    }

    /// Builds the IDL AST root from a green tree.
    pub fn build_root(&self, green_tree: GreenNode<IdlLanguage>, source: &SourceText) -> Result<IdlRoot, OakError> {
        let mut items = Vec::new();
        let mut current_offset = 0;

        for child in green_tree.children {
            match child {
                GreenTree::Node(n) => {
                    if let Some(item) = self.build_item(n, current_offset, source)? {
                        items.push(item);
                    }
                    current_offset += n.byte_length as usize;
                }
                GreenTree::Leaf(l) => {
                    current_offset += l.length as usize;
                }
            }
        }

        Ok(IdlRoot { items })
    }

    fn build_item(&self, node: &GreenNode<IdlLanguage>, offset: usize, source: &SourceText) -> Result<Option<IdlItem>, OakError> {
        let span = core::range::Range { start: offset, end: offset + node.byte_length as usize };

        match node.kind {
            IdlElementType::Module => {
                let name = source.get_text_in(span.clone()).to_string();
                Ok(Some(IdlItem::Module(Module { name, items: Vec::new(), span: span.into() })))
            }
            IdlElementType::Interface => {
                let name = source.get_text_in(span.clone()).to_string();
                Ok(Some(IdlItem::Interface(Interface { name, members: Vec::new(), span: span.into() })))
            }
            IdlElementType::Struct => {
                let name = source.get_text_in(span.clone()).to_string();
                Ok(Some(IdlItem::Struct(Struct { name, fields: Vec::new(), span: span.into() })))
            }
            _ => {
                // For other types, try to find a child node that is an item
                for child in node.children {
                    if let GreenTree::Node(n) = child {
                        if let Some(item) = self.build_item(n, offset, source)? {
                            return Ok(Some(item));
                        }
                    }
                }
                Ok(None)
            }
        }
    }
}

impl<'config> Builder<IdlLanguage> for IdlBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<IdlLanguage>) -> OakDiagnostics<IdlRoot> {
        let parser = IdlParser::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<IdlLanguage>::default();
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
