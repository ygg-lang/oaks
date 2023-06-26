use crate::{
    DejavuLanguage,
    ast::{ClassDeclaration, EnumDeclaration, FlagsDeclaration, IdentifierNode, ItemNode, TraitDeclaration, VariantDefinition, WidgetDeclaration},
    builder::{DejavuBuilder, text},
    lexer::token_type::DejavuTokenType,
    parser::element_type::DejavuElementType,
};
use oak_core::{OakError, RedNode, RedTree, Source};

impl<'config> DejavuBuilder<'config> {
    pub(crate) fn build_class<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<ClassDeclaration, OakError> {
        let span = node.span();
        let mut name = IdentifierNode { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut parents = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = IdentifierNode { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Attribute => {
                        annotations.push(self.build_attribute(n, source)?);
                    }
                    DejavuElementType::NamePath => {
                        parents.push(self.build_name_path(n, source)?);
                    }
                    DejavuElementType::Type => {
                        for child in n.children() {
                            if let RedTree::Node(inner) = child {
                                if inner.green.kind == DejavuElementType::NamePath {
                                    parents.push(self.build_name_path(inner, source)?);
                                }
                            }
                        }
                    }
                    DejavuElementType::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(ItemNode::Namespace(ns));
                    }
                    DejavuElementType::Class => {
                        let class = self.build_class(n, source)?;
                        items.push(ItemNode::Class(class));
                    }
                    DejavuElementType::Flags => {
                        let flags = self.build_flags(n, source)?;
                        items.push(ItemNode::Flags(flags));
                    }
                    DejavuElementType::Enums => {
                        let enums = self.build_enums(n, source)?;
                        items.push(ItemNode::Enum(enums));
                    }
                    DejavuElementType::Trait => {
                        let trait_node = self.build_trait(n, source)?;
                        items.push(ItemNode::Trait(trait_node));
                    }
                    DejavuElementType::Widget => {
                        let widget = self.build_widget(n, source)?;
                        items.push(ItemNode::Widget(widget));
                    }
                    DejavuElementType::UsingStatement => {
                        let us = self.build_using(n, source)?;
                        items.push(ItemNode::Using(us));
                    }
                    DejavuElementType::Micro => {
                        let micro = self.build_micro(n, source)?;
                        items.push(ItemNode::Micro(micro));
                    }
                    DejavuElementType::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(ItemNode::Statement(stmt));
                    }
                    DejavuElementType::ExprStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        items.push(ItemNode::Statement(stmt));
                    }
                    DejavuElementType::Variant => {
                        let variant = self.build_variant(n, source)?;
                        items.push(ItemNode::Variant(variant));
                    }
                    DejavuElementType::BlockExpression => {
                        for inner_child in n.children() {
                            if let RedTree::Node(inner_n) = inner_child {
                                if let Ok(item) = self.build_item(inner_n, source) {
                                    items.push(item);
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(ClassDeclaration { name, annotations, parents, items, span })
    }

    pub(crate) fn build_flags<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<FlagsDeclaration, OakError> {
        let span = node.span();
        let mut name = IdentifierNode { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = IdentifierNode { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Attribute => annotations.push(self.build_attribute(n, source)?),
                    DejavuElementType::Variant => {
                        let variant = self.build_variant(n, source)?;
                        items.push(ItemNode::Variant(variant));
                    }
                    DejavuElementType::BlockExpression => {
                        for inner_child in n.children() {
                            if let RedTree::Node(inner_n) = inner_child {
                                if let Ok(item) = self.build_item(inner_n, source) {
                                    items.push(item);
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(FlagsDeclaration { name, annotations, items, span })
    }

    pub(crate) fn build_enums<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<EnumDeclaration, OakError> {
        let span = node.span();
        let mut name = IdentifierNode { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = IdentifierNode { name: t_text, span: t.span.clone() }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Attribute => annotations.push(self.build_attribute(n, source)?),
                    DejavuElementType::Variant => {
                        let variant = self.build_variant(n, source)?;
                        items.push(ItemNode::Variant(variant))
                    }
                    DejavuElementType::BlockExpression => {
                        for inner_child in n.children() {
                            if let RedTree::Node(inner_n) = inner_child {
                                if let Ok(item) = self.build_item(inner_n, source) {
                                    items.push(item)
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(EnumDeclaration { name, annotations, items, span })
    }

    pub(crate) fn build_variant<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<VariantDefinition, OakError> {
        let span = node.span();
        let mut name = IdentifierNode { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut value = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = IdentifierNode { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Attribute => annotations.push(self.build_attribute(n, source)?),
                    _ => value = Some(self.build_expr(n, source)?),
                },
            }
        }
        Ok(VariantDefinition { name, annotations, value, span })
    }

    pub(crate) fn build_trait<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<TraitDeclaration, OakError> {
        let span = node.span();
        let mut name = IdentifierNode { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut parents = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = IdentifierNode { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Attribute => annotations.push(self.build_attribute(n, source)?),
                    DejavuElementType::NamePath => parents.push(self.build_name_path(n, source)?),
                    DejavuElementType::Type => {
                        for child in n.children() {
                            if let RedTree::Node(inner) = child {
                                if inner.green.kind == DejavuElementType::NamePath {
                                    parents.push(self.build_name_path(inner, source)?)
                                }
                            }
                        }
                    }
                    DejavuElementType::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(ItemNode::Namespace(ns))
                    }
                    DejavuElementType::Class => {
                        let class = self.build_class(n, source)?;
                        items.push(ItemNode::Class(class))
                    }
                    DejavuElementType::Flags => {
                        let flags = self.build_flags(n, source)?;
                        items.push(ItemNode::Flags(flags))
                    }
                    DejavuElementType::Enums => {
                        let enums = self.build_enums(n, source)?;
                        items.push(ItemNode::Enum(enums))
                    }
                    DejavuElementType::Trait => {
                        let trait_node = self.build_trait(n, source)?;
                        items.push(ItemNode::Trait(trait_node))
                    }
                    DejavuElementType::Widget => {
                        let widget = self.build_widget(n, source)?;
                        items.push(ItemNode::Widget(widget))
                    }
                    DejavuElementType::UsingStatement => {
                        let us = self.build_using(n, source)?;
                        items.push(ItemNode::Using(us))
                    }
                    DejavuElementType::Micro => {
                        let micro = self.build_micro(n, source)?;
                        items.push(ItemNode::Micro(micro))
                    }
                    DejavuElementType::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(ItemNode::Statement(stmt))
                    }
                    DejavuElementType::ExprStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        items.push(ItemNode::Statement(stmt))
                    }
                    DejavuElementType::BlockExpression => {
                        for inner_child in n.children() {
                            if let RedTree::Node(inner_n) = inner_child {
                                if let Ok(item) = self.build_item(inner_n, source) {
                                    items.push(item)
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(TraitDeclaration { name, annotations, parents, items, span })
    }

    pub(crate) fn build_widget<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<WidgetDeclaration, OakError> {
        let span = node.span();
        let mut name = IdentifierNode { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    DejavuTokenType::Identifier => {
                        name = IdentifierNode { name: text(source, t.span.clone().into()), span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Attribute => annotations.push(self.build_attribute(n, source)?),
                    DejavuElementType::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(ItemNode::Namespace(ns))
                    }
                    DejavuElementType::Class => {
                        let class = self.build_class(n, source)?;
                        items.push(ItemNode::Class(class))
                    }
                    DejavuElementType::Flags => {
                        let flags = self.build_flags(n, source)?;
                        items.push(ItemNode::Flags(flags))
                    }
                    DejavuElementType::Trait => {
                        let trait_node = self.build_trait(n, source)?;
                        items.push(ItemNode::Trait(trait_node))
                    }
                    DejavuElementType::Widget => {
                        let widget = self.build_widget(n, source)?;
                        items.push(ItemNode::Widget(widget))
                    }
                    DejavuElementType::UsingStatement => {
                        let us = self.build_using(n, source)?;
                        items.push(ItemNode::Using(us))
                    }
                    DejavuElementType::Micro => {
                        let micro = self.build_micro(n, source)?;
                        items.push(ItemNode::Micro(micro))
                    }
                    DejavuElementType::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(ItemNode::Statement(stmt))
                    }
                    DejavuElementType::ExprStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        items.push(ItemNode::Statement(stmt))
                    }
                    _ => {}
                },
            }
        }
        Ok(WidgetDeclaration { name, annotations, items, span })
    }
}
