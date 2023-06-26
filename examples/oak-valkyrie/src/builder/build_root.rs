use crate::{
    ValkyrieLanguage,
    ast::{Item, ValkyrieRoot},
    builder::ValkyrieBuilder,
    lexer::token_type::ValkyrieTokenType,
    parser::element_type::ValkyrieElementType,
};
use oak_core::{GreenNode, OakError, RedNode, RedTree, Source};

impl<'config> ValkyrieBuilder<'config> {
    /// Builds a strongly-typed AST from a green tree.
    pub fn build_root<S: Source + ?Sized>(&self, green_tree: &GreenNode<ValkyrieLanguage>, source: &S) -> Result<ValkyrieRoot, OakError> {
        let red_root = RedNode::<ValkyrieLanguage>::new(green_tree, 0);
        let mut items = Vec::new();
        for child in red_root.children() {
            match child {
                RedTree::Node(n) => match self.build_item(n, source) {
                    Ok(item) => items.push(item),
                    Err(err) => {
                        return Err(err);
                    }
                },
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Eof => continue,
                    _ => {
                        return Err(source.syntax_error(format!("Unexpected token in root: {:?}", t.kind), t.span.start));
                    }
                },
            }
        }
        Ok(ValkyrieRoot { items })
    }

    pub(crate) fn build_item<S: Source + ?Sized>(&self, n: RedNode<ValkyrieLanguage>, source: &S) -> Result<Item, OakError> {
        match n.green.kind {
            ValkyrieElementType::Namespace => {
                let ns = self.build_namespace(n, source)?;
                Ok(Item::Namespace(ns))
            }
            ValkyrieElementType::Class => {
                let class = self.build_class(n, source)?;
                Ok(Item::Class(class))
            }
            ValkyrieElementType::Flags => {
                let flags = self.build_flags(n, source)?;
                Ok(Item::Flags(flags))
            }
            ValkyrieElementType::Enums => {
                let enums = self.build_enums(n, source)?;
                Ok(Item::Enums(enums))
            }
            ValkyrieElementType::Trait => {
                let trait_node = self.build_trait(n, source)?;
                Ok(Item::Trait(trait_node))
            }
            ValkyrieElementType::Widget => {
                let widget = self.build_widget(n, source)?;
                Ok(Item::Widget(widget))
            }
            ValkyrieElementType::Singleton => {
                let singleton = self.build_singleton(n, source)?;
                Ok(Item::Singleton(singleton))
            }
            ValkyrieElementType::UsingStatement => {
                let us = self.build_using(n, source)?;
                Ok(Item::Using(us))
            }
            ValkyrieElementType::Micro => {
                let micro = self.build_micro(n, source)?;
                Ok(Item::Micro(micro))
            }
            ValkyrieElementType::Mezzo => {
                let mezzo = self.build_mezzo(n, source)?;
                Ok(Item::TypeFunction(mezzo))
            }
            ValkyrieElementType::LetStatement => {
                let stmt = self.build_let(n, source)?;
                Ok(Item::Statement(stmt))
            }
            ValkyrieElementType::ExprStatement => {
                let stmt = self.build_expr_stmt(n, source)?;
                Ok(Item::Statement(stmt))
            }
            ValkyrieElementType::Variant => {
                let variant = self.build_variant_decl(n, source)?;
                Ok(Item::Variant(variant))
            }
            ValkyrieElementType::EffectDefinition => {
                let effect = self.build_effect(n, source)?;
                Ok(Item::Effect(effect))
            }
            ValkyrieElementType::TemplateText => {
                let span = n.span();
                let content = source.get_text_in(span.clone()).to_string();
                Ok(Item::TemplateText { content, span })
            }
            ValkyrieElementType::TemplateControl => {
                let span = n.span();
                let mut items = Vec::new();
                for child in n.children() {
                    if let RedTree::Node(child_node) = child {
                        items.push(self.build_item(child_node, source)?);
                    }
                }
                Ok(Item::TemplateControl { items, span })
            }
            ValkyrieElementType::Interpolation => {
                let span = n.span();
                let mut expr = None;
                for child in n.children() {
                    if let RedTree::Node(child_node) = child {
                        expr = Some(self.build_expr(child_node, source)?);
                    }
                }
                let expr = expr.ok_or_else(|| source.syntax_error("Empty interpolation".to_string(), span.start))?;
                Ok(Item::TemplateInterpolation { expr, span })
            }
            _ => Err(source.syntax_error(format!("Unexpected item: {:?}", n.green.kind), n.span().start)),
        }
    }
}
