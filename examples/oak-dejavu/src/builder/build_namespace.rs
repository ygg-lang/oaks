use crate::{
    DejavuLanguage, DejavuParser,
    ast::{Item, NamePath, Namespace},
    lexer::token_type::DejavuSyntaxKind,
};
use oak_core::{OakError, RedNode, RedTree, source::SourceText};

impl<'config> DejavuParser<'config> {
    pub(crate) fn build_namespace(&self, node: RedNode<DejavuLanguage>, source: &SourceText) -> Result<Namespace, OakError> {
        let span = node.span();
        let mut name = NamePath { parts: Vec::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Token(t) => match t.kind {
                    DejavuSyntaxKind::Whitespace | DejavuSyntaxKind::Newline | DejavuSyntaxKind::LineComment | DejavuSyntaxKind::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuSyntaxKind::Attribute => {
                        annotations.push(self.build_attribute(n, source)?);
                    }
                    DejavuSyntaxKind::NamePath => {
                        name = self.build_name_path(n, source)?;
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
                    DejavuSyntaxKind::Mezzo => {
                        let mezzo = self.build_mezzo(n, source)?;
                        items.push(Item::TypeFunction(mezzo));
                    }
                    DejavuSyntaxKind::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(Item::Statement(stmt));
                    }
                    DejavuSyntaxKind::ExpressionStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        items.push(Item::Statement(stmt));
                    }
                    DejavuSyntaxKind::BlockExpression => {
                        for inner_child in n.children() {
                            if let RedTree::Node(inner_n) = inner_child {
                                match inner_n.green.kind {
                                    DejavuSyntaxKind::Namespace => {
                                        let ns = self.build_namespace(inner_n, source)?;
                                        items.push(Item::Namespace(ns));
                                    }
                                    DejavuSyntaxKind::Class => {
                                        let class = self.build_class(inner_n, source)?;
                                        items.push(Item::Class(class));
                                    }
                                    DejavuSyntaxKind::Flags => {
                                        let flags = self.build_flags(inner_n, source)?;
                                        items.push(Item::Flags(flags));
                                    }
                                    DejavuSyntaxKind::Enums => {
                                        let enums = self.build_enums(inner_n, source)?;
                                        items.push(Item::Enums(enums));
                                    }
                                    DejavuSyntaxKind::Trait => {
                                        let trait_node = self.build_trait(inner_n, source)?;
                                        items.push(Item::Trait(trait_node));
                                    }
                                    DejavuSyntaxKind::Widget => {
                                        let widget = self.build_widget(inner_n, source)?;
                                        items.push(Item::Widget(widget));
                                    }
                                    DejavuSyntaxKind::UsingStatement => {
                                        let us = self.build_using(inner_n, source)?;
                                        items.push(Item::Using(us));
                                    }
                                    DejavuSyntaxKind::Micro => {
                                        let micro = self.build_micro(inner_n, source)?;
                                        items.push(Item::Micro(micro));
                                    }
                                    DejavuSyntaxKind::Mezzo => {
                                        let mezzo = self.build_mezzo(inner_n, source)?;
                                        items.push(Item::TypeFunction(mezzo));
                                    }
                                    DejavuSyntaxKind::LetStatement => {
                                        let stmt = self.build_let(inner_n, source)?;
                                        items.push(Item::Statement(stmt));
                                    }
                                    DejavuSyntaxKind::ExpressionStatement => {
                                        let stmt = self.build_expr_stmt(inner_n, source)?;
                                        items.push(Item::Statement(stmt));
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(Namespace { name, annotations, items, span })
    }
}
