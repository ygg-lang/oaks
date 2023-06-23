use crate::{
    ValkyrieLanguage, ValkyrieParser,
    ast::{Class, Enums, Flags, Identifier, Item, Trait, Variant, Widget},
    builder::text,
    lexer::token_type::ValkyrieSyntaxKind,
};
use oak_core::{OakError, RedNode, RedTree, source::SourceText};

impl<'config> ValkyrieParser<'config> {
    pub(crate) fn build_class(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Class, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut parents = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => {
                        annotations.push(self.build_attribute(n, source)?);
                    }
                    ValkyrieSyntaxKind::NamePath => {
                        parents.push(self.build_name_path(n, source)?);
                    }
                    ValkyrieSyntaxKind::Type => {
                        for child in n.children() {
                            if let RedTree::Node(inner) = child {
                                if inner.green.kind == ValkyrieSyntaxKind::NamePath {
                                    parents.push(self.build_name_path(inner, source)?);
                                }
                            }
                        }
                    }
                    ValkyrieSyntaxKind::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(Item::Namespace(ns));
                    }
                    ValkyrieSyntaxKind::Class => {
                        let class = self.build_class(n, source)?;
                        items.push(Item::Class(class));
                    }
                    ValkyrieSyntaxKind::Flags => {
                        let flags = self.build_flags(n, source)?;
                        items.push(Item::Flags(flags));
                    }
                    ValkyrieSyntaxKind::Enums => {
                        let enums = self.build_enums(n, source)?;
                        items.push(Item::Enums(enums));
                    }
                    ValkyrieSyntaxKind::Trait => {
                        let trait_node = self.build_trait(n, source)?;
                        items.push(Item::Trait(trait_node));
                    }
                    ValkyrieSyntaxKind::Widget => {
                        let widget = self.build_widget(n, source)?;
                        items.push(Item::Widget(widget));
                    }
                    ValkyrieSyntaxKind::UsingStatement => {
                        let us = self.build_using(n, source)?;
                        items.push(Item::Using(us));
                    }
                    ValkyrieSyntaxKind::Micro => {
                        let micro = self.build_micro(n, source)?;
                        items.push(Item::Micro(micro));
                    }
                    ValkyrieSyntaxKind::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(Item::Statement(stmt));
                    }
                    ValkyrieSyntaxKind::ExpressionStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        items.push(Item::Statement(stmt));
                    }
                    ValkyrieSyntaxKind::Variant => {
                        let variant = self.build_variant(n, source)?;
                        items.push(Item::Variant(variant));
                    }
                    ValkyrieSyntaxKind::BlockExpression => {
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

    pub(crate) fn build_flags(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Flags, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    ValkyrieSyntaxKind::Variant => {
                        let variant = self.build_variant(n, source)?;
                        items.push(Item::Variant(variant));
                    }
                    ValkyrieSyntaxKind::BlockExpression => {
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

    pub(crate) fn build_enums(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Enums, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    ValkyrieSyntaxKind::Variant => {
                        let variant = self.build_variant(n, source)?;
                        items.push(Item::Variant(variant))
                    }
                    ValkyrieSyntaxKind::BlockExpression => {
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

    pub(crate) fn build_variant(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Variant, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut value = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    _ => value = Some(self.build_expr(n, source)?),
                },
            }
        }
        Ok(Variant { name, annotations, value, span })
    }

    pub(crate) fn build_trait(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Trait, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut parents = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    ValkyrieSyntaxKind::NamePath => parents.push(self.build_name_path(n, source)?),
                    ValkyrieSyntaxKind::Type => {
                        for child in n.children() {
                            if let RedTree::Node(inner) = child {
                                if inner.green.kind == ValkyrieSyntaxKind::NamePath {
                                    parents.push(self.build_name_path(inner, source)?)
                                }
                            }
                        }
                    }
                    ValkyrieSyntaxKind::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(Item::Namespace(ns))
                    }
                    ValkyrieSyntaxKind::Class => {
                        let class = self.build_class(n, source)?;
                        items.push(Item::Class(class))
                    }
                    ValkyrieSyntaxKind::Flags => {
                        let flags = self.build_flags(n, source)?;
                        items.push(Item::Flags(flags))
                    }
                    ValkyrieSyntaxKind::Enums => {
                        let enums = self.build_enums(n, source)?;
                        items.push(Item::Enums(enums))
                    }
                    ValkyrieSyntaxKind::Trait => {
                        let trait_node = self.build_trait(n, source)?;
                        items.push(Item::Trait(trait_node))
                    }
                    ValkyrieSyntaxKind::Widget => {
                        let widget = self.build_widget(n, source)?;
                        items.push(Item::Widget(widget))
                    }
                    ValkyrieSyntaxKind::UsingStatement => {
                        let us = self.build_using(n, source)?;
                        items.push(Item::Using(us))
                    }
                    ValkyrieSyntaxKind::Micro => {
                        let micro = self.build_micro(n, source)?;
                        items.push(Item::Micro(micro))
                    }
                    ValkyrieSyntaxKind::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(Item::Statement(stmt))
                    }
                    ValkyrieSyntaxKind::ExpressionStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        items.push(Item::Statement(stmt))
                    }
                    ValkyrieSyntaxKind::BlockExpression => {
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

    pub(crate) fn build_widget(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Widget, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        name = Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    ValkyrieSyntaxKind::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(Item::Namespace(ns))
                    }
                    ValkyrieSyntaxKind::Class => {
                        let class = self.build_class(n, source)?;
                        items.push(Item::Class(class))
                    }
                    ValkyrieSyntaxKind::Flags => {
                        let flags = self.build_flags(n, source)?;
                        items.push(Item::Flags(flags))
                    }
                    ValkyrieSyntaxKind::Trait => {
                        let trait_node = self.build_trait(n, source)?;
                        items.push(Item::Trait(trait_node))
                    }
                    ValkyrieSyntaxKind::Widget => {
                        let widget = self.build_widget(n, source)?;
                        items.push(Item::Widget(widget))
                    }
                    ValkyrieSyntaxKind::UsingStatement => {
                        let us = self.build_using(n, source)?;
                        items.push(Item::Using(us))
                    }
                    ValkyrieSyntaxKind::Micro => {
                        let micro = self.build_micro(n, source)?;
                        items.push(Item::Micro(micro))
                    }
                    ValkyrieSyntaxKind::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(Item::Statement(stmt))
                    }
                    ValkyrieSyntaxKind::ExpressionStatement => {
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
