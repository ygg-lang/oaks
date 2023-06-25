use crate::{VerilogParser, ast::*, language::VerilogLanguage, lexer::token_type::VerilogKind};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// AST builder for Verilog.
#[derive(Clone, Copy)]
pub struct VerilogBuilder<'config> {
    config: &'config VerilogLanguage,
}

impl<'config> VerilogBuilder<'config> {
    pub const fn new(config: &'config VerilogLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<VerilogLanguage> for VerilogBuilder<'config> {
    fn build<'a, S: oak_core::source::Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<VerilogLanguage>) -> BuildOutput<VerilogLanguage> {
        let parser = VerilogParser::new(self.config);
        let mut session = oak_core::parser::session::ParseSession::<VerilogLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(&green_tree, &source_text) {
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

impl<'config> VerilogBuilder<'config> {
    fn build_root(&self, green_tree: &GreenNode<VerilogLanguage>, source: &SourceText) -> Result<VerilogRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut modules = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == VerilogKind::Module {
                    modules.push(self.build_module(n, source)?);
                }
            }
        }

        Ok(VerilogRoot { modules })
    }

    fn build_module(&self, node: RedNode<VerilogLanguage>, source: &SourceText) -> Result<VerilogModule, OakError> {
        let mut name = String::new();
        let mut ports = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == VerilogKind::Identifier {
                        if name.is_empty() {
                            name = source.get_text_in(t.span.clone().into()).to_string();
                        }
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    VerilogKind::PortList => {
                        ports = self.build_port_list(n, source)?;
                    }
                    VerilogKind::ModuleItem => {
                        if let Some(item) = self.build_module_item(n, source)? {
                            items.push(item);
                        }
                    }
                    _ => {}
                },
            }
        }

        Ok(VerilogModule { name, ports, items })
    }

    fn build_port_list(&self, node: RedNode<VerilogLanguage>, source: &SourceText) -> Result<Vec<VerilogPort>, OakError> {
        let mut ports = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == VerilogKind::Port {
                    ports.push(self.build_port(n, source)?);
                }
            }
        }
        Ok(ports)
    }

    fn build_port(&self, node: RedNode<VerilogLanguage>, source: &SourceText) -> Result<VerilogPort, OakError> {
        let mut name = String::new();
        let mut direction = None;
        let mut ty = None;

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                let text = source.get_text_in(t.span.clone().into()).to_string();
                match t.kind {
                    VerilogKind::InputKw | VerilogKind::OutputKw | VerilogKind::InoutKw => {
                        direction = Some(text);
                    }
                    VerilogKind::WireKw | VerilogKind::RegKw => {
                        ty = Some(text);
                    }
                    VerilogKind::Identifier => {
                        name = text;
                    }
                    _ => {}
                }
            }
        }

        Ok(VerilogPort { name, direction, ty })
    }

    fn build_module_item(&self, node: RedNode<VerilogLanguage>, source: &SourceText) -> Result<Option<VerilogModuleItem>, OakError> {
        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    VerilogKind::Assign => {
                        return Ok(Some(VerilogModuleItem::Assign(self.build_assign(n, source)?)));
                    }
                    VerilogKind::Always => {
                        return Ok(Some(VerilogModuleItem::Always(self.build_always(n, source)?)));
                    }
                    VerilogKind::Initial => {
                        return Ok(Some(VerilogModuleItem::Initial(self.build_initial(n, source)?)));
                    }
                    VerilogKind::Declaration => {
                        return Ok(Some(VerilogModuleItem::Declaration(self.build_declaration(n, source)?)));
                    }
                    _ => {}
                }
            }
        }
        Ok(None)
    }

    fn build_assign(&self, node: RedNode<VerilogLanguage>, source: &SourceText) -> Result<VerilogAssign, OakError> {
        let mut text = String::new();
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                let t_text = source.get_text_in(t.span.clone().into()).to_string();
                if t_text != "assign" && t_text != ";" {
                    text.push_str(&t_text);
                }
            }
        }
        let parts: Vec<&str> = text.split('=').collect();
        let left = parts.get(0).unwrap_or(&"").trim().to_string();
        let right = parts.get(1).unwrap_or(&"").trim().to_string();
        Ok(VerilogAssign { left, right })
    }

    fn build_always(&self, node: RedNode<VerilogLanguage>, source: &SourceText) -> Result<VerilogAlways, OakError> {
        let mut sensitivity = None;
        let mut statement = String::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == VerilogKind::At {
                        // Handled by sensitivity list if we had a more granular parser
                    }
                    else if t.kind != VerilogKind::AlwaysKw {
                        statement.push_str(&source.get_text_in(t.span.clone()));
                    }
                }
                RedTree::Node(n) => {
                    if n.green.kind == VerilogKind::Block || n.green.kind == VerilogKind::Statement {
                        statement.push_str(&self.get_node_text(n, source));
                    }
                    else if n.green.kind == VerilogKind::Expression {
                        sensitivity = Some(self.get_node_text(n, source));
                    }
                }
            }
        }

        Ok(VerilogAlways { sensitivity, statement })
    }

    fn build_initial(&self, node: RedNode<VerilogLanguage>, source: &SourceText) -> Result<VerilogInitial, OakError> {
        let mut statement = String::new();
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind != VerilogKind::InitialKw {
                        statement.push_str(&source.get_text_in(t.span.clone()));
                    }
                }
                RedTree::Node(n) => {
                    statement.push_str(&self.get_node_text(n, source));
                }
            }
        }
        Ok(VerilogInitial { statement })
    }

    fn build_declaration(&self, node: RedNode<VerilogLanguage>, source: &SourceText) -> Result<VerilogDeclaration, OakError> {
        let mut ty = String::new();
        let mut name = String::new();
        let mut value = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    let t_text = source.get_text_in(t.span.clone().into()).to_string();
                    if t.kind == VerilogKind::WireKw || t.kind == VerilogKind::RegKw || t.kind == VerilogKind::ParameterKw {
                        ty = t_text;
                    }
                    else if t.kind == VerilogKind::Identifier && name.is_empty() {
                        name = t_text;
                    }
                    else if t.kind == VerilogKind::Equal || t.kind == VerilogKind::AssignKw {
                        // Value follows
                    }
                    else if !t_text.trim().is_empty() && t_text != ";" && t_text != "," {
                        if value.is_none() {
                            value = Some(t_text);
                        }
                        else {
                            value.as_mut().unwrap().push_str(&t_text);
                        }
                    }
                }
                RedTree::Node(n) => {
                    let n_text = self.get_node_text(n, source);
                    if value.is_none() {
                        value = Some(n_text);
                    }
                    else {
                        value.as_mut().unwrap().push_str(&n_text);
                    }
                }
            }
        }

        Ok(VerilogDeclaration { ty, name, value })
    }

    fn get_node_text(&self, node: RedNode<VerilogLanguage>, source: &SourceText) -> String {
        let mut text = String::new();
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    text.push_str(&source.get_text_in(t.span.clone().into()));
                }
                RedTree::Node(n) => {
                    text.push_str(&self.get_node_text(n, source));
                }
            }
        }
        text
    }
}
