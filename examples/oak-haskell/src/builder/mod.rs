use crate::{
    ast::*,
    language::HaskellLanguage,
    lexer::token_type::HaskellTokenType,
    parser::{HaskellParser, element_type::HaskellElementType},
};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, RedNode, RedTree, Source, SourceText, TextEdit};

/// AST builder for the Haskell language.
#[derive(Clone)]
pub struct HaskellBuilder<'config> {
    /// Language configuration.
    config: &'config HaskellLanguage,
}

impl<'config> HaskellBuilder<'config> {
    /// Creates a new Haskell builder with the given configuration.
    pub fn new(config: &'config HaskellLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<HaskellLanguage> for HaskellBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<HaskellLanguage>) -> OakDiagnostics<HaskellRoot> {
        let parser = HaskellParser::new(self.config);
        let lexer = crate::lexer::HaskellLexer::new(&self.config);

        let mut cache = oak_core::parser::session::ParseSession::<HaskellLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut cache);

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

impl<'config> HaskellBuilder<'config> {
    /// Builds the root node.
    pub(crate) fn build_root(&self, green_tree: GreenNode<HaskellLanguage>, source: &SourceText) -> Result<HaskellRoot, OakError> {
        let red_root = RedNode::new(&green_tree, 0);
        let mut module_name = None;
        let mut items = Vec::new();

        for child in red_root.children() {
            if let RedTree::Node(node) = child {
                match node.green.kind {
                    HaskellElementType::ModuleDeclaration => {
                        module_name = self.extract_module_name(&node, source);
                    }
                    _ => {
                        if let Some(item) = self.build_item(&node, source) {
                            items.push(item);
                        }
                    }
                }
            }
        }

        Ok(HaskellRoot { module_name, items })
    }

    fn build_item(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<Item> {
        match node.green.kind {
            HaskellElementType::Function => self.build_function(node, source).map(Item::Function),
            HaskellElementType::DataDeclaration => self.build_data(node, source).map(Item::DataDeclaration),
            HaskellElementType::TypeAliasDeclaration => self.build_type_alias(node, source).map(Item::TypeAlias),
            HaskellElementType::ImportDeclaration => self.build_import(node, source).map(Item::Import),
            _ => None,
        }
    }

    fn build_function(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<Function> {
        let mut name = None;
        let mut type_signature = None;
        let mut equations = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    HaskellElementType::Identifier | HaskellElementType::Constructor => {
                        if name.is_none() {
                            name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                    }
                    HaskellElementType::TypeSignature => {
                        type_signature = self.build_type_signature(&n, source);
                    }
                    HaskellElementType::Equation => {
                        if let Some(equation) = self.build_equation(&n, source) {
                            equations.push(equation);
                        }
                    }
                    _ => {}
                }
            }
        }

        name.map(|name| Function { name, type_signature, equations, span: node.span() })
    }

    fn build_type_signature(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<Type> {
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == HaskellElementType::Type {
                    return self.build_type(&n, source);
                }
            }
        }
        None
    }

    fn build_type(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<Type> {
        let mut types = Vec::new();
        let mut is_function = false;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    HaskellElementType::Identifier => {
                        types.push(Type::Variable(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() }));
                    }
                    HaskellElementType::Constructor => {
                        types.push(Type::Constructor(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() }, vec![]));
                    }
                    HaskellElementType::Type => {
                        if let Some(t) = self.build_type(&n, source) {
                            types.push(t);
                        }
                    }
                    _ => {}
                },
                RedTree::Leaf(l) => {
                    if l.kind == HaskellTokenType::Arrow {
                        is_function = true;
                    }
                }
            }
        }

        if is_function && types.len() >= 2 {
            let mut result = types.pop().unwrap();
            while let Some(t) = types.pop() {
                result = Type::Function(Box::new(t), Box::new(result));
            }
            Some(result)
        }
        else if let Some(first) = types.into_iter().next() {
            Some(first)
        }
        else {
            None
        }
    }

    fn build_data(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<DataDeclaration> {
        let mut name = None;
        let mut type_params = Vec::new();
        let mut constructors = Vec::new();

        let mut found_assign = false;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    HaskellElementType::Constructor if !found_assign => {
                        name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                    }
                    HaskellElementType::Identifier if !found_assign => {
                        type_params.push(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                    }
                    HaskellElementType::Constructor if found_assign => {
                        if let Some(cons) = self.build_constructor_def(&n, source) {
                            constructors.push(cons);
                        }
                    }
                    _ => {}
                }
            }
            else if let RedTree::Leaf(l) = child {
                if l.kind == HaskellTokenType::Assign {
                    found_assign = true;
                }
            }
        }

        name.map(|name| DataDeclaration { name, type_params, constructors })
    }

    fn build_constructor_def(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<ConstructorDef> {
        let mut name = None;
        let mut fields = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    HaskellElementType::Constructor if name.is_none() => {
                        name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                    }
                    HaskellElementType::Type => {
                        if let Some(t) = self.build_type(&n, source) {
                            fields.push(t);
                        }
                    }
                    _ => {}
                }
            }
        }

        name.map(|name| ConstructorDef { name, fields })
    }

    fn build_type_alias(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<TypeAlias> {
        let mut name = None;
        let mut type_params = Vec::new();
        let mut target = None;

        let mut found_assign = false;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    HaskellElementType::Identifier if !found_assign => {
                        if name.is_none() {
                            name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                        else {
                            type_params.push(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                    }
                    HaskellElementType::Type if found_assign => {
                        target = self.build_type(&n, source);
                    }
                    _ => {}
                }
            }
            else if let RedTree::Leaf(l) = child {
                if l.kind == HaskellTokenType::Assign {
                    found_assign = true;
                }
            }
        }

        if let (Some(name), Some(target)) = (name, target) { Some(TypeAlias { name, type_params, target }) } else { None }
    }

    fn build_import(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<Import> {
        let mut module = None;
        let mut qualified = false;
        let mut as_name = None;

        let mut found_as = false;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    HaskellElementType::Constructor => {
                        if found_as {
                            as_name = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                        else {
                            module = Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                        }
                    }
                    _ => {}
                }
            }
            else if let RedTree::Leaf(l) = child {
                match l.kind {
                    HaskellTokenType::Qualified => qualified = true,
                    HaskellTokenType::As => found_as = true,
                    _ => {}
                }
            }
        }

        module.map(|module| Import { module, qualified, as_name })
    }

    fn build_equation(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<Equation> {
        let mut patterns = Vec::new();
        let mut body = None;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    HaskellElementType::Pattern => {
                        if let Some(pattern) = self.build_pattern(&n, source) {
                            patterns.push(pattern);
                        }
                    }
                    _ if n.green.kind as u16 >= HaskellElementType::LiteralExpression as u16 && n.green.kind as u16 <= HaskellElementType::CaseExpression as u16 => {
                        body = self.build_expression(&n, source);
                    }
                    _ => {}
                }
            }
        }

        body.map(|body| Equation { patterns, body })
    }

    fn build_expression(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<Expression> {
        match node.green.kind {
            HaskellElementType::IdentifierExpression => {
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        if n.green.kind == HaskellElementType::Identifier {
                            return Some(Expression::Variable(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() }));
                        }
                    }
                }
                None
            }
            HaskellElementType::LiteralExpression => {
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        if let Some(lit) = self.build_literal(&n, source) {
                            return Some(Expression::Literal(lit));
                        }
                    }
                }
                None
            }
            HaskellElementType::ApplicationExpression => {
                let mut exprs = Vec::new();
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        if let Some(expr) = self.build_expression(&n, source) {
                            exprs.push(expr);
                        }
                    }
                }
                if exprs.len() >= 2 {
                    let mut iter = exprs.into_iter();
                    let mut result = iter.next().unwrap();
                    for next in iter {
                        result = Expression::Application(Box::new(result), Box::new(next));
                    }
                    Some(result)
                }
                else {
                    exprs.into_iter().next()
                }
            }
            HaskellElementType::LambdaExpression => {
                let mut patterns = Vec::new();
                let mut body = None;
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        match n.green.kind {
                            HaskellElementType::Pattern => {
                                if let Some(p) = self.build_pattern(&n, source) {
                                    patterns.push(p);
                                }
                            }
                            _ if n.green.kind as u16 >= HaskellElementType::LiteralExpression as u16 && n.green.kind as u16 <= HaskellElementType::CaseExpression as u16 => {
                                body = self.build_expression(&n, source);
                            }
                            _ => {}
                        }
                    }
                }
                body.map(|body| Expression::Lambda(patterns, Box::new(body)))
            }
            HaskellElementType::LetExpression => {
                let mut items = Vec::new();
                let mut body = None;
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        match n.green.kind {
                            _ if n.green.kind as u16 >= HaskellElementType::Function as u16 && n.green.kind as u16 <= HaskellElementType::TypeAliasDeclaration as u16 => {
                                if let Some(item) = self.build_item(&n, source) {
                                    items.push(item);
                                }
                            }
                            _ if n.green.kind as u16 >= HaskellElementType::LiteralExpression as u16 && n.green.kind as u16 <= HaskellElementType::CaseExpression as u16 => {
                                body = self.build_expression(&n, source);
                            }
                            _ => {}
                        }
                    }
                }
                body.map(|body| Expression::Let(items, Box::new(body)))
            }
            HaskellElementType::CaseExpression => {
                let mut target = None;
                let mut arms = Vec::new();
                for child in node.children() {
                    if let RedTree::Node(n) = child {
                        match n.green.kind {
                            HaskellElementType::CaseArm => {
                                if let Some(arm) = self.build_case_arm(&n, source) {
                                    arms.push(arm);
                                }
                            }
                            _ if n.green.kind as u16 >= HaskellElementType::LiteralExpression as u16 && n.green.kind as u16 <= HaskellElementType::CaseExpression as u16 => {
                                if target.is_none() {
                                    target = self.build_expression(&n, source);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                target.map(|target| Expression::Case(Box::new(target), arms))
            }
            _ => None,
        }
    }

    fn build_case_arm(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<CaseArm> {
        let mut pattern = None;
        let mut body = None;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    HaskellElementType::Pattern => {
                        pattern = self.build_pattern(&n, source);
                    }
                    _ if n.green.kind as u16 >= HaskellElementType::LiteralExpression as u16 && n.green.kind as u16 <= HaskellElementType::CaseExpression as u16 => {
                        body = self.build_expression(&n, source);
                    }
                    _ => {}
                }
            }
        }

        if let (Some(pattern), Some(body)) = (pattern, body) { Some(CaseArm { pattern, body }) } else { None }
    }

    fn build_literal(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<Literal> {
        for child in node.children() {
            if let RedTree::Leaf(l) = child {
                let text = source.get_text_in(l.span());
                match l.kind {
                    HaskellTokenType::Integer => {
                        if let Ok(val) = text.parse::<i64>() {
                            return Some(Literal::Integer(val));
                        }
                    }
                    HaskellTokenType::Float => {
                        if let Ok(val) = text.parse::<f64>() {
                            return Some(Literal::Float(val));
                        }
                    }
                    HaskellTokenType::StringLiteral => {
                        return Some(Literal::String(text[1..text.len() - 1].to_string()));
                    }
                    HaskellTokenType::CharLiteral => {
                        if let Some(c) = text.chars().nth(1) {
                            return Some(Literal::Char(c));
                        }
                    }
                    _ => {}
                }
            }
        }
        None
    }

    fn build_pattern(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<Pattern> {
        for child in node.children() {
            match child {
                RedTree::Leaf(leaf) => {
                    if leaf.kind == HaskellTokenType::Underscore {
                        return Some(Pattern::Wildcard);
                    }
                }
                RedTree::Node(n) => match n.green.kind {
                    HaskellElementType::Identifier => {
                        return Some(Pattern::Variable(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() }));
                    }
                    HaskellElementType::Constructor => {
                        let mut name = None;
                        let mut args = Vec::new();
                        for c in n.children() {
                            if let RedTree::Node(cn) = c {
                                if cn.green.kind == HaskellElementType::Constructor && name.is_none() {
                                    name = Some(Identifier { name: source.get_text_in(cn.span()).to_string(), span: cn.span() });
                                }
                                else if cn.green.kind == HaskellElementType::Pattern {
                                    if let Some(p) = self.build_pattern(&cn, source) {
                                        args.push(p);
                                    }
                                }
                            }
                        }
                        if let Some(name) = name {
                            return Some(Pattern::Constructor(name, args));
                        }
                    }
                    _ if n.green.kind == HaskellElementType::LiteralExpression => {
                        if let Some(expr) = self.build_expression(&n, source) {
                            if let Expression::Literal(lit) = expr {
                                return Some(Pattern::Literal(lit));
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        None
    }

    fn extract_module_name(&self, node: &RedNode<HaskellLanguage>, source: &SourceText) -> Option<Identifier> {
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == HaskellElementType::Constructor {
                    return Some(Identifier { name: source.get_text_in(n.span()).to_string(), span: n.span() });
                }
            }
        }
        None
    }
}
