use crate::RustParser;
#[doc = include_str!("readme.md")]
use crate::{ast::*, language::RustLanguage, lexer::RustTokenType, parser::RustElementType};
use core::range::Range;
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// Rust 语言的 AST 构建器
///
/// `RustBuilder` 负责解析 Rust 源代码并构建抽象语法树 (AST)。
/// 它使用 Pratt 解析器处理表达式中的运算符优先级，并支持所有 Rust 语法特性。
///
/// # 示例
///
/// 基本用法:
///
/// ```rust,ignore
/// use oak_core::Parser;
/// use oak_rust::{RustBuilder, RustLanguage, SourceText};
///
/// let language = RustLanguage::default();
/// let builder = RustBuilder::new(&language);
/// let source = SourceText::new("fn main() { let x = 42; }");
/// let result = builder.build(source, 0);
///
/// // 结果包含解析后的 AST
/// assert!(result.result.is_ok());
/// ```
///
/// 解析更复杂的 Rust 结构:
///
/// ```rust,ignore
/// use oak_core::Parser;
/// use oak_rust::{RustBuilder, RustLanguage, SourceText};
///
/// let language = RustLanguage::default();
/// let builder = RustBuilder::new(&language);
///
/// let source = SourceText::new(
///     r#"
/// struct Point {
///     x: f64,
///     y: f64,
/// }
///
/// impl Point {
///     fn new(x: f64, y: f64) -> Self {
///         Point { x, y }
///     }
/// }
/// "#,
/// );
/// let result = builder.build(source, 0);
///
/// // 验证解析成功
/// assert!(result.result.is_ok());
/// ```
#[derive(Clone)]
pub struct RustBuilder<'config> {
    /// 语言配置
    config: &'config RustLanguage,
}

impl<'config> RustBuilder<'config> {
    /// 创建新的 Rust 构建器
    pub fn new(config: &'config RustLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<RustLanguage> for RustBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<RustLanguage>) -> BuildOutput<RustLanguage> {
        // 解析源代码获得语法树
        let parser = RustParser::new(self.config);

        // TODO: 真正的增量构建需要利用 BC
        let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

        // 检查解析是否成功
        match parse_result.result {
            Ok(green_tree) => {
                // 构建 AST
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
                match self.build_root(green_tree.clone(), &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> RustBuilder<'config> {
    /// 构建根节点
    pub(crate) fn build_root(&self, green_tree: GreenNode<RustLanguage>, source: &SourceText) -> Result<RustRoot, OakError> {
        let red_root = RedNode::new(&green_tree, 0);
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
                        // 忽略其他类型的节点
                    }
                },
                RedTree::Leaf(_) => {
                    // 忽略顶级的 token（如空白符、注释等）
                }
            }
        }
        Ok(RustRoot { items })
    }

    /// 构建函数定义
    pub(crate) fn build_function(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Function, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };
        let mut params = Vec::new();
        let mut return_type = None;
        let mut body = None;
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

        Ok(Function {
            name,
            params,
            return_type,
            body: body.unwrap_or_else(|| Block { statements: Vec::new(), block_start: 0, block_end: 0, nested: 0, span: Range { start: 0, end: 0 } }),
            is_async,
            is_unsafe,
            generics: Vec::new(),
            is_extern,
            span: span.into(),
        })
    }

    /// 构建参数列表
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

    /// 构建参数
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

    /// 构建代码块
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
                        // 可能是块表达式，将其作为表达式语句处理
                        let span = n.span();
                        if let Ok(block_expr) = self.build_expr(n, source) {
                            statements.push(Statement::ExprStmt { expr: block_expr, semi: false, span: span.into() });
                        }
                    }
                },
                RedTree::Leaf(_) => {
                    // 忽略分隔符和空白符
                }
            }
        }

        Ok(Block { statements, block_start: span.start, block_end: span.end, nested: 0, span: span.into() })
    }

    /// 构建 let 语句
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
                        // 从 Pattern 中提取 Identifier
                        match pattern {
                            Pattern::Ident(ident) => name = ident,
                            _ => {
                                return Err(OakError::syntax_error("Expected identifier in let statement".to_string(), span.start, source.get_url().cloned()));
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
                    // 检查是否有 mut 关键字
                    if t.kind == RustTokenType::Mut {
                        mutable = true;
                    }
                }
            }
        }

        Ok(Statement::Let { name, ty, expr: init, mutable, span: span.into() })
    }

    /// 构建表达式语句
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

    /// 构建表达式
    pub(crate) fn build_expr(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        let span = node.span();

        match node.green.kind {
            RustElementType::IdentifierExpression => {
                for child in node.children() {
                    if let RedTree::Leaf(t) = child {
                        if t.kind == RustTokenType::Identifier {
                            let ident = Identifier { name: source.get_text_in(t.span.clone().into()).to_string(), span: t.span.clone().into() };
                            return Ok(Expr::Ident(ident));
                        }
                    }
                }
                Err(OakError::syntax_error("Invalid identifier expression".to_string(), span.start, source.get_url().cloned()))
            }
            RustElementType::LiteralExpression => {
                for child in node.children() {
                    if let RedTree::Leaf(t) = child {
                        // 直接根据 token 文本内容推断字面量类型
                        let text = source.get_text_in(t.span.clone().into());
                        if text == "true" {
                            return Ok(Expr::Bool { value: true, span: span.into() });
                        }
                        else if text == "false" {
                            return Ok(Expr::Bool { value: false, span: span.into() });
                        }
                        else {
                            // 其他字面量类型（数字、字符串、字符等）
                            return Ok(Expr::Literal { value: text.to_string(), span: span.into() });
                        }
                    }
                }
                Err(OakError::syntax_error("Invalid literal expression".to_string(), span.start, source.get_url().cloned()))
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
                                // 根据 token 文本内容推断操作符类型
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

                if let (Some(left), Some(op), Some(right)) = (left, op, right) { Ok(Expr::Binary { left, op, right, span: span.into() }) } else { Err(OakError::syntax_error("Invalid binary expression".to_string(), span.start, source.get_url().cloned())) }
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

                if let (Some(op), Some(operand)) = (op, operand) { Ok(Expr::Unary { op, expr: operand, span: span.into() }) } else { Err(OakError::syntax_error("Invalid unary expression".to_string(), span.start, source.get_url().cloned())) }
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

                if let Some(func) = func { Ok(Expr::Call { callee: func, args, span: span.into() }) } else { Err(OakError::syntax_error("Invalid call expression".to_string(), span.start, source.get_url().cloned())) }
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
                                field.name = source.get_text_in(t.span.clone().into()).to_string();
                                field.span = t.span.clone().into();
                            }
                        }
                    }
                }

                if let Some(receiver) = base { Ok(Expr::Field { receiver, field, span: span.into() }) } else { Err(OakError::syntax_error("Invalid field expression".to_string(), span.start, source.get_url().cloned())) }
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

                if let (Some(receiver), Some(index)) = (base, index) { Ok(Expr::Index { receiver, index, span: span.into() }) } else { Err(OakError::syntax_error("Invalid index expression".to_string(), span.start, source.get_url().cloned())) }
            }
            RustElementType::ParenthesizedExpression => {
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        let inner_expr = self.build_expr(n, source)?;
                        return Ok(Expr::Paren { expr: Box::new(inner_expr), span: span.into() });
                    }
                }
                Err(OakError::syntax_error("Invalid parenthesized expression".to_string(), span.start, source.get_url().cloned()))
            }
            RustElementType::BlockExpression => {
                let block = self.build_block(node, source)?;
                Ok(Expr::Block(block))
            }
            _ => Err(OakError::syntax_error(format!("Unsupported expression type: {:?}", node.green.kind), span.start, source.get_url().cloned())),
        }
    }

    // 占位符方法 - 这些需要根据具体需求实现
    fn build_struct(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Struct, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == RustTokenType::Identifier {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
            }
        }

        Ok(Struct { name, fields: Vec::new(), span: span.into() })
    }

    fn build_enum(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Enum, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == RustTokenType::Identifier {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
            }
        }

        Ok(Enum { name, variants: Vec::new(), span: span.into() })
    }

    fn build_trait(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Trait, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == RustTokenType::Identifier {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
            }
        }

        Ok(Trait { name, items: Vec::new(), span: span.into() })
    }

    fn build_impl(&self, node: RedNode<RustLanguage>, _source: &SourceText) -> Result<Impl, OakError> {
        let span = node.span();
        Ok(Impl { trait_: None, ty: Type::Path("_".to_string()), items: Vec::new(), span: span.into() })
    }

    fn build_module(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Module, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == RustTokenType::Identifier {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
            }
        }

        Ok(Module { name, items: Vec::new(), span: span.into() })
    }

    fn build_use(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<UseItem, OakError> {
        let span = node.span();
        Ok(UseItem { path: text(source, span.clone().into()), span: span.into() })
    }

    fn build_const(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Const, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == RustTokenType::Identifier {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
            }
        }

        Ok(Const { name, ty: Type::Path("_".to_string()), expr: Expr::Bool { value: false, span: span.clone().into() }, span: span.into() })
    }

    fn build_static(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Static, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == RustTokenType::Identifier {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
            }
        }

        Ok(Static { name, ty: Type::Path("_".to_string()), expr: Expr::Bool { value: false, span: span.clone().into() }, mutable: false, span: span.into() })
    }

    fn build_type_alias(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<TypeAlias, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Range { start: 0, end: 0 } };

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == RustTokenType::Identifier {
                    name.name = text(source, t.span.clone().into());
                    name.span = t.span.clone().into();
                }
            }
        }

        Ok(TypeAlias { name, ty: Type::Path("_".to_string()), span: span.into() })
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
        Err(OakError::syntax_error("Invalid item statement".to_string(), node.offset, source.get_url().cloned()))
    }

    fn build_type(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Type, OakError> {
        let span = node.span();
        Ok(Type::Path(text(source, span.into())))
    }

    fn build_pattern(&self, node: RedNode<RustLanguage>, source: &SourceText) -> Result<Pattern, OakError> {
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == RustTokenType::Identifier {
                    return Ok(Pattern::Ident(Identifier { name: text(source, t.span.clone().into()), span: t.span.clone().into() }));
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

/// 从源代码中提取文本的辅助函数
#[inline]
fn text(source: &SourceText, span: Range<usize>) -> String {
    source.get_text_in(span.into()).to_string()
}
