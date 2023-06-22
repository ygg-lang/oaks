#[doc = include_str!("readme.md")]
use crate::{
    ValkyrieLanguage, ValkyrieParser,
    ast::{ValkyrieRoot, *},
    kind::ValkyrieSyntaxKind,
    lexer::ValkyrieKeywords,
};
use core::range::Range;
use oak_core::{
    Builder, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText,
    builder::{BuildOutput, BuilderCache},
    source::{Source, TextEdit},
};

/// A builder for the Valkyrie programming language.
///
/// The `ValkyrieParser` is responsible for parsing Valkyrie source code and building an Abstract Syntax Tree (AST).
/// It uses a Pratt parser for handling operator precedence in expressions and supports all Valkyrie syntax features.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use oak_core::{Parser, parser::ParseSession, source::SourceText};
/// use oak_valkyrie::{ValkyrieLanguage, ValkyrieParser};
///
/// let language = ValkyrieLanguage::default();
/// let parser = ValkyrieParser::new(&language);
/// let source = SourceText::new("namespace Main { fn main() { let x = 42; } }");
/// let mut cache = ParseSession::default();
/// let result = parser.parse(&source, &[], &mut cache);
///
/// // The result contains the parsed AST
/// assert!(result.result.is_ok());
/// ```
///
/// Parsing a more complex Valkyrie structure:
///
/// ```
/// use oak_core::{Parser, parser::ParseSession, source::SourceText};
/// use oak_valkyrie::{ValkyrieLanguage, ValkyrieParser};
///
/// let language = ValkyrieLanguage::default();
/// let parser = ValkyrieParser::new(&language);
/// let mut cache = ParseSession::default();
///
/// let source = SourceText::new(
///     r#"
/// namespace Math {
///     fn add(x: i32, y: i32) -> i32 {
///         x + y
///     }
///
///     micro Calculator {
///         fn multiply(a: f64, b: f64) -> f64 {
///             a * b
///         }
///     }
/// }
/// "#,
/// );
/// let result = parser.parse(&source, &[], &mut cache);
///
/// // Verify that parsing succeeded
/// assert!(result.result.is_ok());
/// ```
#[derive(Clone)]
pub struct ValkyrieBuilder<'config> {
    /// Language configuration
    config: &'config ValkyrieLanguage,
}

impl<'config> ValkyrieBuilder<'config> {
    /// Creates a new Valkyrie builder.
    pub fn new(config: &'config ValkyrieLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<ValkyrieLanguage> for ValkyrieBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<ValkyrieLanguage>) -> BuildOutput<ValkyrieLanguage> {
        let parser = ValkyrieParser::new(self.config);
        // let lexer = ValkyrieLexer::new(self.config);

        // 使用解析器获取绿树
        let mut parse_cache = oak_core::parser::ParseSession::<ValkyrieLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        // 检查解析是否成功
        match parse_result.result {
            Ok(green_tree) => {
                // 提前构造 SourceText 引用以便后续 AST 构建
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
                // 构建 AST
                match parser.build_root(green_tree, &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error);
                        OakDiagnostics { result: Err(OakError::custom_error("Failed to build AST")), diagnostics }
                    }
                }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> ValkyrieParser<'config> {
    pub(crate) fn build_root(&self, green_tree: &GreenNode<ValkyrieLanguage>, source: &SourceText) -> Result<ValkyrieRoot, OakError> {
        let red_root = RedNode::<ValkyrieLanguage>::new(green_tree, 0);
        let mut items = Vec::new();
        for child in red_root.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Function => {
                        let func = self.build_function(n, source)?;
                        items.push(Item::Function(func));
                    }
                    ValkyrieSyntaxKind::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(Item::Namespace(ns));
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
                    _ => {
                        return Err(source.syntax_error("Unexpected item in root".to_string(), n.span().start));
                    }
                },
                RedTree::Leaf(t) => {
                    return Err(source.syntax_error("Unexpected token in root".to_string(), t.span.start));
                }
            }
        }
        Ok(ValkyrieRoot { items })
    }

    /// 从红绿树提取强类型 AST

    pub(crate) fn build_namespace(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Namespace, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == ValkyrieSyntaxKind::Identifier {
                        name.name = text(source, t.span.clone().into());
                        name.span = t.span.clone();
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Function => {
                        let func = self.build_function(n, source)?;
                        items.push(Item::Function(func));
                    }
                    ValkyrieSyntaxKind::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(Item::Namespace(ns));
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
                    _ => {
                        return Err(source.syntax_error("Unexpected item in namespace".to_string(), n.span().start));
                    }
                },
            }
        }
        Ok(Namespace { name, items, span })
    }

    pub(crate) fn build_micro(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<MicroDefinition, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == ValkyrieSyntaxKind::Identifier {
                        name.name = text(source, t.span.clone().into());
                        name.span = t.span.clone();
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Function => {
                        let func = self.build_function(n, source)?;
                        items.push(Item::Function(func));
                    }
                    ValkyrieSyntaxKind::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(Item::Namespace(ns));
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
                    _ => {
                        return Err(source.syntax_error("Unexpected item in micro definition".to_string(), n.span().start));
                    }
                },
            }
        }
        Ok(MicroDefinition { name, items, span })
    }

    pub(crate) fn build_function(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Function, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut params = Vec::new();
        let mut return_type = None;
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == ValkyrieSyntaxKind::Identifier {
                        name.name = text(source, t.span.clone().into());
                        name.span = t.span.clone();
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::ParameterList => {
                        params = self.build_params(n, source)?;
                    }
                    ValkyrieSyntaxKind::Type => {
                        return_type = Some(text(source, n.span().into()));
                    }
                    ValkyrieSyntaxKind::BlockExpression => {
                        body = Some(self.build_block(n, source)?);
                    }
                    _ => {
                        return Err(source.syntax_error("Unexpected item in function definition".to_string(), n.span().start));
                    }
                },
            }
        }

        let body = body.ok_or_else(|| source.syntax_error(format!("Missing function body at {:?}", span), span.start))?;

        Ok(Function { name, params, return_type, body, span })
    }

    fn build_params(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Vec<Param>, OakError> {
        let mut params = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == ValkyrieSyntaxKind::Parameter {
                    params.push(self.build_param(n, source)?);
                }
            }
        }
        Ok(params)
    }

    fn build_param(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Param, OakError> {
        let span = node.span();
        let mut name: Option<Identifier> = None;
        let mut ty = None;
        // children: IDENT ':' IDENT
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == ValkyrieSyntaxKind::Identifier {
                        if name.is_none() {
                            name = Some(Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() });
                        }
                        else {
                            ty = Some(text(source, t.span.clone().into()));
                        }
                    }
                    else if t.kind != ValkyrieSyntaxKind::Colon {
                        return Err(source.syntax_error("Unexpected token in parameter definition", t.span.start));
                    }
                }
                _ => {
                    return Err(source.syntax_error("Unexpected token in parameter definition", child.span().start));
                }
            }
        }
        return if let (Some(name), Some(ty)) = (name, ty) { Ok(Param { name, ty, span }) } else { Err(source.syntax_error(format!("Missing name or type in parameter at {:?}", span), span.start)) };
    }

    fn build_block(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Block, OakError> {
        let span = node.span();
        let mut statements = Vec::new();
        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::LetStatement => statements.push(self.build_let(n, source)?),
                    ValkyrieSyntaxKind::ExpressionStatement => statements.push(self.build_expr_stmt(n, source)?),
                    _ => {
                        return Err(source.syntax_error("Unexpected statement in block", n.span().start));
                    }
                },
                RedTree::Leaf(t) => {
                    if t.kind != ValkyrieSyntaxKind::LeftBrace && t.kind != ValkyrieSyntaxKind::RightBrace {
                        return Err(source.syntax_error("Unexpected token in block", t.span.start));
                    }
                }
            }
        }
        Ok(Block { statements, span })
    }

    fn build_expr_stmt(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Statement, OakError> {
        let span = node.span();
        let mut children_iter = node.children().peekable();

        let expr_node = children_iter.next().ok_or_else(|| source.syntax_error("Missing expression in expression statement", span.start))?;

        let expr = match expr_node {
            RedTree::Node(n) => self.build_expr(n, source)?,
            RedTree::Leaf(t) => {
                return Err(source.syntax_error("Expected an expression, found a token", t.span.start));
            }
        };

        let mut semi = false;
        if let Some(RedTree::Leaf(t)) = children_iter.peek() {
            if t.kind == ValkyrieSyntaxKind::Semicolon {
                semi = true;
                children_iter.next(); // Consume the semicolon
            }
        }

        if let Some(unexpected_child) = children_iter.next() {
            return Err(source.syntax_error("Unexpected token or expression after semicolon", unexpected_child.span().start));
        }

        Ok(Statement::ExprStmt { expr, semi, span })
    }

    fn build_let(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Statement, OakError> {
        let span = node.span();
        let mut children_iter = node.children().peekable();

        // Expect 'let' keyword
        let let_keyword = children_iter.next().ok_or_else(|| source.syntax_error("Missing 'let' keyword", span.start))?;
        match let_keyword {
            RedTree::Leaf(t) if t.kind == ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Let) => {}
            _ => {
                return Err(source.syntax_error("Expected 'let' keyword", let_keyword.span().start));
            }
        }

        // Expect identifier
        let name_node = children_iter.next().ok_or_else(|| source.syntax_error("Missing identifier in let statement", span.start))?;
        let name = match name_node {
            RedTree::Leaf(t) if t.kind == ValkyrieSyntaxKind::Identifier => Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() },
            _ => {
                return Err(source.syntax_error("Expected identifier in let statement", name_node.span().start));
            }
        };

        let mut expr: Option<Expr> = None;

        // Check for optional '=' and expression
        if let Some(RedTree::Leaf(t)) = children_iter.peek() {
            if t.kind == ValkyrieSyntaxKind::Eq {
                children_iter.next(); // Consume '=' token

                let expr_node = children_iter.next().ok_or_else(|| source.syntax_error("Missing expression after '=' in let statement", span.end))?;

                expr = Some(match expr_node {
                    RedTree::Node(n) => self.build_expr(n, source)?,
                    RedTree::Leaf(t) => {
                        return Err(source.syntax_error("Expected an expression, found a token after '=' in let statement", t.span.start));
                    }
                });
            }
        }

        if let Some(unexpected_child) = children_iter.next() {
            return Err(source.syntax_error("Unexpected token or expression in let statement", unexpected_child.span().start));
        }

        let expr = expr.ok_or_else(|| source.syntax_error("Missing expression in let statement", span.start))?;

        Ok(Statement::Let { name, expr, span })
    }

    pub(crate) fn build_expr(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Expr, OakError> {
        match node.green.kind {
            ValkyrieSyntaxKind::IdentifierExpression => {
                let span = node.span();
                // child: IDENT
                for child in node.children() {
                    if let RedTree::Leaf(t) = child {
                        if t.kind == ValkyrieSyntaxKind::Identifier {
                            return Ok(Expr::Ident(Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() }));
                        }
                    }
                }
                Err(source.syntax_error(format!("Missing identifier in identifier expression at {:?}", span), span.start))
            }
            ValkyrieSyntaxKind::LiteralExpression => {
                let span = node.span();
                for child in node.children() {
                    if let RedTree::Leaf(t) = child {
                        return Ok(Expr::Literal { value: text(source, t.span.into()), span });
                    }
                }
                Err(source.syntax_error(format!("Missing literal in literal expression at {:?}", span), span.start))
            }
            ValkyrieSyntaxKind::BooleanLiteral => {
                let span = node.span();
                for child in node.children() {
                    if let RedTree::Leaf(t) = child {
                        return Ok(Expr::Bool { value: t.kind == ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::True), span });
                    }
                }
                Err(source.syntax_error(format!("Missing boolean literal in boolean literal expression at {:?}", span), span.start))
            }
            ValkyrieSyntaxKind::ParenthesizedExpression => {
                let span = node.span();
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        return Ok(Expr::Paren { expr: Box::new(self.build_expr(n, source)?), span });
                    }
                }
                Err(source.syntax_error(format!("Missing expression in parenthesized expression at {:?}", span), span.start))
            }
            ValkyrieSyntaxKind::UnaryExpression => {
                let span = node.span();
                // children: operator expression
                let mut op: Option<ValkyrieSyntaxKind> = None;
                let mut expr: Option<Expr> = None;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            expr = Some(self.build_expr(n, source)?);
                        }
                        RedTree::Leaf(t) => {
                            op = Some(t.kind);
                        }
                    }
                }
                if let (Some(op_kind), Some(expr_val)) = (op, expr) { Ok(Expr::Unary { op: op_kind, expr: Box::new(expr_val), span }) } else { Err(source.syntax_error(format!("Missing operand in unary expression at {:?}", span), span.start)) }
            }
            ValkyrieSyntaxKind::BinaryExpression => {
                let span = node.span();
                // children: left operator right
                let mut left: Option<Expr> = None;
                let mut op: Option<ValkyrieSyntaxKind> = None;
                let mut right: Option<Expr> = None;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if left.is_none() {
                                left = Some(self.build_expr(n, source)?);
                            }
                            else {
                                right = Some(self.build_expr(n, source)?);
                            }
                        }
                        RedTree::Leaf(t) => {
                            op = Some(t.kind);
                        }
                    }
                }
                if let (Some(left_expr), Some(op_kind), Some(right_expr)) = (left, op, right) {
                    Ok(Expr::Binary { left: Box::new(left_expr), op: op_kind, right: Box::new(right_expr), span })
                }
                else {
                    Err(source.syntax_error(format!("Missing operands in binary expression at {:?}", span), span.start))
                }
            }
            ValkyrieSyntaxKind::CallExpression => {
                let span = node.span();
                // children: callee '(' args... ')' with commas
                let mut callee: Option<Expr> = None;
                let mut args: Vec<Expr> = Vec::new();
                let mut seen_paren = false;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if !seen_paren && callee.is_none() {
                                callee = Some(self.build_expr(n, source)?);
                            }
                            else {
                                args.push(self.build_expr(n, source)?);
                            }
                        }
                        RedTree::Leaf(t) => {
                            if t.kind == ValkyrieSyntaxKind::LeftParen {
                                seen_paren = true;
                            }
                        }
                    }
                }
                if let Some(callee_expr) = callee { Ok(Expr::Call { callee: Box::new(callee_expr), args, span }) } else { Err(source.syntax_error(format!("Missing callee in call expression at {:?}", span), span.start)) }
            }
            ValkyrieSyntaxKind::FieldExpression => {
                let span = node.span();
                let mut receiver: Option<Expr> = None;
                let mut field: Option<Identifier> = None;
                let mut idx = 0;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if idx == 0 {
                                // The first node is the receiver
                                receiver = Some(self.build_expr(n, source)?);
                            }
                        }
                        RedTree::Leaf(t) => {
                            if idx == 2 && t.kind == ValkyrieSyntaxKind::Identifier {
                                // The third child (leaf) is the identifier
                                field = Some(Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() });
                            }
                        }
                    }
                    idx += 1;
                }
                if let (Some(receiver_val), Some(field_val)) = (receiver, field) {
                    Ok(Expr::Field { receiver: Box::new(receiver_val), field: field_val, span })
                }
                else {
                    Err(source.syntax_error(format!("Missing receiver or field in field expression at {:?}", span), span.start))
                }
            }
            ValkyrieSyntaxKind::IndexExpression => {
                let span = node.span();
                // children: base '[' index ']'
                let mut base: Option<Expr> = None;
                let mut index: Option<Expr> = None;
                let mut idx = 0;
                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if idx == 0 {
                                base = Some(self.build_expr(n, source)?);
                            }
                            else {
                                index = Some(self.build_expr(n, source)?);
                            }
                        }
                        _ => {}
                    }
                    idx += 1;
                }
                if let (Some(base_expr), Some(index_expr)) = (base, index) {
                    Ok(Expr::Index { receiver: Box::new(base_expr), index: Box::new(index_expr), span })
                }
                else {
                    Err(source.syntax_error(format!("Missing base or index in index expression at {:?}", span), span.start))
                }
            }
            ValkyrieSyntaxKind::BlockExpression => Ok(Expr::Block(self.build_block(node, source)?)),
            _ => Err(source.syntax_error(format!("Unknown expression type at {:?}", node.span()), node.span().start)),
        }
    }
}

#[inline]
fn text(source: &SourceText, span: Range<usize>) -> String {
    source.get_text_in(span.into()).to_string()
}
