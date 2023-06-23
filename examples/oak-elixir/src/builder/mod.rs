use crate::{
    ast::*,
    language::ElixirLanguage,
    lexer::token_type::ElixirTokenType,
    parser::{ElixirParser, element_type::ElixirElementType},
};
use core::range::Range;
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// Elixir 语言的 AST 构建器
#[derive(Clone, Copy)]
pub struct ElixirBuilder<'config> {
    /// 语言配置
    config: &'config ElixirLanguage,
}

impl<'config> ElixirBuilder<'config> {
    /// 创建新的 Elixir 构建器
    pub fn new(config: &'config ElixirLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<ElixirLanguage> for ElixirBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<ElixirLanguage>) -> BuildOutput<ElixirLanguage> {
        let parser = ElixirParser::new(self.config);

        let mut cache = oak_core::parser::session::ParseSession::<ElixirLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
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

impl<'config> ElixirBuilder<'config> {
    /// 构建根节点
    pub(crate) fn build_root(&self, green_tree: GreenNode<ElixirLanguage>, source: &SourceText) -> Result<ElixirRoot, OakError> {
        let red_root = RedNode::new(&green_tree, 0);
        let mut items = Vec::new();

        for child in red_root.children() {
            match child {
                RedTree::Node(n) => {
                    if let Some(item) = self.build_item(n, source)? {
                        items.push(item);
                    }
                }
                RedTree::Leaf(_) => {}
            }
        }
        Ok(ElixirRoot { items })
    }

    fn build_item(&self, node: RedNode<ElixirLanguage>, source: &SourceText) -> Result<Option<Item>, OakError> {
        match node.green.kind {
            ElixirElementType::ModuleDefinition => {
                let module = self.build_module(node, source)?;
                Ok(Some(Item::Module(module)))
            }
            ElixirElementType::FunctionDefinition => {
                let func = self.build_function(node, source)?;
                Ok(Some(Item::Function(func)))
            }
            _ => {
                if let Some(expr) = self.build_expr_opt(node, source)? {
                    Ok(Some(Item::Statement(Statement::ExprStmt { span: expr_span(&expr), expr })))
                }
                else {
                    Ok(None)
                }
            }
        }
    }

    fn build_module(&self, node: RedNode<ElixirLanguage>, source: &SourceText) -> Result<Module, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: (0..0).into() };
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    ElixirElementType::IdentifierExpression => {
                        name = self.build_identifier(n, source)?;
                    }
                    _ => {
                        if let Some(item) = self.build_item(n, source)? {
                            items.push(item);
                        }
                    }
                },
                RedTree::Leaf(t) => {
                    if t.kind == ElixirTokenType::Identifier {
                        name = Identifier { name: text(source, t.span.clone().into()), span: t.span.clone().into() };
                    }
                }
            }
        }

        Ok(Module { name, items, span: span.into() })
    }

    fn build_function(&self, node: RedNode<ElixirLanguage>, source: &SourceText) -> Result<Function, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: (0..0).into() };
        let mut params = Vec::new();
        let mut body = Block { statements: Vec::new(), span: (0..0).into() };

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    ElixirElementType::IdentifierExpression => {
                        name = self.build_identifier(n, source)?;
                    }
                    ElixirElementType::CallExpression => {
                        // In Elixir, function name and params are often parsed as a call expression
                        // e.g., def hello(name) do ... end
                        for grand_child in n.children() {
                            match grand_child {
                                RedTree::Node(gn) => {
                                    if let Some(expr) = self.build_expr_opt(gn, source)? {
                                        let span = expr_span(&expr);
                                        match expr {
                                            Expr::Ident(id) if name.name.is_empty() => name = id,
                                            _ => params.push(Param {
                                                name: match expr {
                                                    Expr::Ident(id) => id,
                                                    _ => Identifier { name: "_".to_string(), span: (0..0).into() }, // simplified
                                                },
                                                ty: None,
                                                span,
                                            }),
                                        }
                                    }
                                }
                                RedTree::Leaf(gt) if gt.kind == ElixirTokenType::Identifier => {
                                    if name.name.is_empty() {
                                        name = Identifier { name: text(source, gt.span.clone().into()), span: gt.span.clone().into() };
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {
                        if let Some(expr) = self.build_expr_opt(n, source)? {
                            body.statements.push(Statement::ExprStmt { span: expr_span(&expr), expr });
                        }
                    }
                },
                RedTree::Leaf(t) if t.kind == ElixirTokenType::Identifier => {
                    if name.name.is_empty() {
                        name = Identifier { name: text(source, t.span.clone().into()), span: t.span.clone().into() };
                    }
                }
                _ => {}
            }
        }

        body.span = span.into();
        Ok(Function { name, params, body, span: span.into() })
    }

    fn build_expr_opt(&self, node: RedNode<ElixirLanguage>, source: &SourceText) -> Result<Option<Expr>, OakError> {
        match node.green.kind {
            ElixirElementType::IdentifierExpression => Ok(Some(Expr::Ident(self.build_identifier(node, source)?))),
            ElixirElementType::LiteralExpression => {
                for child in node.children() {
                    if let RedTree::Leaf(t) = child {
                        match t.kind {
                            ElixirTokenType::Number | ElixirTokenType::Float => {
                                return Ok(Some(Expr::Number { value: text(source, t.span.clone().into()), span: t.span.clone().into() }));
                            }
                            ElixirTokenType::String => {
                                return Ok(Some(Expr::String { value: text(source, t.span.clone().into()), span: t.span.clone().into() }));
                            }
                            ElixirTokenType::Atom => {
                                return Ok(Some(Expr::Atom { value: text(source, t.span.clone().into()), span: t.span.clone().into() }));
                            }
                            ElixirTokenType::True => {
                                return Ok(Some(Expr::Bool { value: true, span: t.span.clone().into() }));
                            }
                            ElixirTokenType::False => {
                                return Ok(Some(Expr::Bool { value: false, span: t.span.clone().into() }));
                            }
                            _ => {}
                        }
                    }
                }
                Ok(None)
            }
            ElixirElementType::BinaryExpression => {
                let mut left = None;
                let mut op = None;
                let mut right = None;
                let span = node.span();

                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            let expr = self.build_expr_opt(n, source)?;
                            if left.is_none() {
                                left = expr;
                            }
                            else {
                                right = expr;
                            }
                        }
                        RedTree::Leaf(t) => {
                            op = Some(t.kind);
                        }
                    }
                }

                if let (Some(l), Some(o), Some(r)) = (left, op, right) { Ok(Some(Expr::Binary { left: Box::new(l), op: o, right: Box::new(r), span: span.into() })) } else { Ok(None) }
            }
            ElixirElementType::MatchExpression => {
                let mut left = None;
                let mut right = None;
                let span = node.span();

                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        let expr = self.build_expr_opt(n, source)?;
                        if left.is_none() {
                            left = expr;
                        }
                        else {
                            right = expr;
                        }
                    }
                }

                if let (Some(l), Some(r)) = (left, right) { Ok(Some(Expr::Match { left: Box::new(l), right: Box::new(r), span: span.into() })) } else { Ok(None) }
            }
            ElixirElementType::UnaryExpression => {
                let mut op = None;
                let mut expr = None;
                let span = node.span();

                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            expr = self.build_expr_opt(n, source)?;
                        }
                        RedTree::Leaf(t) => {
                            op = Some(t.kind);
                        }
                    }
                }

                if let (Some(o), Some(e)) = (op, expr) {
                    if o == ElixirTokenType::At {
                        if let Expr::Ident(id) = e {
                            return Ok(Some(Expr::Attribute { name: id, span: span.into() }));
                        }
                    }
                    Ok(Some(Expr::Unary { op: o, expr: Box::new(e), span: span.into() }))
                }
                else {
                    Ok(None)
                }
            }
            ElixirElementType::CallExpression => {
                let mut callee = None;
                let mut args = Vec::new();
                let span = node.span();

                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if let Some(expr) = self.build_expr_opt(n, source)? {
                                if callee.is_none() {
                                    callee = Some(Box::new(expr));
                                }
                                else {
                                    args.push(expr);
                                }
                            }
                        }
                        RedTree::Leaf(t) if t.kind == ElixirTokenType::Identifier => {
                            if callee.is_none() {
                                callee = Some(Box::new(Expr::Ident(Identifier { name: text(source, t.span.clone().into()), span: t.span.clone().into() })));
                            }
                        }
                        _ => {}
                    }
                }

                if let Some(c) = callee { Ok(Some(Expr::Call { callee: c, args, span: span.into() })) } else { Ok(None) }
            }
            ElixirElementType::AccessExpression => {
                let mut receiver = None;
                let mut field = None;
                let span = node.span();

                for child in node.children() {
                    match child {
                        RedTree::Node(n) => {
                            if let Some(expr) = self.build_expr_opt(n, source)? {
                                if receiver.is_none() {
                                    receiver = Some(Box::new(expr));
                                }
                                else if let Expr::Ident(id) = expr {
                                    field = Some(id);
                                }
                            }
                        }
                        RedTree::Leaf(t) if t.kind == ElixirTokenType::Identifier => {
                            if receiver.is_none() {
                                receiver = Some(Box::new(Expr::Ident(Identifier { name: text(source, t.span.clone().into()), span: t.span.clone().into() })));
                            }
                            else {
                                field = Some(Identifier { name: text(source, t.span.clone().into()), span: t.span.clone().into() });
                            }
                        }
                        _ => {}
                    }
                }

                if let (Some(r), Some(f)) = (receiver, field) { Ok(Some(Expr::Field { receiver: r, field: f, span: span.into() })) } else { Ok(None) }
            }
            ElixirElementType::BlockExpression => {
                let mut statements = Vec::new();
                let span = node.span();

                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        if let Some(expr) = self.build_expr_opt(n, source)? {
                            statements.push(Statement::ExprStmt { span: expr_span(&expr), expr });
                        }
                    }
                }
                Ok(Some(Expr::Block(Block { statements, span: span.into() })))
            }
            ElixirElementType::ListLiteral => {
                let mut items = Vec::new();
                let span = node.span();
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        if let Some(expr) = self.build_expr_opt(n, source)? {
                            items.push(expr);
                        }
                    }
                }
                Ok(Some(Expr::List { items, span: span.into() }))
            }
            ElixirElementType::TupleLiteral => {
                let mut items = Vec::new();
                let span = node.span();
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        if let Some(expr) = self.build_expr_opt(n, source)? {
                            items.push(expr);
                        }
                    }
                }
                Ok(Some(Expr::Tuple { items, span: span.into() }))
            }
            _ => Ok(None),
        }
    }

    fn build_identifier(&self, node: RedNode<ElixirLanguage>, source: &SourceText) -> Result<Identifier, OakError> {
        let span = node.span();
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == ElixirTokenType::Identifier {
                    return Ok(Identifier { name: text(source, t.span.clone().into()), span: t.span.clone().into() });
                }
            }
        }
        Ok(Identifier { name: String::new(), span: span.into() })
    }
}

fn text(source: &SourceText, span: Range<usize>) -> String {
    source.get_text_in(span).into_owned()
}

fn expr_span(expr: &Expr) -> Range<usize> {
    match expr {
        Expr::Ident(id) => id.span.clone(),
        Expr::Atom { span, .. } => span.clone(),
        Expr::Number { span, .. } => span.clone(),
        Expr::String { span, .. } => span.clone(),
        Expr::Bool { span, .. } => span.clone(),
        Expr::Unary { span, .. } => span.clone(),
        Expr::Binary { span, .. } => span.clone(),
        Expr::Match { span, .. } => span.clone(),
        Expr::Call { span, .. } => span.clone(),
        Expr::Field { span, .. } => span.clone(),
        Expr::Attribute { span, .. } => span.clone(),
        Expr::Index { span, .. } => span.clone(),
        Expr::Paren { span, .. } => span.clone(),
        Expr::List { span, .. } => span.clone(),
        Expr::Tuple { span, .. } => span.clone(),
        Expr::Map { span, .. } => span.clone(),
        Expr::Block(b) => b.span.clone(),
    }
}
