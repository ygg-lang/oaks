use crate::{ast::KotlinRoot, language::KotlinLanguage, lexer::KotlinLexer, parser::KotlinParser};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, SourceText, TextEdit, source::Source};

pub struct KotlinBuilder<'config> {
    config: &'config KotlinLanguage,
}

impl<'config> KotlinBuilder<'config> {
    pub fn new(config: &'config KotlinLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<KotlinLanguage> for KotlinBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<KotlinLanguage>) -> OakDiagnostics<KotlinRoot> {
        let parser = KotlinParser::new(self.config);
        let lexer = KotlinLexer::new(&self.config);

        let mut session = oak_core::parser::session::ParseSession::<KotlinLanguage>::default();
        lexer.lex(source, edits, &mut session);
        let parse_result = parser.parse(source, edits, &mut session);

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

impl<'config> KotlinBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: GreenNode<KotlinLanguage>, _source: &SourceText) -> Result<KotlinRoot, OakError> {
        let mut declarations = Vec::new();
        let mut offset = 0;

        for child in green_tree.children() {
            if let oak_core::GreenTree::Node(node) = child {
                if let Some(decl) = self.build_declaration(node, _source, offset) {
                    declarations.push(decl)
                }
            }
            offset += child.len() as usize
        }

        Ok(KotlinRoot { span: (0.._source.length()).into(), declarations })
    }

    fn build_declaration(&self, node: &oak_core::GreenNode<KotlinLanguage>, source: &SourceText, offset: usize) -> Option<crate::ast::Declaration> {
        use crate::{lexer::token_type::KotlinTokenType, parser::element_type::KotlinElementType};
        let kind: KotlinElementType = node.kind;
        let start = offset;

        match kind {
            KotlinElementType::ClassDeclaration => {
                let mut name = "MyClass".to_string();
                let mut members = vec![];
                let mut inner_offset = offset;
                let mut found_name = false;

                for child in node.children() {
                    match child {
                        oak_core::GreenTree::Node(child_node) => {
                            let child_kind: KotlinElementType = child_node.kind;
                            if child_kind == KotlinElementType::Block {
                                let mut block_offset = inner_offset;
                                for member in child_node.children() {
                                    if let oak_core::GreenTree::Node(m_node) = member {
                                        if let Some(m_decl) = self.build_declaration(m_node, source, block_offset) {
                                            members.push(m_decl)
                                        }
                                    }
                                    block_offset += member.len() as usize
                                }
                            }
                        }
                        oak_core::GreenTree::Leaf(leaf) => {
                            let leaf_kind: KotlinTokenType = leaf.kind;
                            if leaf_kind == KotlinTokenType::Identifier && !found_name {
                                name = source.get_text_in((inner_offset..inner_offset + leaf.length as usize).into()).to_string();
                                found_name = true
                            }
                        }
                    }
                    inner_offset += child.len() as usize
                }
                Some(crate::ast::Declaration::Class { name, members, span: (start..start + node.text_len as usize).into() })
            }
            KotlinElementType::FunctionDeclaration => {
                let mut name = "main".to_string();
                let mut params = vec![];
                let mut body = vec![];
                let mut inner_offset = offset;
                let mut found_name = false;

                for child in node.children() {
                    match child {
                        oak_core::GreenTree::Node(child_node) => {
                            let child_kind: KotlinElementType = child_node.kind;
                            match child_kind {
                                KotlinElementType::Parameter => {
                                    if let Some(p) = self.build_parameter(child_node, source, inner_offset) {
                                        params.push(p)
                                    }
                                }
                                KotlinElementType::Block => {
                                    let mut block_offset = inner_offset;
                                    for stmt in child_node.children() {
                                        if let oak_core::GreenTree::Node(s_node) = stmt {
                                            if let Some(s) = self.build_statement(s_node, source, block_offset) {
                                                body.push(s)
                                            }
                                        }
                                        block_offset += stmt.len() as usize
                                    }
                                }
                                _ => {}
                            }
                        }
                        oak_core::GreenTree::Leaf(leaf) => {
                            let leaf_kind: KotlinTokenType = leaf.kind;
                            if leaf_kind == KotlinTokenType::Identifier && !found_name {
                                name = source.get_text_in((inner_offset..inner_offset + leaf.length as usize).into()).to_string();
                                found_name = true
                            }
                        }
                    }
                    inner_offset += child.len() as usize
                }
                Some(crate::ast::Declaration::Function { name, params, body, span: (start..start + node.text_len as usize).into() })
            }
            KotlinElementType::VariableDeclaration => {
                let mut name = "v".to_string();
                let mut is_val = true;
                let mut inner_offset = offset;
                for child in node.children() {
                    match child {
                        oak_core::GreenTree::Leaf(leaf) => {
                            let leaf_kind: KotlinTokenType = leaf.kind;
                            if leaf_kind == KotlinTokenType::Identifier {
                                name = source.get_text_in((inner_offset..inner_offset + leaf.length as usize).into()).to_string()
                            }
                            else if leaf_kind == KotlinTokenType::Var {
                                is_val = false
                            }
                        }
                        _ => {}
                    }
                    inner_offset += child.len() as usize
                }
                Some(crate::ast::Declaration::Variable { name, is_val, span: (start..start + node.text_len as usize).into() })
            }
            _ => None,
        }
    }

    fn build_parameter(&self, node: &oak_core::GreenNode<KotlinLanguage>, source: &SourceText, offset: usize) -> Option<crate::ast::Parameter> {
        use crate::lexer::token_type::KotlinTokenType;
        let mut name = "p".to_string();
        let mut inner_offset = offset;
        for child in node.children() {
            if let oak_core::GreenTree::Leaf(leaf) = child {
                let leaf_kind: KotlinTokenType = leaf.kind;
                if leaf_kind == KotlinTokenType::Identifier {
                    name = source.get_text_in((inner_offset..inner_offset + leaf.length as usize).into()).to_string()
                }
            }
            inner_offset += child.len() as usize
        }
        Some(crate::ast::Parameter { name, type_name: None, span: (offset..offset + node.text_len as usize).into() })
    }

    fn build_statement(&self, node: &oak_core::GreenNode<KotlinLanguage>, source: &SourceText, offset: usize) -> Option<crate::ast::Statement> {
        use crate::{lexer::token_type::KotlinTokenType, parser::element_type::KotlinElementType};
        let kind: KotlinElementType = node.kind;

        match kind {
            KotlinElementType::ReturnStatement => {
                let mut inner_offset = offset;
                let mut expr = None;
                for child in node.children() {
                    if let oak_core::GreenTree::Node(child_node) = child {
                        // Assuming the first node after 'return' is the expression
                        expr = Some(source.get_text_in((inner_offset..inner_offset + child_node.text_len as usize).into()).to_string());
                        break;
                    }
                    inner_offset += child.len() as usize
                }
                Some(crate::ast::Statement::Return(expr))
            }
            KotlinElementType::VariableDeclaration => {
                let mut name = "v".to_string();
                let mut is_val = true;
                let mut inner_offset = offset;
                for child in node.children() {
                    match child {
                        oak_core::GreenTree::Leaf(leaf) => {
                            let leaf_kind: KotlinTokenType = leaf.kind;
                            if leaf_kind == KotlinTokenType::Identifier {
                                name = source.get_text_in((inner_offset..inner_offset + leaf.length as usize).into()).to_string()
                            }
                            else if leaf_kind == KotlinTokenType::Var {
                                is_val = false
                            }
                        }
                        _ => {}
                    }
                    inner_offset += child.len() as usize
                }
                Some(crate::ast::Statement::Variable { name, is_val })
            }
            KotlinElementType::AssignmentExpression => {
                let mut target = String::new();
                let mut value = String::new();
                let mut inner_offset = offset;
                let mut found_assign = false;

                for child in node.children() {
                    match child {
                        oak_core::GreenTree::Node(child_node) => {
                            let text = source.get_text_in((inner_offset..inner_offset + child_node.text_len as usize).into()).to_string();
                            if !found_assign { target = text } else { value = text }
                        }
                        oak_core::GreenTree::Leaf(leaf) => {
                            let leaf_kind: KotlinTokenType = leaf.kind;
                            if leaf_kind == KotlinTokenType::Equals {
                                found_assign = true
                            }
                        }
                    }
                    inner_offset += child.len() as usize
                }
                Some(crate::ast::Statement::Assignment { target, value })
            }
            _ => None,
        }
    }
}
