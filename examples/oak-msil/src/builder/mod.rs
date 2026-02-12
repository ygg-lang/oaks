//! Builder implementation for the MSIL language.

use crate::{
    ast::*,
    language::MsilLanguage,
    parser::{MsilParser, element_type::MsilElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Source, SourceText, TextEdit};

/// Builder for the MSIL language.
#[derive(Clone)]
pub struct MsilBuilder<'config> {
    #[allow(dead_code)]
    config: &'config MsilLanguage,
}

impl<'config> MsilBuilder<'config> {
    /// Creates a new `MsilBuilder` with the given configuration.
    pub fn new(config: &'config MsilLanguage) -> Self {
        Self { config }
    }

    /// Builds the root AST node from a green tree.
    pub fn build_root(&self, green_tree: &GreenNode<MsilLanguage>, source: &SourceText) -> Result<MsilRoot, oak_core::OakError> {
        let red_root = oak_core::RedNode::new(green_tree, 0);

        let mut items = Vec::new();
        for child in red_root.children() {
            if let oak_core::RedTree::Node(node) = child {
                if let Some(item) = self.build_item(&node, source) {
                    items.push(item)
                }
            }
        }

        Ok(MsilRoot { items })
    }

    fn build_item(&self, node: &oak_core::RedNode<MsilLanguage>, source: &SourceText) -> Option<Item> {
        let kind = node.green.kind;
        match kind {
            MsilElementType::Assembly => {
                let mut name = "unknown".to_string();
                for child in node.children() {
                    if let oak_core::RedTree::Node(n) = child {
                        if n.green.kind == MsilElementType::Identifier {
                            name = source.get_text_in(n.span()).to_string();
                            break;
                        }
                    }
                }
                Some(Item::Assembly(crate::ast::Assembly { name, span: node.span() }))
            }
            MsilElementType::AssemblyExtern => {
                let mut name = "unknown".to_string();
                for child in node.children() {
                    if let oak_core::RedTree::Node(n) = child {
                        if n.green.kind == MsilElementType::Identifier {
                            name = source.get_text_in(n.span()).to_string();
                            break;
                        }
                    }
                }
                Some(Item::AssemblyExtern(name))
            }
            MsilElementType::Module => {
                let mut name = "unknown".to_string();
                for child in node.children() {
                    if let oak_core::RedTree::Node(n) = child {
                        if n.green.kind == MsilElementType::Identifier {
                            name = source.get_text_in(n.span()).to_string();
                            break;
                        }
                    }
                }
                Some(Item::Module(name))
            }
            MsilElementType::Class => {
                let mut name = "Unknown".to_string();
                let mut methods = Vec::new();
                for child in node.children() {
                    if let oak_core::RedTree::Node(n) = child {
                        if n.green.kind == MsilElementType::Identifier {
                            name = source.get_text_in(n.span()).to_string()
                        }
                        else if n.green.kind == MsilElementType::Method {
                            if let Some(method) = self.build_method(&n, source) {
                                methods.push(method)
                            }
                        }
                    }
                }
                Some(Item::Class(crate::ast::Class { name, methods, span: node.span() }))
            }
            _ => None,
        }
    }

    fn build_method(&self, node: &oak_core::RedNode<MsilLanguage>, source: &SourceText) -> Option<crate::ast::Method> {
        let mut name = "Unknown".to_string();
        let mut instructions = Vec::new();
        for child in node.children() {
            if let oak_core::RedTree::Node(n) = child {
                if n.green.kind == MsilElementType::Identifier {
                    name = source.get_text_in(n.span()).to_string();
                }
                else if n.green.kind == MsilElementType::Instruction {
                    if let Some(inst) = self.build_instruction(&n, source) {
                        instructions.push(inst);
                    }
                }
            }
        }

        Some(crate::ast::Method { name, instructions, span: node.span() })
    }

    fn build_instruction(&self, node: &oak_core::RedNode<MsilLanguage>, source: &SourceText) -> Option<Instruction> {
        let mut opcode = String::new();
        let mut operand = None;

        for child in node.children() {
            if let oak_core::RedTree::Node(n) = child {
                match n.green.kind {
                    MsilElementType::Identifier => {
                        if opcode.is_empty() {
                            opcode = source.get_text_in(n.span()).to_string();
                        }
                        else {
                            operand = Some(source.get_text_in(n.span()).to_string());
                        }
                    }
                    MsilElementType::String => {
                        operand = Some(source.get_text_in(n.span()).to_string());
                    }
                    _ => {}
                }
            }
        }

        if opcode == "ldstr" {
            Some(Instruction::String(operand.unwrap_or_default()))
        }
        else if opcode == "call" {
            Some(Instruction::Call(operand.unwrap_or_default()))
        }
        else if !opcode.is_empty() {
            Some(Instruction::Simple(opcode))
        }
        else {
            None
        }
    }
}

impl<'config> Builder<MsilLanguage> for MsilBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<MsilLanguage>) -> OakDiagnostics<MsilRoot> {
        let parser = MsilParser::new(self.config);
        let lexer = crate::lexer::MsilLexer::new(&self.config);

        let mut cache = oak_core::parser::session::ParseSession::<MsilLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                OakDiagnostics { result: self.build_root(green_tree, &source_text), diagnostics: parse_result.diagnostics }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
