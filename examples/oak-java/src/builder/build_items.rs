use crate::{ast::*, builder::JavaBuilder, language::JavaLanguage, lexer::token_type::JavaTokenType, parser::element_type::JavaElementType};
use oak_core::{
    GreenNode,
    tree::red_tree::{RedNode, RedTree},
};

impl<'config> JavaBuilder<'config> {
    pub(crate) fn build_root(&self, green: &GreenNode<JavaLanguage>, source: &str) -> JavaRoot {
        let red = RedNode::new(green, 0);
        let mut items = Vec::new();

        for child in red.children() {
            if let RedTree::Node(node) = child {
                match node.green.kind {
                    JavaElementType::CompilationUnit => {
                        for sub_child in node.children() {
                            if let RedTree::Node(sub_node) = sub_child {
                                if let Some(item) = self.build_item(sub_node, source) {
                                    items.push(item)
                                }
                            }
                        }
                    }
                    _ => {
                        if let Some(item) = self.build_item(node, source) {
                            items.push(item)
                        }
                    }
                }
            }
        }

        JavaRoot { items }
    }

    pub(crate) fn build_item(&self, node: RedNode<JavaLanguage>, source: &str) -> Option<Item> {
        match node.green.kind {
            JavaElementType::ClassDeclaration => Some(Item::Class(self.build_class(node, source))),
            JavaElementType::InterfaceDeclaration => Some(Item::Interface(self.build_interface(node, source))),
            JavaElementType::EnumDeclaration => Some(Item::Enum(self.build_enum(node, source))),
            JavaElementType::StructDeclaration => Some(Item::Struct(self.build_struct(node, source))),
            JavaElementType::RecordDeclaration => Some(Item::Record(self.build_record(node, source))),
            JavaElementType::Package => Some(Item::Package(PackageDeclaration { name: self.extract_path(node, source), span: node.span() })),
            JavaElementType::Import => {
                let mut is_static = false;
                for child in node.children() {
                    if let RedTree::Leaf(leaf) = child {
                        if leaf.kind == JavaTokenType::Static {
                            is_static = true;
                            break;
                        }
                    }
                }
                Some(Item::Import(ImportDeclaration { path: self.extract_path(node, source), is_static, span: node.span() }))
            }
            _ => None,
        }
    }

    pub(crate) fn build_class(&self, node: RedNode<JavaLanguage>, source: &str) -> ClassDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let mut members = Vec::new();
        let modifiers = self.extract_modifiers(node.clone(), source);
        let annotations = self.extract_annotations(node.clone(), source);
        let (extends, implements) = self.extract_inheritance(node.clone(), source, &name, &modifiers);

        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                match sub_node.green.kind {
                    JavaElementType::MethodDeclaration => members.push(Member::Method(self.build_method(sub_node, source))),
                    JavaElementType::FieldDeclaration => members.push(Member::Field(self.build_field(sub_node, source))),
                    _ => self.collect_members(sub_node, source, &mut members),
                }
            }
        }

        let class = ClassDeclaration { modifiers, annotations, name, extends, implements, members, span: node.span() };

        class
    }

    pub(crate) fn build_interface(&self, node: RedNode<JavaLanguage>, source: &str) -> InterfaceDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let mut members = Vec::new();
        let modifiers = self.extract_modifiers(node.clone(), source);
        let annotations = self.extract_annotations(node.clone(), source);
        let (_, extends_raw) = self.extract_inheritance(node.clone(), source, &name, &modifiers);
        let extends = extends_raw; // Interfaces only have extends (which are actually implements in some sense but called extends in Java)

        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                match sub_node.green.kind {
                    JavaElementType::MethodDeclaration => members.push(Member::Method(self.build_method(sub_node, source))),
                    JavaElementType::FieldDeclaration => members.push(Member::Field(self.build_field(sub_node, source))),
                    _ => self.collect_members(sub_node, source, &mut members),
                }
            }
        }

        let interface = InterfaceDeclaration { modifiers, annotations, name, extends, members, span: node.span() };

        interface
    }

    pub(crate) fn build_enum(&self, node: RedNode<JavaLanguage>, source: &str) -> EnumDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let mut members = Vec::new();
        let modifiers = self.extract_modifiers(node.clone(), source);
        let annotations = self.extract_annotations(node.clone(), source);
        let (_, implements) = self.extract_inheritance(node.clone(), source, &name, &modifiers);

        let mut variants = Vec::new();

        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                match sub_node.green.kind {
                    JavaElementType::Identifier => variants.push(self.get_text(sub_node.span(), source).trim().to_string()),
                    JavaElementType::MethodDeclaration => members.push(Member::Method(self.build_method(sub_node, source))),
                    JavaElementType::FieldDeclaration => members.push(Member::Field(self.build_field(sub_node, source))),
                    _ => self.collect_members(sub_node, source, &mut members),
                }
            }
        }

        let enum_decl = EnumDeclaration { modifiers, annotations, name, variants, members, implements, span: node.span() };

        enum_decl
    }

    pub(crate) fn build_struct(&self, node: RedNode<JavaLanguage>, source: &str) -> StructDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let modifiers = self.extract_modifiers(node.clone(), source);
        let annotations = self.extract_annotations(node.clone(), source);
        let (_, implements) = self.extract_inheritance(node.clone(), source, &name, &modifiers);

        let mut members = Vec::new();
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                self.collect_members(sub_node, source, &mut members);
            }
        }

        let res = StructDeclaration { name, modifiers, annotations, implements, members, span: node.span() };

        res
    }

    pub(crate) fn build_record(&self, node: RedNode<JavaLanguage>, source: &str) -> RecordDeclaration {
        let name = self.extract_identifier(node.clone(), source);
        let modifiers = self.extract_modifiers(node.clone(), source);
        let annotations = self.extract_annotations(node.clone(), source);
        let (_, implements) = self.extract_inheritance(node.clone(), source, &name, &modifiers);

        let mut parameters = Vec::new();
        let mut members = Vec::new();

        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                match sub_node.green.kind {
                    JavaElementType::Parameter => parameters.push(self.build_parameter(sub_node, source)),
                    _ => self.collect_members(sub_node, source, &mut members),
                }
            }
        }

        let res = RecordDeclaration { name, modifiers, annotations, parameters, implements, members, span: node.span() };

        res
    }
}
