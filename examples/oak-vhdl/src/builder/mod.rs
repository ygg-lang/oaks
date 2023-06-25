use crate::{VhdlParser, ast::*, language::VhdlLanguage, lexer::token_type::VhdlTokenType, parser::element_type::VhdlElementType};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, Source, SourceText, TextEdit, builder::BuildOutput};

/// AST builder for the VHDL language.
#[derive(Clone, Copy)]
pub struct VhdlBuilder<'config> {
    /// Language configuration
    config: &'config VhdlLanguage,
}

impl<'config> VhdlBuilder<'config> {
    /// Creates a new `VhdlBuilder` with the given configuration.
    pub const fn new(config: &'config VhdlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<VhdlLanguage> for VhdlBuilder<'config> {
    fn build<'a, S: oak_core::source::Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<VhdlLanguage>) -> BuildOutput<VhdlLanguage> {
        let parser = VhdlParser::new(self.config);
        let mut session = oak_core::parser::session::ParseSession::<VhdlLanguage>::default();
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

impl<'config> VhdlBuilder<'config> {
    fn build_root<S: oak_core::source::Source + ?Sized>(&self, green_tree: &GreenNode<VhdlLanguage>, source: &S) -> Result<VhdlRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut units = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    VhdlElementType::EntityDeclaration => {
                        units.push(DesignUnit::Entity(self.build_entity(n, source)?));
                    }
                    VhdlElementType::ArchitectureBody => {
                        units.push(DesignUnit::Architecture(self.build_architecture(n, source)?));
                    }
                    VhdlElementType::PackageDeclaration => {
                        units.push(DesignUnit::Package(self.build_package(n, source)?));
                    }
                    _ => {}
                }
            }
        }

        Ok(VhdlRoot { units })
    }

    fn build_entity<S: oak_core::source::Source + ?Sized>(&self, node: RedNode<VhdlLanguage>, source: &S) -> Result<EntityDeclaration, OakError> {
        let mut name = String::new();
        let mut ports = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == VhdlTokenType::Identifier {
                        name = source.get_text_in(t.span.clone()).to_string();
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    VhdlElementType::PortClause => {
                        ports = self.build_port_clause(n, source)?;
                    }
                    _ => {}
                },
            }
        }

        Ok(EntityDeclaration { name, ports })
    }

    fn build_port_clause<S: oak_core::source::Source + ?Sized>(&self, node: RedNode<VhdlLanguage>, source: &S) -> Result<Vec<PortDeclaration>, OakError> {
        let mut ports = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == VhdlElementType::PortDeclaration {
                    ports.push(self.build_port_declaration(n, source)?);
                }
            }
        }
        Ok(ports)
    }

    fn build_port_declaration<S: oak_core::source::Source + ?Sized>(&self, node: RedNode<VhdlLanguage>, source: &S) -> Result<PortDeclaration, OakError> {
        let mut name = String::new();
        let mut direction = PortDirection::In;
        let mut data_type = String::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == VhdlTokenType::Identifier {
                        if name.is_empty() {
                            name = source.get_text_in(t.span.clone()).to_string();
                        }
                        else {
                            data_type = source.get_text_in(t.span.clone()).to_string();
                        }
                    }
                    else if t.kind == VhdlTokenType::InKw {
                        direction = PortDirection::In;
                    }
                    else if t.kind == VhdlTokenType::OutKw {
                        direction = PortDirection::Out;
                    }
                    else if t.kind == VhdlTokenType::InoutKw {
                        direction = PortDirection::Inout;
                    }
                    else if t.kind == VhdlTokenType::BufferKw {
                        direction = PortDirection::Buffer;
                    }
                    else if t.kind == VhdlTokenType::LinkageKw {
                        direction = PortDirection::Linkage;
                    }
                }
                _ => {}
            }
        }

        Ok(PortDeclaration { name, direction, data_type })
    }

    fn build_architecture<S: oak_core::source::Source + ?Sized>(&self, node: RedNode<VhdlLanguage>, source: &S) -> Result<ArchitectureBody, OakError> {
        let mut name = String::new();
        let mut entity_name = String::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == VhdlTokenType::Identifier {
                        if name.is_empty() {
                            name = source.get_text_in(t.span.clone()).to_string();
                        }
                        else {
                            entity_name = source.get_text_in(t.span.clone()).to_string();
                        }
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    VhdlElementType::SignalDeclaration => {
                        items.push(ArchitectureItem::Signal(self.build_signal(n, source)?));
                    }
                    VhdlElementType::ProcessStatement => {
                        items.push(ArchitectureItem::Process(self.build_process(n, source)?));
                    }
                    VhdlElementType::ComponentDeclaration => {
                        items.push(ArchitectureItem::Component(self.build_component(n, source)?));
                    }
                    _ => {}
                },
            }
        }

        Ok(ArchitectureBody { name, entity_name, items })
    }

    fn build_signal<S: oak_core::source::Source + ?Sized>(&self, node: RedNode<VhdlLanguage>, source: &S) -> Result<SignalDeclaration, OakError> {
        let mut name = String::new();
        let mut data_type = String::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == VhdlTokenType::Identifier {
                    if name.is_empty() {
                        name = source.get_text_in(t.span.clone()).to_string();
                    }
                    else {
                        data_type = source.get_text_in(t.span.clone()).to_string();
                    }
                }
            }
        }

        Ok(SignalDeclaration { name, data_type })
    }

    fn build_process<S: oak_core::source::Source + ?Sized>(&self, node: RedNode<VhdlLanguage>, source: &S) -> Result<ProcessStatement, OakError> {
        let mut label = None;
        let sensitivity_list = Vec::new();
        let body = String::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == VhdlTokenType::Identifier {
                        label = Some(source.get_text_in(t.span.clone()).to_string());
                    }
                }
                _ => {}
            }
        }

        Ok(ProcessStatement { label, sensitivity_list, body })
    }

    fn build_component<S: oak_core::source::Source + ?Sized>(&self, node: RedNode<VhdlLanguage>, source: &S) -> Result<ComponentDeclaration, OakError> {
        let mut name = String::new();
        let mut ports = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == VhdlTokenType::Identifier {
                        name = source.get_text_in(t.span.clone()).to_string();
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    VhdlElementType::PortClause => {
                        ports = self.build_port_clause(n, source)?;
                    }
                    _ => {}
                },
            }
        }

        Ok(ComponentDeclaration { name, ports })
    }

    fn build_package<S: oak_core::source::Source + ?Sized>(&self, node: RedNode<VhdlLanguage>, source: &S) -> Result<PackageDeclaration, OakError> {
        let mut name = String::new();
        let items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == VhdlTokenType::Identifier {
                        name = source.get_text_in(t.span.clone()).to_string();
                    }
                }
                _ => {}
            }
        }

        Ok(PackageDeclaration { name, items })
    }
}
