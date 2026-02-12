use crate::{
    ast::*,
    language::ProtobufLanguage,
    parser::{ProtobufParser, element_type::ProtobufElementType},
};
use oak_core::{
    GreenNode, GreenTree, OakDiagnostics, OakError, SourceText,
    builder::{BuildOutput, Builder, BuilderCache},
    parser::Parser,
    source::{Source, TextEdit},
};

/// Builder for Protobuf AST, coordinating parsing.
#[derive(Debug)]
pub struct ProtobufBuilder<'config> {
    config: &'config ProtobufLanguage,
}

impl<'config> ProtobufBuilder<'config> {
    /// Creates a new `ProtobufBuilder` with the given configuration.
    pub fn new(config: &'config ProtobufLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<ProtobufLanguage> for ProtobufBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<ProtobufLanguage>) -> BuildOutput<ProtobufLanguage> {
        let parser = ProtobufParser::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<ProtobufLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let text = source.get_text_in((0..source.length()).into());
                let source_text = SourceText::new(text.into_owned());
                match self.build_root(&green_tree, &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> ProtobufBuilder<'config> {
    fn build_root(&self, green_tree: &GreenNode<ProtobufLanguage>, source: &SourceText) -> Result<ProtobufRoot, OakError> {
        let mut definitions = Vec::new();
        let mut current_offset = 0;

        for child in green_tree.children {
            match child {
                GreenTree::Node(n) => {
                    if let Some(def) = self.build_definition(n, current_offset, source)? {
                        definitions.push(def);
                    }
                    current_offset += n.byte_length as usize;
                }
                GreenTree::Leaf(l) => {
                    current_offset += l.length as usize;
                }
            }
        }

        Ok(ProtobufRoot { definitions })
    }

    fn build_definition(&self, node: &GreenNode<ProtobufLanguage>, offset: usize, source: &SourceText) -> Result<Option<Definition>, OakError> {
        let span = oak_core::Range { start: offset, end: offset + node.byte_length as usize };

        match node.kind {
            ProtobufElementType::SyntaxDef => {
                let version = source.get_text_in(span).to_string();
                Ok(Some(Definition::Syntax(Syntax { version })))
            }
            ProtobufElementType::PackageDef => {
                let name = source.get_text_in(span).to_string();
                Ok(Some(Definition::Package(Package { name })))
            }
            ProtobufElementType::ImportDef => {
                let path = source.get_text_in(span).to_string();
                Ok(Some(Definition::Import(Import { path, is_public: false, is_weak: false })))
            }
            ProtobufElementType::MessageDef => {
                let name = source.get_text_in(span).to_string();
                Ok(Some(Definition::Message(Message { name, fields: Vec::new() })))
            }
            ProtobufElementType::EnumDef => {
                let name = source.get_text_in(span).to_string();
                Ok(Some(Definition::Enum(Enum { name, values: Vec::new() })))
            }
            ProtobufElementType::ServiceDef => {
                let name = source.get_text_in(span).to_string();
                Ok(Some(Definition::Service(Service { name, methods: Vec::new() })))
            }
            _ => {
                // For other types, try to find a child node that is a definition
                for child in node.children {
                    if let GreenTree::Node(n) = child {
                        if let Some(def) = self.build_definition(n, offset, source)? {
                            return Ok(Some(def));
                        }
                    }
                }
                Ok(None)
            }
        }
    }
}
