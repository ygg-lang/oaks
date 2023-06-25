use crate::{WgslParser, ast::*, language::WgslLanguage, lexer::token_type::WgslTokenType, parser::element_type::WgslElementType};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, TextEdit, builder::BuildOutput, source::Source};

/// AST builder for the WebGPU Shading Language (WGSL).
#[derive(Clone, Copy)]
pub struct WgslBuilder<'config> {
    /// Language configuration
    config: &'config WgslLanguage,
}

impl<'config> WgslBuilder<'config> {
    /// Creates a new `WgslBuilder` with the given configuration.
    pub const fn new(config: &'config WgslLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<WgslLanguage> for WgslBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<WgslLanguage>) -> BuildOutput<WgslLanguage> {
        let parser = WgslParser::new(self.config);
        let mut session = oak_core::parser::session::ParseSession::<WgslLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => match self.build_root(green_tree, source) {
                Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                Err(build_error) => {
                    let mut diagnostics = parse_result.diagnostics;
                    diagnostics.push(build_error.clone());
                    OakDiagnostics { result: Err(build_error), diagnostics }
                }
            },
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> WgslBuilder<'config> {
    fn build_root<'a, S: Source + ?Sized>(&self, green_tree: &'a GreenNode<'a, WgslLanguage>, source: &S) -> Result<WgslRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut items = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    WgslElementType::Function => {
                        items.push(WgslItem::Function(self.build_function(n, source)?));
                    }
                    WgslElementType::Variable => {
                        items.push(WgslItem::Variable(self.build_variable(n, source)?));
                    }
                    WgslElementType::Struct => {
                        items.push(WgslItem::Struct(self.build_struct(n, source)?));
                    }
                    _ => {}
                }
            }
        }

        Ok(WgslRoot { items })
    }

    fn build_function<'a, S: Source + ?Sized>(&self, node: RedNode<'a, WgslLanguage>, source: &S) -> Result<WgslFunction, OakError> {
        let mut name = String::new();
        let mut params = Vec::new();
        let mut return_type = None;
        let mut found_fn_kw = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == WgslTokenType::FnKw {
                        found_fn_kw = true;
                    }
                    else if found_fn_kw && t.kind == WgslTokenType::Identifier {
                        if name.is_empty() {
                            name = source.get_text_in(t.span.clone().into()).to_string();
                        }
                        else {
                            return_type = Some(WgslType { name: source.get_text_in(t.span.clone().into()).to_string() });
                        }
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    WgslElementType::Param => {
                        params.push(self.build_param(n, source)?);
                    }
                    _ => {}
                },
            }
        }

        Ok(WgslFunction { name, params, return_type })
    }

    fn build_param<'a, S: Source + ?Sized>(&self, node: RedNode<'a, WgslLanguage>, source: &S) -> Result<WgslParam, OakError> {
        let mut name = String::new();
        let mut ty = WgslType { name: String::new() };

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == WgslTokenType::Identifier {
                    if name.is_empty() {
                        name = source.get_text_in(t.span.clone().into()).to_string();
                    }
                    else {
                        ty = WgslType { name: source.get_text_in(t.span.clone().into()).to_string() };
                    }
                }
            }
        }

        Ok(WgslParam { name, ty })
    }

    fn build_variable<'a, S: Source + ?Sized>(&self, node: RedNode<'a, WgslLanguage>, source: &S) -> Result<WgslVariable, OakError> {
        let mut name = String::new();
        let mut ty = None;
        let value = None;
        let mut found_var_or_let = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == WgslTokenType::VarKw || t.kind == WgslTokenType::LetKw {
                        found_var_or_let = true;
                    }
                    else if found_var_or_let && t.kind == WgslTokenType::Identifier {
                        if name.is_empty() {
                            name = source.get_text_in(t.span.clone().into()).to_string();
                        }
                        else {
                            ty = Some(WgslType { name: source.get_text_in(t.span.clone().into()).to_string() });
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(WgslVariable { name, ty, value })
    }

    fn build_struct<'a, S: Source + ?Sized>(&self, node: RedNode<'a, WgslLanguage>, source: &S) -> Result<WgslStruct, OakError> {
        let mut name = String::new();
        let mut members = Vec::new();
        let mut found_struct_kw = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == WgslTokenType::StructKw {
                        found_struct_kw = true;
                    }
                    else if found_struct_kw && t.kind == WgslTokenType::Identifier {
                        name = source.get_text_in(t.span.clone().into()).to_string();
                    }
                }
                RedTree::Node(n) => {
                    if n.green.kind == WgslElementType::StructMember {
                        members.push(self.build_struct_member(n, source)?);
                    }
                }
            }
        }

        Ok(WgslStruct { name, members })
    }

    fn build_struct_member<'a, S: Source + ?Sized>(&self, node: RedNode<'a, WgslLanguage>, source: &S) -> Result<WgslStructMember, OakError> {
        let mut name = String::new();
        let mut ty = WgslType { name: String::new() };

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == WgslTokenType::Identifier {
                    if name.is_empty() {
                        name = source.get_text_in(t.span.clone().into()).to_string();
                    }
                    else {
                        ty = WgslType { name: source.get_text_in(t.span.clone().into()).to_string() };
                    }
                }
            }
        }

        Ok(WgslStructMember { name, ty })
    }
}
