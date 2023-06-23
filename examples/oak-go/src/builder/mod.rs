//! Go 语言构建器

use crate::{
    ast::{self, Declaration, GoRoot},
    language::GoLanguage,
    lexer::{GoLexer, token_type::GoTokenType},
    parser::{GoParser, element_type::GoElementType},
};
use oak_core::{
    Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, ParseSession, Parser, SourceText, TextEdit, TokenType,
    builder::BuildOutput,
    source::Source,
    tree::{RedNode, RedTree},
};

/// Go 语言构建器
pub struct GoBuilder<'config> {
    pub(crate) config: &'config GoLanguage,
}

impl<'config> GoBuilder<'config> {
    pub fn new(config: &'config GoLanguage) -> Self {
        Self { config }
    }

    fn build_root<'a>(&self, green: &'a GreenNode<'a, GoLanguage>, source: &SourceText) -> Result<GoRoot, OakError> {
        let red = RedNode::new(green, 0);
        let mut package = None;
        let mut imports = vec![];
        let mut declarations = vec![];

        for child in red.children() {
            if let RedTree::Node(node) = child {
                match node.green.kind {
                    GoElementType::PackageClause => {
                        package = self.extract_package(node, source);
                    }
                    GoElementType::ImportDeclaration => {
                        imports.extend(self.extract_imports(node, source));
                    }
                    GoElementType::FunctionDeclaration => {
                        declarations.push(Declaration::Function(self.extract_function(node, source)?));
                    }
                    GoElementType::VariableDeclaration => {
                        declarations.extend(self.extract_variables(node, source)?);
                    }
                    GoElementType::ConstDeclaration => {
                        declarations.extend(self.extract_consts(node, source)?);
                    }
                    GoElementType::TypeDeclaration => {
                        declarations.extend(self.extract_types(node, source)?);
                    }
                    _ => {}
                }
            }
        }

        Ok(GoRoot { package, imports, declarations })
    }

    fn extract_package(&self, node: RedNode<GoLanguage>, source: &SourceText) -> Option<String> {
        for child in node.children() {
            if let RedTree::Leaf(leaf) = child {
                if leaf.kind == GoTokenType::Identifier {
                    return Some(source.get_text_in(leaf.span).trim().to_string());
                }
            }
        }
        None
    }

    fn extract_imports(&self, node: RedNode<GoLanguage>, source: &SourceText) -> Vec<ast::Import> {
        let mut imports = vec![];
        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    GoElementType::ImportSpec => {
                        let mut path = String::new();
                        let mut alias = None;
                        for spec_child in n.children() {
                            match spec_child {
                                RedTree::Leaf(leaf) if leaf.kind == GoTokenType::Identifier => {
                                    alias = Some(source.get_text_in(leaf.span).trim().to_string());
                                }
                                RedTree::Leaf(leaf) if leaf.kind == GoTokenType::StringLiteral => {
                                    path = source.get_text_in(leaf.span).trim_matches('"').to_string();
                                }
                                _ => {}
                            }
                        }
                        imports.push(ast::Import { path, alias, span: n.span() });
                    }
                    _ => {
                        // 处理 import ( ... )
                        imports.extend(self.extract_imports(n, source));
                    }
                }
            }
        }
        imports
    }

    fn extract_function(&self, node: RedNode<GoLanguage>, source: &SourceText) -> Result<ast::Function, OakError> {
        let mut name = String::new();
        let mut params = vec![];
        let mut return_types = vec![];
        let mut body = ast::Block { statements: vec![], span: (0..0).into() };
        let span = node.span();

        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) if leaf.kind == GoTokenType::Identifier => {
                    name = source.get_text_in(leaf.span).trim().to_string();
                }
                RedTree::Node(n) => match n.green.kind {
                    GoElementType::ParameterList => {
                        for p_child in n.children() {
                            if let RedTree::Node(pn) = p_child {
                                if pn.green.kind == GoElementType::ParameterDecl {
                                    let mut p_name = String::new();
                                    let mut p_type = String::new();
                                    for pd_child in pn.children() {
                                        match pd_child {
                                            RedTree::Leaf(leaf) if leaf.kind == GoTokenType::Identifier => {
                                                if p_name.is_empty() {
                                                    p_name = source.get_text_in(leaf.span).trim().to_string();
                                                }
                                                else {
                                                    p_type = source.get_text_in(leaf.span).trim().to_string();
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                    params.push(ast::Parameter { name: p_name, param_type: p_type, span: pn.span() });
                                }
                            }
                        }
                    }
                    GoElementType::Block => {
                        body = self.extract_block(n, source)?;
                    }
                    _ if n.green.kind.is_keyword() || n.green.kind == GoElementType::Identifier => {
                        return_types.push(source.get_text_in(n.span()).to_string());
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(ast::Function { name, params, return_types, body, span })
    }

    fn extract_block(&self, node: RedNode<GoLanguage>, source: &SourceText) -> Result<ast::Block, OakError> {
        let mut statements = vec![];
        let span = node.span();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                if let Some(stmt) = self.extract_statement(n, source)? {
                    statements.push(stmt);
                }
            }
        }

        Ok(ast::Block { statements, span })
    }

    fn extract_statement(&self, node: RedNode<GoLanguage>, source: &SourceText) -> Result<Option<ast::Statement>, OakError> {
        match node.green.kind {
            GoElementType::ReturnStatement => {
                let mut values = vec![];
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        values.push(self.extract_expression(n, source)?);
                    }
                }
                Ok(Some(ast::Statement::Return { values, span: node.span() }))
            }
            GoElementType::IfStatement => {
                let mut condition = ast::Expression::Literal { value: "true".to_string(), span: node.span() };
                let mut then_block = ast::Block { statements: vec![], span: node.span() };
                let mut else_block = None;

                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        match n.green.kind {
                            GoElementType::BinaryExpression | GoElementType::CallExpression | GoElementType::Identifier => {
                                condition = self.extract_expression(n, source)?;
                            }
                            GoElementType::Block => {
                                if then_block.statements.is_empty() {
                                    then_block = self.extract_block(n, source)?;
                                }
                                else {
                                    else_block = Some(self.extract_block(n, source)?);
                                }
                            }
                            GoElementType::IfStatement => {
                                // 处理 else if
                                let inner_if = self.extract_statement(n, source)?;
                                if let Some(ast::Statement::If { condition, then_block, else_block: inner_else, span }) = inner_if {
                                    else_block = Some(ast::Block { statements: vec![ast::Statement::If { condition, then_block, else_block: inner_else, span }], span });
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Some(ast::Statement::If { condition, then_block, else_block, span: node.span() }))
            }
            GoElementType::ForStatement => {
                let mut init = None;
                let mut condition = None;
                let mut post = None;
                let mut body = ast::Block { statements: vec![], span: node.span() };

                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        match n.green.kind {
                            GoElementType::ShortVarDecl | GoElementType::AssignmentStatement | GoElementType::VariableDeclaration => {
                                if init.is_none() {
                                    init = self.extract_statement(n, source)?.map(Box::new);
                                }
                                else {
                                    post = self.extract_statement(n, source)?.map(Box::new);
                                }
                            }
                            GoElementType::BinaryExpression | GoElementType::CallExpression | GoElementType::Identifier => {
                                condition = Some(self.extract_expression(n, source)?);
                            }
                            GoElementType::Block => {
                                body = self.extract_block(n, source)?;
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Some(ast::Statement::For { init, condition, post, body, span: node.span() }))
            }
            GoElementType::AssignmentStatement | GoElementType::ShortVarDecl | GoElementType::VariableDeclaration | GoElementType::VariableSpec => {
                // 支持多重赋值
                let mut targets = vec![];
                let mut values = vec![];

                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) if leaf.kind == GoTokenType::Identifier => {
                            targets.push(source.get_text_in(leaf.span).trim().to_string());
                        }
                        RedTree::Node(n) => {
                            if n.green.kind == GoElementType::VariableSpec || n.green.kind == GoElementType::VariableDeclaration {
                                // 递归提取
                                if let Some(ast::Statement::Assignment { targets: t, values: v, .. }) = self.extract_statement(n, source)? {
                                    targets.extend(t);
                                    values.extend(v);
                                }
                            }
                            else if n.green.kind != GoElementType::Identifier && !n.green.kind.is_keyword() {
                                values.push(self.extract_expression(n, source)?);
                            }
                            else if n.green.kind == GoElementType::Identifier {
                                targets.push(source.get_text_in(n.span()).trim().to_string());
                            }
                        }
                        _ => {}
                    }
                }

                if targets.is_empty() {
                    return Ok(None);
                }

                // 如果没有值（如变量声明），填充默认值
                if values.is_empty() {
                    for _ in &targets {
                        values.push(ast::Expression::Literal { value: "0".to_string(), span: node.span() });
                    }
                }

                Ok(Some(ast::Statement::Assignment { targets, values, span: node.span() }))
            }
            _ => {
                // 可能是表达式语句
                if let Ok(expr) = self.extract_expression(node, source) { Ok(Some(ast::Statement::Expression(expr))) } else { Ok(None) }
            }
        }
    }

    fn extract_expression(&self, node: RedNode<GoLanguage>, source: &SourceText) -> Result<ast::Expression, OakError> {
        self.extract_expression_internal(RedTree::Node(node), source)
    }

    fn extract_expression_internal(&self, tree: RedTree<GoLanguage>, source: &SourceText) -> Result<ast::Expression, OakError> {
        match tree {
            RedTree::Leaf(leaf) => {
                if leaf.kind == GoTokenType::Identifier {
                    let name = source.get_text_in(leaf.span).trim().to_string();
                    if name.is_empty() {
                        return Err(OakError::parse_error("Empty identifier leaf"));
                    }
                    Ok(ast::Expression::Identifier { name, span: leaf.span })
                }
                else if leaf.kind == GoTokenType::IntLiteral || leaf.kind == GoTokenType::StringLiteral || leaf.kind == GoTokenType::BoolLiteral {
                    Ok(ast::Expression::Literal { value: source.get_text_in(leaf.span).to_string(), span: leaf.span })
                }
                else {
                    Err(OakError::parse_error(format!("Unexpected leaf in expression: {:?}", leaf.kind)))
                }
            }
            RedTree::Node(node) => match node.green.kind {
                GoElementType::Identifier => {
                    let mut name = String::new();
                    for child in node.children() {
                        match child {
                            RedTree::Leaf(leaf) if leaf.kind == GoTokenType::Identifier => {
                                name = source.get_text_in(leaf.span).trim().to_string();
                                if !name.is_empty() {
                                    break;
                                }
                            }
                            RedTree::Node(n) => {
                                println!("DEBUG: Identifier node has a Node child: kind={:?}, span={:?}", n.green.kind, n.span());
                            }
                            _ => {}
                        }
                    }
                    if name.is_empty() {
                        name = source.get_text_in(node.span()).trim().to_string();
                    }
                    if name.is_empty() {
                        println!("DEBUG: Final empty identifier node details:");
                        println!("  Node kind: {:?}", node.green.kind);
                        println!("  Node span: {:?}", node.span());
                        println!("  Children count: {}", node.children().count());
                        for (i, child) in node.children().enumerate() {
                            match child {
                                RedTree::Node(n) => println!("    child {}: Node kind={:?}, span={:?}", i, n.green.kind, n.span()),
                                RedTree::Leaf(l) => println!("    child {}: Leaf kind={:?}, span={:?}, text={:?}", i, l.kind, l.span, source.get_text_in(l.span)),
                            }
                        }
                        return Err(OakError::parse_error(format!("Empty identifier at {:?}", node.span())));
                    }
                    Ok(ast::Expression::Identifier { name, span: node.span() })
                }
                GoElementType::IntLiteral | GoElementType::StringLiteral | GoElementType::BoolLiteral => Ok(ast::Expression::Literal { value: source.get_text_in(node.span()).trim().to_string(), span: node.span() }),
                GoElementType::BinaryExpression => {
                    let mut left = None;
                    let mut op = String::new();
                    let mut right = None;

                    for child in node.children() {
                        match child {
                            RedTree::Node(n) => {
                                if left.is_none() {
                                    left = Some(Box::new(self.extract_expression(n, source)?));
                                }
                                else {
                                    right = Some(Box::new(self.extract_expression(n, source)?));
                                }
                            }
                            RedTree::Leaf(leaf) => {
                                if leaf.kind == GoTokenType::Identifier {
                                    if left.is_none() {
                                        left = Some(Box::new(ast::Expression::Identifier { name: source.get_text_in(leaf.span).trim().to_string(), span: leaf.span }));
                                    }
                                    else {
                                        right = Some(Box::new(ast::Expression::Identifier { name: source.get_text_in(leaf.span).trim().to_string(), span: leaf.span }));
                                    }
                                }
                                else if TokenType::role(&leaf.kind) == oak_core::UniversalTokenRole::Operator {
                                    op = source.get_text_in(leaf.span).to_string();
                                }
                            }
                        }
                    }

                    if let (Some(left), Some(right)) = (left, right) { Ok(ast::Expression::Binary { left, op, right, span: node.span() }) } else { Ok(ast::Expression::Literal { value: source.get_text_in(node.span()).to_string(), span: node.span() }) }
                }
                GoElementType::CallExpression => {
                    let mut func = None;
                    let mut args = vec![];

                    for child in node.children() {
                        match child {
                            RedTree::Node(n) => {
                                if func.is_none() {
                                    func = Some(Box::new(self.extract_expression(n, source)?));
                                }
                                else if n.green.kind == GoElementType::ExpressionList {
                                    for list_child in n.children() {
                                        if let RedTree::Node(ln) = list_child {
                                            args.push(self.extract_expression(ln, source)?);
                                        }
                                        else if let RedTree::Leaf(leaf) = list_child {
                                            if let Ok(expr) = self.extract_expression_internal(RedTree::Leaf(leaf), source) {
                                                args.push(expr);
                                            }
                                        }
                                    }
                                }
                                else {
                                    args.push(self.extract_expression(n, source)?);
                                }
                            }
                            RedTree::Leaf(leaf) if leaf.kind == GoTokenType::Identifier => {
                                if func.is_none() {
                                    func = Some(Box::new(ast::Expression::Identifier { name: source.get_text_in(leaf.span).trim().to_string(), span: leaf.span }));
                                }
                                else {
                                    args.push(ast::Expression::Identifier { name: source.get_text_in(leaf.span).trim().to_string(), span: leaf.span });
                                }
                            }
                            _ => {}
                        }
                    }

                    if let Some(func) = func { Ok(ast::Expression::Call { func, args, span: node.span() }) } else { Ok(ast::Expression::Literal { value: source.get_text_in(node.span()).to_string(), span: node.span() }) }
                }
                _ => Ok(ast::Expression::Literal { value: source.get_text_in(node.span()).to_string(), span: node.span() }),
            },
        }
    }

    fn extract_variables(&self, node: RedNode<GoLanguage>, source: &SourceText) -> Result<Vec<Declaration>, OakError> {
        let mut vars = vec![];
        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    GoElementType::VariableSpec => {
                        let mut name = String::new();
                        let mut var_type = None;
                        let mut value = None;
                        for spec_child in n.children() {
                            match spec_child {
                                RedTree::Leaf(leaf) if leaf.kind == GoTokenType::Identifier => {
                                    if name.is_empty() {
                                        name = source.get_text_in(leaf.span).trim().to_string();
                                    }
                                    else if var_type.is_none() {
                                        var_type = Some(source.get_text_in(leaf.span).trim().to_string());
                                    }
                                }
                                RedTree::Node(en) => {
                                    value = Some(self.extract_expression(en, source)?);
                                }
                                _ => {}
                            }
                        }
                        vars.push(Declaration::Variable(ast::Variable { name, var_type, value, span: n.span() }));
                    }
                    _ => {
                        // 处理 var ( ... )
                        vars.extend(self.extract_variables(n, source)?);
                    }
                }
            }
        }
        Ok(vars)
    }

    fn extract_consts(&self, node: RedNode<GoLanguage>, source: &SourceText) -> Result<Vec<Declaration>, OakError> {
        let mut consts = vec![];
        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    GoElementType::ConstSpec => {
                        let mut name = String::new();
                        let mut const_type = None;
                        let mut value = ast::Expression::Literal { value: "0".to_string(), span: n.span() };
                        for spec_child in n.children() {
                            match spec_child {
                                RedTree::Leaf(leaf) if leaf.kind == GoTokenType::Identifier => {
                                    if name.is_empty() {
                                        name = source.get_text_in(leaf.span).trim().to_string();
                                    }
                                    else if const_type.is_none() {
                                        const_type = Some(source.get_text_in(leaf.span).trim().to_string());
                                    }
                                }
                                RedTree::Node(en) => {
                                    value = self.extract_expression(en, source)?;
                                }
                                _ => {}
                            }
                        }
                        consts.push(Declaration::Const(ast::Const { name, const_type, value, span: n.span() }));
                    }
                    _ => {
                        // 处理 const ( ... )
                        consts.extend(self.extract_consts(n, source)?);
                    }
                }
            }
        }
        Ok(consts)
    }

    fn extract_types(&self, node: RedNode<GoLanguage>, source: &SourceText) -> Result<Vec<Declaration>, OakError> {
        let mut types = vec![];
        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    GoElementType::TypeSpec => {
                        let mut name = String::new();
                        let mut definition = String::new();
                        for spec_child in n.children() {
                            match spec_child {
                                RedTree::Leaf(leaf) if leaf.kind == GoTokenType::Identifier => {
                                    if name.is_empty() {
                                        name = source.get_text_in(leaf.span).trim().to_string();
                                    }
                                    else {
                                        definition = source.get_text_in(leaf.span).trim().to_string();
                                    }
                                }
                                RedTree::Node(tn) => {
                                    definition = source.get_text_in(tn.span()).to_string();
                                }
                                _ => {}
                            }
                        }
                        types.push(Declaration::Type(ast::TypeDecl { name, definition, span: n.span() }));
                    }
                    _ => {
                        // 处理 type ( ... )
                        types.extend(self.extract_types(n, source)?);
                    }
                }
            }
        }
        Ok(types)
    }
}

impl<'config> Builder<GoLanguage> for GoBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<GoLanguage>) -> BuildOutput<GoLanguage> {
        let parser = GoParser::new(self.config);
        let lexer = GoLexer::new(self.config);

        let mut session = ParseSession::<GoLanguage>::default();
        lexer.lex(source, edits, &mut session);
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
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
