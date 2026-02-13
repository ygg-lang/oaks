use crate::{
    DejavuLanguage, DejavuParser,
    ast::{Class, Enums, Flags, Identifier, Item, Trait, Variant, Widget},
    builder::text,
    lexer::token_type::DejavuSyntaxKind,
};
use oak_core::{OakError, RedNode, RedTree, source::SourceText};

impl<'config> DejavuParser<'config> {
    pub(crate) fn build_class(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Class, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut parents = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Attribute => {
                        annotations.push(self.build_attribute(n, source)?);
                    }
                    DejavuSyntaxKind::NamePath => {
                        parents.push(self.build_name_path(n, source)?);
                    }
                    DejavuSyntaxKind::Type => {
                        for child in n.children() {
                            if let RedTree::Node(inner) = child {
                                if inner.green.kind == DejavuSyntaxKind::NamePath {
                                    parents.push(self.build_name_path(inner, source)?);
                                }
                            }
                        }
                    }
                    DejavuSyntaxKind::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(Item::Namespace(ns));
                    }
                    DejavuSyntaxKind::Class => {
                        let class = self.build_class(n, source)?;
                        items.push(Item::Class(class));
                    }
                    DejavuSyntaxKind::Flags => {
                        let flags = self.build_flags(n, source)?;
                        items.push(Item::Flags(flags));
                    }
                    DejavuSyntaxKind::Enums => {
                        let enums = self.build_enums(n, source)?;
                        items.push(Item::Enums(enums));
                    }
                    DejavuSyntaxKind::Trait => {
                        let trait_node = self.build_trait(n, source)?;
                        items.push(Item::Trait(trait_node));
                    }
                    DejavuSyntaxKind::Widget => {
                        let widget = self.build_widget(n, source)?;
                        items.push(Item::Widget(widget));
                    }
                    DejavuSyntaxKind::UsingStatement => {
                        let us = self.build_using(n, source)?;
                        items.push(Item::Using(us));
                    }
                    DejavuSyntaxKind::Micro => {
                        let micro = self.build_micro(n, source)?;
                        items.push(Item::Micro(micro));
                    }
                    DejavuSyntaxKind::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(Item::Statement(stmt));
                    }
                    DejavuSyntaxKind::ExpressionStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        items.push(Item::Statement(stmt));
                    }
                    DejavuSyntaxKind::Variant => {
                        let variant = self.build_variant(n, source)?;
                        items.push(Item::Variant(variant));
                    }
                    DejavuSyntaxKind::BlockExpression => {
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
        Ok(Class { name, annotations, parents, items, span })
    }

    pub(crate) fn build_flags(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Flags, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    DejavuSyntaxKind::Variant => {
                        let variant = self.build_variant(n, source)?;
                        items.push(Item::Variant(variant));
                    }
                    DejavuSyntaxKind::BlockExpression => {
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
        Ok(Flags { name, annotations, items, span })
    }

    pub(crate) fn build_enums(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Enums, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    DejavuSyntaxKind::Variant => {
                        let variant = self.build_variant(n, source)?;
                        items.push(Item::Variant(variant))
                    }
                    DejavuSyntaxKind::BlockExpression => {
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
        Ok(Enums { name, annotations, items, span })
    }

    pub(crate) fn build_variant(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Variant, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut value = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    _ => value = Some(self.build_expr(n, source)?),
                },
            }
        }
        Ok(Variant { name, annotations, value, span })
    }

    pub(crate) fn build_trait(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Trait, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut parents = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    DejavuSyntaxKind::NamePath => parents.push(self.build_name_path(n, source)?),
                    DejavuSyntaxKind::Type => {
                        for child in n.children() {
                            if let RedTree::Node(inner) = child {
                                if inner.green.kind == DejavuSyntaxKind::NamePath {
                                    parents.push(self.build_name_path(inner, source)?)
                                }
                            }
                        }
                    }
                    DejavuSyntaxKind::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(Item::Namespace(ns))
                    }
                    DejavuSyntaxKind::Class => {
                        let class = self.build_class(n, source)?;
                        items.push(Item::Class(class))
                    }
                    DejavuSyntaxKind::Flags => {
                        let flags = self.build_flags(n, source)?;
                        items.push(Item::Flags(flags))
                    }
                    DejavuSyntaxKind::Enums => {
                        let enums = self.build_enums(n, source)?;
                        items.push(Item::Enums(enums))
                    }
                    DejavuSyntaxKind::Trait => {
                        let trait_node = self.build_trait(n, source)?;
                        items.push(Item::Trait(trait_node))
                    }
                    DejavuSyntaxKind::Widget => {
                        let widget = self.build_widget(n, source)?;
                        items.push(Item::Widget(widget))
                    }
                    DejavuSyntaxKind::UsingStatement => {
                        let us = self.build_using(n, source)?;
                        items.push(Item::Using(us))
                    }
                    DejavuSyntaxKind::Micro => {
                        let micro = self.build_micro(n, source)?;
                        items.push(Item::Micro(micro))
                    }
                    DejavuSyntaxKind::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(Item::Statement(stmt))
                    }
                    DejavuSyntaxKind::ExpressionStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        items.push(Item::Statement(stmt))
                    }
                    DejavuSyntaxKind::BlockExpression => {
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
        Ok(Trait { name, annotations, parents, items, span })
    }

    pub(crate) fn build_widget(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Widget, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    DejavuSyntaxKind::Identifier => {
                        name = Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    DejavuSyntaxKind::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(Item::Namespace(ns))
                    }
                    DejavuSyntaxKind::Class => {
                        let class = self.build_class(n, source)?;
                        items.push(Item::Class(class))
                    }
                    DejavuSyntaxKind::Flags => {
                        let flags = self.build_flags(n, source)?;
                        items.push(Item::Flags(flags))
                    }
                    DejavuSyntaxKind::Trait => {
                        let trait_node = self.build_trait(n, source)?;
                        items.push(Item::Trait(trait_node))
                    }
                    DejavuSyntaxKind::Widget => {
                        let widget = self.build_widget(n, source)?;
                        items.push(Item::Widget(widget))
                    }
                    DejavuSyntaxKind::UsingStatement => {
                        let us = self.build_using(n, source)?;
                        items.push(Item::Using(us))
                    }
                    DejavuSyntaxKind::Micro => {
                        let micro = self.build_micro(n, source)?;
                        items.push(Item::Micro(micro))
                    }
                    DejavuSyntaxKind::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(Item::Statement(stmt))
                    }
                    DejavuSyntaxKind::ExpressionStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        items.push(Item::Statement(stmt))
                    }
                    _ => {}
                },
            }
        }
        Ok(Widget { name, annotations, items, span })
    }
}
