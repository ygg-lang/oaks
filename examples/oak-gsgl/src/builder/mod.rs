#![doc = include_str!("readme.md")]
use crate::{ast::*, language::GsglLanguage, lexer::token_type::GsglTokenType, parser::element_type::GsglElementType};
use oak_core::{Builder, BuilderCache, OakDiagnostics, OakError, Parser, RedNode, RedTree, Source, SourceText, TextEdit, builder::BuildOutput};

/// Builder for the GSGL language.
pub struct GsglBuilder {}

impl GsglBuilder {
    /// Creates a new `GsglBuilder`.
    pub fn new(_lang: &GsglLanguage) -> Self {
        Self {}
    }

    fn build_type(&self, node: RedNode<GsglLanguage>, source: &SourceText) -> Result<GsglType, OakError> {
        let mut name = String::new();
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == GsglTokenType::Identifier || (t.kind as u8 >= 84 && t.kind as u8 <= 107) {
                    name = source.get_text_in(t.span.clone().into()).to_string();
                    break;
                }
            }
        }
        Ok(GsglType { name })
    }

    fn build_param(&self, node: RedNode<GsglLanguage>, source: &SourceText) -> Result<GsglParam, OakError> {
        let mut name = String::new();
        let mut ty = GsglType { name: String::new() };
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == GsglTokenType::Identifier {
                    if name.is_empty() {
                        name = source.get_text_in(t.span.clone().into()).to_string();
                    }
                }
                else if t.kind as u8 >= 84 && t.kind as u8 <= 107 {
                    ty = GsglType { name: source.get_text_in(t.span.clone().into()).to_string() };
                }
            }
        }
        Ok(GsglParam { name, ty })
    }

    fn build_function(&self, node: RedNode<GsglLanguage>, source: &SourceText) -> Result<GsglFunction, OakError> {
        let mut name = String::new();
        let mut return_type = GsglType { name: String::new() };
        let mut params = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == GsglTokenType::Identifier {
                        if return_type.name.is_empty() {
                            return_type.name = source.get_text_in(t.span.clone().into()).to_string();
                        }
                        else if name.is_empty() {
                            name = source.get_text_in(t.span.clone().into()).to_string();
                        }
                    }
                    else if t.kind as u8 >= 84 && t.kind as u8 <= 107 {
                        return_type.name = source.get_text_in(t.span.clone().into()).to_string();
                    }
                }
                RedTree::Node(n) => {
                    if n.kind::<GsglElementType>() == GsglElementType::Parameter {
                        params.push(self.build_param(n, source)?);
                    }
                }
            }
        }

        Ok(GsglFunction { name, return_type, params })
    }

    fn build_struct_member(&self, node: RedNode<GsglLanguage>, source: &SourceText) -> Result<GsglStructMember, OakError> {
        let mut name = String::new();
        let mut ty = GsglType { name: String::new() };
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == GsglTokenType::Identifier {
                    if ty.name.is_empty() {
                        ty.name = source.get_text_in(t.span.clone().into()).to_string();
                    }
                    else if name.is_empty() {
                        name = source.get_text_in(t.span.clone().into()).to_string();
                    }
                }
                else if t.kind as u8 >= 84 && t.kind as u8 <= 107 {
                    ty.name = source.get_text_in(t.span.clone().into()).to_string();
                }
            }
        }
        Ok(GsglStructMember { name, ty })
    }

    fn build_struct(&self, node: RedNode<GsglLanguage>, source: &SourceText) -> Result<GsglStruct, OakError> {
        let mut name = String::new();
        let mut members = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == GsglTokenType::Identifier && name.is_empty() {
                        name = source.get_text_in(t.span.clone().into()).to_string();
                    }
                }
                RedTree::Node(n) => {
                    if n.kind::<GsglElementType>() == GsglElementType::VariableDecl {
                        members.push(self.build_struct_member(n, source)?);
                    }
                }
            }
        }

        Ok(GsglStruct { name, members })
    }
}

impl Builder<GsglLanguage> for GsglBuilder {
    fn build<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<GsglLanguage>) -> BuildOutput<GsglLanguage> {
        let source = SourceText::new(text.get_text_from(0).as_ref());
        let mut functions = Vec::new();
        let mut structs = Vec::new();

        let parse_output = crate::parser::GsglParser::new(GsglLanguage::default()).parse(text, edits, cache);
        let root = RedNode::new(parse_output.green(), 0);

        let result = (|| {
            for child in root.children() {
                if let RedTree::Node(n) = child {
                    match n.kind() {
                        GsglElementType::FunctionDecl => {
                            functions.push(self.build_function(n, &source)?);
                        }
                        GsglElementType::StructDecl => {
                            structs.push(self.build_struct(n, &source)?);
                        }
                        _ => {}
                    }
                }
            }
            Ok(GsglRoot { functions, structs })
        })();

        match result {
            Ok(root) => OakDiagnostics::success(root),
            Err(e) => OakDiagnostics::error(e),
        }
    }
}
