use crate::{
    DejavuLanguage,
    ast::{ItemNode, NamePathNode, NamespaceDeclaration},
    builder::DejavuBuilder,
    lexer::token_type::DejavuTokenType,
    parser::element_type::DejavuElementType,
};
use oak_core::{OakError, RedNode, RedTree, Source};

impl<'config> DejavuBuilder<'config> {
    pub(crate) fn build_namespace<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<NamespaceDeclaration, OakError> {
        let span = node.span();
        let mut name = NamePathNode { parts: Vec::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    DejavuTokenType::Whitespace | DejavuTokenType::LineComment | DejavuTokenType::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    DejavuElementType::Attribute => {
                        annotations.push(self.build_attribute(n, source)?);
                    }
                    DejavuElementType::NamePath => {
                        name = self.build_name_path(n, source)?;
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
                    DejavuElementType::Mezzo => {
                        let mezzo = self.build_mezzo(n, source)?;
                        items.push(ItemNode::TypeFunction(mezzo));
                    }
                    DejavuElementType::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(ItemNode::Statement(stmt));
                    }
                    DejavuElementType::ExprStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        items.push(ItemNode::Statement(stmt));
                    }
                    DejavuElementType::BlockExpression => {
                        for inner_child in n.children() {
                            if let RedTree::Node(inner_n) = inner_child {
                                match inner_n.green.kind {
                                    DejavuElementType::Namespace => {
                                        let ns = self.build_namespace(inner_n, source)?;
                                        items.push(ItemNode::Namespace(ns));
                                    }
                                    DejavuElementType::Class => {
                                        let class = self.build_class(inner_n, source)?;
                                        items.push(ItemNode::Class(class));
                                    }
                                    DejavuElementType::Flags => {
                                        let flags = self.build_flags(inner_n, source)?;
                                        items.push(ItemNode::Flags(flags));
                                    }
                                    DejavuElementType::Enums => {
                                        let enums = self.build_enums(inner_n, source)?;
                                        items.push(ItemNode::Enum(enums));
                                    }
                                    DejavuElementType::Trait => {
                                        let trait_node = self.build_trait(inner_n, source)?;
                                        items.push(ItemNode::Trait(trait_node));
                                    }
                                    DejavuElementType::Widget => {
                                        let widget = self.build_widget(inner_n, source)?;
                                        items.push(ItemNode::Widget(widget));
                                    }
                                    DejavuElementType::UsingStatement => {
                                        let us = self.build_using(inner_n, source)?;
                                        items.push(ItemNode::Using(us));
                                    }
                                    DejavuElementType::Micro => {
                                        let micro = self.build_micro(inner_n, source)?;
                                        items.push(ItemNode::Micro(micro));
                                    }
                                    DejavuElementType::Mezzo => {
                                        let mezzo = self.build_mezzo(inner_n, source)?;
                                        items.push(ItemNode::TypeFunction(mezzo));
                                    }
                                    DejavuElementType::LetStatement => {
                                        let stmt = self.build_let(inner_n, source)?;
                                        items.push(ItemNode::Statement(stmt));
                                    }
                                    DejavuElementType::ExprStatement => {
                                        let stmt = self.build_expr_stmt(inner_n, source)?;
                                        items.push(ItemNode::Statement(stmt));
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
        Ok(NamespaceDeclaration { name, annotations, items, span })
    }
}
