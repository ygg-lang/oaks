use crate::RustParser;

use crate::{ast::*, language::RustLanguage, lexer::RustTokenType, parser::RustElementType};
use core::range::Range;
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, builder::BuildOutput, source::Source};
use std::sync::Arc;

mod build_generics;

pub use build_generics::*;

/// AST builder for the Rust language.
///
/// `RustBuilder` is responsible for parsing Rust source code and constructing
/// an Abstract Syntax Tree (AST). It uses a Pratt parser to handle operator
/// precedence in expressions and supports all Rust syntax features.
#[derive(Clone, Copy)]
pub struct RustBuilder<'config> {
    /// Language configuration
    config: &'config RustLanguage,
}

impl<'config> RustBuilder<'config> {
    /// Creates a new `RustBuilder` with the given configuration.
    pub const fn new(config: &'config RustLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<RustLanguage> for RustBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<RustLanguage>) -> BuildOutput<RustLanguage> {
        let parser = RustParser::new(self.config);
        let mut session = oak_core::parser::session::ParseSession::<RustLanguage>::default();
        let OakDiagnostics { result, diagnostics } = parser.parse(source, edits, &mut session);

        match result {
            Ok(green) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green, &source_text) {
                    Ok(root) => OakDiagnostics { result: Ok(root), diagnostics },
                    Err(e) => {
                        let mut diagnostics = diagnostics;
                        diagnostics.push(e.clone());
                        OakDiagnostics { result: Err(e), diagnostics }
                    }
                }
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics },
        }
    }
}

impl<'config> RustBuilder<'config> {
    /// Builds the root of the AST (the entire source file).
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, RustLanguage>, source: &SourceText) -> Result<RustRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut items = Vec::new();

        for child in red_root.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    RustElementType::Function => {
                        let func = self.build_function(n, source)?;
                        items.push(Item::Function(func));
                    }
                    RustElementType::StructItem => {
                        let struct_def = self.build_struct(n, source)?;
                        items.push(Item::Struct(struct_def));
                    }
                    RustElementType::EnumItem => {
                        let enum_def = self.build_enum(n, source)?;
                        items.push(Item::Enum(enum_def));
                    }
                    RustElementType::Trait => {
                        let trait_def = self.build_trait(n, source)?;
                        items.push(Item::Trait(trait_def));
                    }
                    RustElementType::Impl => {
                        let impl_block = self.build_impl(n, source)?;
                        items.push(Item::Impl(impl_block));
                    }
                    RustElementType::ModuleItem => {
                        let module = self.build_module(n, source)?;
                        items.push(Item::Module(module));
                    }
                    RustElementType::UseItem => {
                        let use_decl = self.build_use(n, source)?;
                        items.push(Item::Use(use_decl));
                    }
                    RustElementType::Const => {
                        let const_def = self.build_const(n, source)?;
                        items.push(Item::Const(const_def));
                    }
                    RustElementType::Static => {
                        let static_def = self.build_static(n, source)?;
                        items.push(Item::Static(static_def));
                    }
                    RustElementType::TypeAlias => {
                        let type_alias = self.build_type_alias(n, source)?;
                        items.push(Item::TypeAlias(type_alias));
                    }
                    _ => {
                        // Ignore other node types
                    }
                },
                RedTree::Leaf(_) => {
                    // Ignore top-level tokens (whitespace, comments, etc.)
                }
            }
        }
        let root = RustRoot { items };
        Ok(root)
    }

    /// Builds a function definition.
    pub(crate) fn build_function(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Arc<Function>, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut params = Vec::new();
        let mut return_type = None;
        let mut body = None;
        let generics = self.build_generics_and_where(node.clone(), source)?;
        let mut is_async = false;
        let mut is_unsafe = false;
        let mut is_extern = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    RustTokenType::Async => is_async = true,
                    RustTokenType::Unsafe => is_unsafe = true,
                    RustTokenType::Extern => is_extern = true,
                    RustTokenType::Identifier => {
                        name.name = text(source, t.span.clone().into());
                        name.span = t.span.clone().into();
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    RustElementType::ParameterList => {
                        params = self.build_param_list(n, source)?;
                    }
                    RustElementType::ReturnType => {
                        return_type = Some(self.build_type(n, source)?);
                    }
                    RustElementType::BlockExpression => {
                        body = Some(self.build_block(n, source)?);
                    }
                    _ => {}
                },
            }
        }

        let function = Arc::new(Function {
            name,
            params,
            return_type,
            body: body.unwrap_or_else(|| Block { statements: Vec::new(), block_start: 0, block_end: 0, nested: 0, span: Range { start: 0, end: 0 } }),
            is_async,
            is_unsafe,
            generics,
            is_extern,
            span: span.into(),
        });

        Ok(function)
    }

    /// Builds a parameter list.
    fn build_param_list(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Vec<Param>, OakError> {
        let mut params = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == RustElementType::Parameter {
                    params.push(self.build_param(n, source)?);
                }
            }
        }
        Ok(params)
    }

    /// Builds a parameter.
    fn build_param(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Param, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut ty = Type::Path("_".to_string());

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if let RustTokenType::Identifier = t.kind {
                        name.name = text(source, t.span.clone().into());
                        name.span = t.span.clone().into();
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    RustElementType::Type => {
                        ty = self.build_type(n, source)?;
                    }
                    _ => {}
                },
            }
        }

        Ok(Param { name, ty, is_mut: false, span: span.into() })
    }

    /// Builds a code block.
    fn build_block(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Block, OakError> {
        let span = node.span();
        let mut statements = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    RustElementType::LetStatement => {
                        statements.push(self.build_let_statement(n, source)?);
                    }
                    RustElementType::ExpressionStatement => {
                        statements.push(self.build_expr_statement(n, source)?);
                    }
                    RustElementType::ItemStatement => {
                        let item = self.build_item_statement(n, source)?;
                        statements.push(Statement::Item(item));
                    }
                    _ => {
                        // Could be a block expression, treat it as an expression statement
                        let span = n.span();
                        if let Ok(block_expr) = self.build_expr(n, source) {
                            statements.push(Statement::ExprStmt { expr: block_expr, semi: false, span: span.into() });
                        }
                    }
                },
                RedTree::Leaf(_) => {
                    // Ignore separators and whitespace
                }
            }
        }

        Ok(Block { statements, block_start: span.start, block_end: span.end, nested: 0, span: span.into() })
    }

    /// Builds a let statement.
    fn build_let_statement(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Statement, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut ty = None;
        let mut init = None;
        let mut mutable = false;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    RustElementType::Pattern => {
                        let pattern = self.build_pattern(n, source)?;
                        // Extract Identifier from Pattern
                        match pattern {
                            Pattern::Ident(ident) => name = ident,
                            _ => {
                                return Err(OakError::syntax_error("Expected identifier in let statement".to_string(), span.start, None));
                            }
                        }
                    }
                    RustElementType::Type => {
                        ty = Some(self.build_type(n, source)?);
                    }
                    RustElementType::Expression => {
                        init = Some(self.build_expr(n, source)?);
                    }
                    _ => {}
                },
                RedTree::Leaf(t) => {
                    // Check for mut keyword
                    if t.kind == RustTokenType::Mut {
                        mutable = true;
                    }
                }
            }
        }

        Ok(Statement::Let { name, ty, expr: init, mutable, span: span.into() })
    }

    /// Builds an expression statement.
    fn build_expr_statement(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Statement, OakError> {
        let span = node.span();
        let mut expr = Expr::Bool { value: false, span: span.clone().into() };
        let mut has_semicolon = false;

        for child in node.children() {
            match child {
                RedTree::Node(n) => {
                    if let Ok(expression) = self.build_expr(n, source) {
                        expr = expression;
                    }
                }
                RedTree::Leaf(t) => {
                    if t.kind == RustTokenType::Semicolon {
                        has_semicolon = true;
                    }
                }
            }
        }

        Ok(Statement::ExprStmt { expr, semi: has_semicolon, span: span.into() })
    }

    /// Builds an expression.
    pub(crate) fn build_expr(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        let span = node.span();

        match node.green.kind {
            RustElementType::IdentifierExpression => {
                for child in node.children() {
                    if let RedTree::Leaf(t) = child {
                        if t.kind == RustTokenType::Identifier {
                            let ident = Identifier { name: text(source, t.span.clone().into()), span: t.span.clone().into() };
                            return Ok(Expr::Ident(ident));
                        }
                    }
                }
                Err(OakError::syntax_error("Invalid identifier expression".to_string(), span.start, None))
            }
            RustElementType::LiteralExpression => {
                for child in node.children() {
                    if let RedTree::Leaf(t) = child {
                        // Infer literal type directly from token text
                        let text = source.get_text_in(t.span.clone().into());
                        if text == "true" {
                            return Ok(Expr::Bool { value: true, span: span.into() });
                        }
                        else if text == "false" {
                            return Ok(Expr::Bool { value: false, span: span.into() });
                        }
                        else {
                            // Other literal types (numbers, strings, characters, etc.)
                            return Ok(Expr::Literal { value: text.to_string(), span: span.into() });
                        }
                    }
                }
                Err(OakError::syntax_error("Invalid literal expression".to_string(), span.start, None))
            }
            RustElementType::BinaryExpression => {
                let mut left = None;
                let mut op = None;
                let mut right = None;

                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if left.is_none() {
                                left = Some(Box::new(self.build_expr(n, source)?));
                            }
                            else if right.is_none() {
                                right = Some(Box::new(self.build_expr(n, source)?));
                            }
                        }
                        RedTree::Leaf(t) => {
                            if op.is_none() {
                                // Infer operator type from token text
                                let text = source.get_text_in(t.span.clone().into());
                                op = match text.as_ref() {
                                    "+" => Some(RustTokenType::Plus),
                                    "-" => Some(RustTokenType::Minus),
                                    "*" => Some(RustTokenType::Star),
                                    "/" => Some(RustTokenType::Slash),
                                    "%" => Some(RustTokenType::Percent),
                                    "==" => Some(RustTokenType::EqEq),
                                    "!=" => Some(RustTokenType::Ne),
                                    "<" => Some(RustTokenType::Lt),
                                    "<=" => Some(RustTokenType::Le),
                                    ">" => Some(RustTokenType::Gt),
                                    ">=" => Some(RustTokenType::Ge),
                                    "&&" => Some(RustTokenType::AndAnd),
                                    "||" => Some(RustTokenType::OrOr),
                                    "&" => Some(RustTokenType::Ampersand),
                                    _ => None,
                                };
                            }
                        }
                    }
                }

                if let (Some(left), Some(op), Some(right)) = (left, op, right) { Ok(Expr::Binary { left, op, right, span: span.into() }) } else { Err(OakError::syntax_error("Invalid binary expression".to_string(), span.start, None)) }
            }
            RustElementType::UnaryExpression => {
                let mut op = None;
                let mut operand = None;

                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            operand = Some(Box::new(self.build_expr(n, source)?));
                        }
                        RedTree::Leaf(t) => {
                            if op.is_none() {
                                // Try to infer from the token text if available
                                let token_text = source.get_text_in(t.span.clone().into());
                                match token_text.as_ref() {
                                    "!" => op = Some(RustTokenType::Bang),
                                    "-" => op = Some(RustTokenType::Minus),
                                    "+" => op = Some(RustTokenType::Plus),
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                if let (Some(op), Some(operand)) = (op, operand) { Ok(Expr::Unary { op, expr: operand, span: span.into() }) } else { Err(OakError::syntax_error("Invalid unary expression".to_string(), span.start, None)) }
            }
            RustElementType::CallExpression => {
                let mut func = None;
                let mut args = Vec::new();

                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if func.is_none() {
                                func = Some(Box::new(self.build_expr(n, source)?));
                            }
                            else if n.green.kind == RustElementType::ArgumentList {
                                args = self.build_argument_list(n, source)?;
                            }
                        }
                        _ => {}
                    }
                }

                if let Some(func) = func { Ok(Expr::Call { callee: func, args, span: span.into() }) } else { Err(OakError::syntax_error("Invalid call expression".to_string(), span.start, None)) }
            }
            RustElementType::FieldExpression => {
                let mut base = None;
                let mut field = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };

                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            base = Some(Box::new(self.build_expr(n, source)?));
                        }
                        RedTree::Leaf(t) => {
                            if t.kind == RustTokenType::Identifier {
                                field.name = text(source, t.span.clone().into());
                                field.span = t.span.clone().into();
                            }
                        }
                    }
                }

                if let Some(receiver) = base { Ok(Expr::Field { receiver, field, span: span.into() }) } else { Err(OakError::syntax_error("Invalid field expression".to_string(), span.start, None)) }
            }
            RustElementType::IndexExpression => {
                let mut base = None;
                let mut index = None;

                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        if base.is_none() {
                            base = Some(Box::new(self.build_expr(n, source)?));
                        }
                        else if index.is_none() {
                            index = Some(Box::new(self.build_expr(n, source)?));
                        }
                    }
                }

                if let (Some(receiver), Some(index)) = (base, index) { Ok(Expr::Index { receiver, index, span: span.into() }) } else { Err(OakError::syntax_error("Invalid index expression".to_string(), span.start, None)) }
            }
            RustElementType::ParenthesizedExpression => {
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        let inner_expr = self.build_expr(n, source)?;
                        return Ok(Expr::Paren { expr: Box::new(inner_expr), span: span.into() });
                    }
                }
                Err(OakError::syntax_error("Invalid parenthesized expression".to_string(), span.start, None))
            }
            RustElementType::BlockExpression => {
                let block = self.build_block(node, source)?;
                Ok(Expr::Block(block))
            }
            _ => Err(OakError::syntax_error(format!("Unsupported expression type: {:?}", node.green.kind), span.start, None)),
        }
    }

    /// Builds a struct definition.
    pub(crate) fn build_struct(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Arc<Struct>, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut fields = Vec::new();
        let generics = self.build_generics_and_where(node.clone(), source)?;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) if t.kind == RustTokenType::Identifier => {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
                RedTree::Node(n) if n.green.kind == RustElementType::StructBody => {
                    fields = self.build_fields(n, source)?;
                }
                _ => {}
            }
        }

        let struct_def = Arc::new(Struct { name, generics, fields, span: span.into() });
        Ok(struct_def)
    }

    /// Builds a list of fields (for structs or enums).
    fn build_fields(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Vec<Field>, OakError> {
        let mut fields = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == RustElementType::Field {
                    fields.push(self.build_field(n, source)?);
                }
            }
        }
        Ok(fields)
    }

    /// Builds a single field.
    fn build_field(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Field, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut ty = Type::Path("_".to_string());

        for child in node.children() {
            match child {
                RedTree::Leaf(t) if t.kind == RustTokenType::Identifier => {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
                RedTree::Node(n) if n.green.kind == RustElementType::Type => {
                    ty = self.build_type(n, source)?;
                }
                _ => {}
            }
        }

        Ok(Field { name, ty, span: span.into() })
    }

    /// Builds an enum definition.
    pub(crate) fn build_enum(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Arc<Enum>, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut variants = Vec::new();
        let generics = self.build_generics_and_where(node.clone(), source)?;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) if t.kind == RustTokenType::Identifier => {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
                RedTree::Node(n) if n.green.kind == RustElementType::EnumBody => {
                    variants = self.build_variants(n, source)?;
                }
                _ => {}
            }
        }

        let enum_def = Arc::new(Enum { name, generics, variants, span: span.into() });
        Ok(enum_def)
    }

    /// Builds a list of enum variants.
    fn build_variants(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Vec<Variant>, OakError> {
        let mut variants = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == RustElementType::Variant {
                    variants.push(self.build_variant(n, source)?);
                }
            }
        }
        Ok(variants)
    }

    /// Builds a single enum variant.
    fn build_variant(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Variant, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut fields = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) if t.kind == RustTokenType::Identifier => {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
                RedTree::Node(n) if n.green.kind == RustElementType::TupleBody || n.green.kind == RustElementType::StructBody => {
                    fields = Some(self.build_fields(n, source)?);
                }
                _ => {}
            }
        }

        Ok(Variant { name, fields, span: span.into() })
    }

    /// Builds a trait definition.
    pub(crate) fn build_trait(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Arc<Trait>, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut items = Vec::new();
        let generics = self.build_generics_and_where(node.clone(), source)?;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) if t.kind == RustTokenType::Identifier => {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
                RedTree::Node(n) if n.green.kind == RustElementType::TraitBody => {
                    items = self.build_trait_items(n, source)?;
                }
                _ => {}
            }
        }

        let trait_def = Arc::new(Trait { name, generics, items, span: span.into() });
        Ok(trait_def)
    }

    /// Builds items within a trait.
    fn build_trait_items(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Vec<TraitItem>, OakError> {
        let mut items = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    RustElementType::Function => {
                        let func = self.build_function(n, source)?;
                        items.push(TraitItem::Method((*func).clone()));
                    }
                    RustElementType::TypeAlias => {
                        let alias = self.build_type_alias(n, source)?;
                        items.push(TraitItem::Type((*alias).clone()));
                    }
                    _ => {}
                }
            }
        }
        Ok(items)
    }

    /// Builds an impl block.
    pub(crate) fn build_impl(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Arc<Impl>, OakError> {
        let span = node.span();
        let mut ty = Type::Path("_".to_string());
        let mut trait_ = None;
        let mut items = Vec::new();
        let generics = self.build_generics_and_where(node.clone(), source)?;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    RustElementType::Type => {
                        // In Rust, `impl Trait for Type` or `impl Type`.
                        // The parser distinguishes these.
                        if trait_.is_none() {
                            ty = self.build_type(n, source)?;
                        }
                        else {
                            // This would be the type after 'for'
                            ty = self.build_type(n, source)?;
                        }
                    }
                    RustElementType::TraitRef => {
                        trait_ = Some(self.build_type(n, source)?);
                    }
                    RustElementType::ImplBody => {
                        items = self.build_impl_items(n, source)?;
                    }
                    _ => {}
                }
            }
        }

        let impl_block = Arc::new(Impl { generics, ty, trait_, items, span: span.into() });
        Ok(impl_block)
    }

    /// Builds items within an impl block.
    fn build_impl_items(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Vec<ImplItem>, OakError> {
        let mut items = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    RustElementType::Function => {
                        let func = self.build_function(n, source)?;
                        items.push(ImplItem::Method((*func).clone()));
                    }
                    RustElementType::TypeAlias => {
                        let alias = self.build_type_alias(n, source)?;
                        items.push(ImplItem::Type((*alias).clone()));
                    }
                    RustElementType::Const => {
                        let constant = self.build_const(n, source)?;
                        items.push(ImplItem::Const((*constant).clone()));
                    }
                    _ => {}
                }
            }
        }
        Ok(items)
    }

    /// Builds a module definition.
    pub(crate) fn build_module(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Arc<Module>, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) if t.kind == RustTokenType::Identifier => {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
                RedTree::Node(n) if n.green.kind == RustElementType::Block => {
                    // Reuse build_root logic for module items
                    let root = self.build_root(n.green, source)?;
                    items = root.items;
                }
                _ => {}
            }
        }

        let module = Arc::new(Module { name, items, span: span.into() });
        Ok(module)
    }

    /// Builds a use statement.
    pub(crate) fn build_use(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Arc<UseItem>, OakError> {
        let span = node.span();
        let mut path = String::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == RustTokenType::Identifier {
                    path = text(source, t.span().into());
                }
            }
        }

        let use_item = Arc::new(UseItem { path, span: span.into() });
        Ok(use_item)
    }

    /// Builds a constant definition.
    pub(crate) fn build_const(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Arc<Const>, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut ty = Type::Path("_".to_string());
        let mut expr = Expr::Bool { value: false, span: span.clone().into() };

        for child in node.children() {
            match child {
                RedTree::Leaf(t) if t.kind == RustTokenType::Identifier => {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
                RedTree::Node(n) => match n.green.kind {
                    RustElementType::Type => ty = self.build_type(n, source)?,
                    RustElementType::Expression => expr = self.build_expr(n, source)?,
                    _ => {}
                },
                _ => {}
            }
        }

        let constant = Arc::new(Const { name, ty, expr, span: span.into() });
        Ok(constant)
    }

    /// Builds a static definition.
    pub(crate) fn build_static(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Arc<Static>, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut ty = Type::Path("_".to_string());
        let mut expr = Expr::Bool { value: false, span: span.clone().into() };
        let mut mutable = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    RustTokenType::Identifier => {
                        name.name = text(source, t.span.clone().into());
                        name.span = t.span.clone().into();
                    }
                    RustTokenType::Mut => mutable = true,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    RustElementType::Type => ty = self.build_type(n, source)?,
                    RustElementType::Expression => expr = self.build_expr(n, source)?,
                    _ => {}
                },
            }
        }

        let static_def = Arc::new(Static { name, ty, expr, mutable, span: span.into() });
        Ok(static_def)
    }

    /// Builds a type alias.
    pub(crate) fn build_type_alias(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Arc<TypeAlias>, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut ty = Type::Path("_".to_string());
        let generics = self.build_generics_and_where(node.clone(), source)?;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) if t.kind == RustTokenType::Identifier => {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
                RedTree::Node(n) if n.green.kind == RustElementType::Type => {
                    ty = self.build_type(n, source)?;
                }
                _ => {}
            }
        }

        let type_alias = Arc::new(TypeAlias { name, generics, ty, span: span.into() });
        Ok(type_alias)
    }

    fn build_item_statement(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Item, OakError> {
        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    RustElementType::Function => return Ok(Item::Function(self.build_function(n, source)?)),
                    RustElementType::StructItem => return Ok(Item::Struct(self.build_struct(n, source)?)),
                    RustElementType::EnumItem => return Ok(Item::Enum(self.build_enum(n, source)?)),
                    RustElementType::Trait => return Ok(Item::Trait(self.build_trait(n, source)?)),
                    RustElementType::Impl => return Ok(Item::Impl(self.build_impl(n, source)?)),
                    RustElementType::ModuleItem => return Ok(Item::Module(self.build_module(n, source)?)),
                    RustElementType::UseItem => return Ok(Item::Use(self.build_use(n, source)?)),
                    RustElementType::Const => return Ok(Item::Const(self.build_const(n, source)?)),
                    RustElementType::Static => return Ok(Item::Static(self.build_static(n, source)?)),
                    RustElementType::TypeAlias => return Ok(Item::TypeAlias(self.build_type_alias(n, source)?)),
                    _ => {}
                }
            }
        }
        Err(OakError::syntax_error("Invalid item statement".to_string(), node.offset, None))
    }

    fn build_type(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Type, OakError> {
        let span = node.span();
        match node.green.kind {
            _ => Ok(Type::Path(text(source, span.into()))),
        }
    }

    fn build_pattern(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Pattern, OakError> {
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    RustTokenType::Identifier => {
                        return Ok(Pattern::Ident(Identifier { name: text(source, t.span.clone().into()), span: t.span.clone().into() }));
                    }
                    RustTokenType::Underscore => {
                        return Ok(Pattern::Wildcard);
                    }
                    _ => {}
                },
                RedTree::Node(n) => {
                    // Recurse into nested patterns if necessary
                    if let Ok(pattern) = self.build_pattern(n, source) {
                        if !matches!(pattern, Pattern::Wildcard) {
                            return Ok(pattern);
                        }
                    }
                }
            }
        }
        Ok(Pattern::Wildcard)
    }

    fn build_argument_list(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Vec<Expr>, OakError> {
        let mut args = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                args.push(self.build_expr(n, source)?);
            }
        }
        Ok(args)
    }
}

/// Helper function to extract text from source code.
#[inline]
pub(crate) fn text(source: &SourceText, span: Range<usize>) -> String {
    source.get_text_in(span.into()).to_string()
}
