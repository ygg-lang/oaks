use crate::{
    ValkyrieLanguage,
    ast::*,
    builder::{ValkyrieBuilder, text},
    lexer::token_type::ValkyrieTokenType,
};
use oak_core::{OakError, RedNode, RedTree, Source};

impl<'config> ValkyrieBuilder<'config> {
    /// Builds an anonymous class expression.
    ///
    /// Syntax: `class { ... }` or `class: Trait { ... }`
    pub(crate) fn build_anonymous_class<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let mut parents = Vec::new();
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut captures = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Identifier => {
                        parents.push(text(source, t.span));
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    crate::parser::element_type::ValkyrieElementType::NamePath => {
                        let path = self.build_name_path(n, source)?;
                        if let Some(first) = path.parts.first() {
                            parents.push(first.name.clone());
                        }
                    }
                    crate::parser::element_type::ValkyrieElementType::Field => {
                        if let Ok(field) = self.build_field(n, source) {
                            fields.push(field);
                        }
                    }
                    crate::parser::element_type::ValkyrieElementType::Micro => {
                        if let Ok(method) = self.build_function(n, source) {
                            methods.push(method);
                        }
                    }
                    _ => {}
                },
            }
        }

        Ok(TermExpression::AnonymousClass(Box::new(AnonymousClass { parents, fields, methods, captures, span })))
    }

    /// Builds a super call expression for constructor chaining.
    ///
    /// Syntax: `super.initiate(args)` or `super.alias.initiate(args)`
    ///
    /// ```v
    /// class Derived(Base) {
    ///     initiate(mut self, x: i32) {
    ///         super.initiate(x)  // Call parent constructor
    ///     }
    /// }
    ///
    /// class Child(primary: ParentA, secondary: ParentB) {
    ///     initiate(mut self) {
    ///         super.primary.initiate()  // Call specific parent
    ///         super.secondary.initiate()
    ///     }
    /// }
    /// ```
    pub(crate) fn build_super_call<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let mut parent_alias = None;
        let mut method = None;
        let mut args = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieTokenType::Whitespace | ValkyrieTokenType::Newline | ValkyrieTokenType::LineComment | ValkyrieTokenType::BlockComment => continue,
                    ValkyrieTokenType::Identifier => {
                        if method.is_none() {
                            method = Some(Identifier { name: text(source, t.span), span: t.span });
                        }
                        else if parent_alias.is_none() {
                            parent_alias = method.take();
                            method = Some(Identifier { name: text(source, t.span), span: t.span });
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    crate::parser::element_type::ValkyrieElementType::Whitespace
                    | crate::parser::element_type::ValkyrieElementType::Newline
                    | crate::parser::element_type::ValkyrieElementType::LineComment
                    | crate::parser::element_type::ValkyrieElementType::BlockComment => continue,
                    crate::parser::element_type::ValkyrieElementType::ArgList => {
                        for arg_child in n.children() {
                            if let RedTree::Node(arg_n) = arg_child {
                                if let Ok(arg) = self.build_expr(arg_n, source) {
                                    args.push(arg);
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }

        let method = method.ok_or_else(|| source.syntax_error("Missing method name in super call".to_string(), span.start))?;

        Ok(TermExpression::SuperCall { parent_alias, method, args, span })
    }
}
