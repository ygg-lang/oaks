use crate::{ast::*, builder::JavaBuilder, language::JavaLanguage, lexer::token_type::JavaTokenType, parser::element_type::JavaElementType};
use oak_core::tree::red_tree::{RedNode, RedTree};

impl<'config> JavaBuilder<'config> {
    pub(crate) fn build_block(&self, node: RedNode<JavaLanguage>, source: &str) -> Vec<Statement> {
        let mut statements = Vec::new();
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                statements.push(self.build_statement(sub_node, source))
            }
        }
        statements
    }

    pub(crate) fn build_catch_clause(&self, node: RedNode<JavaLanguage>, source: &str) -> CatchClause {
        let mut parameter = Parameter { name: "".to_string(), r#type: "".to_string() };
        let mut block = Vec::new();
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                match sub_node.green.kind {
                    JavaElementType::Parameter => parameter = self.build_parameter(sub_node, source),
                    JavaElementType::BlockStatement => block = self.build_block(sub_node, source),
                    _ => {}
                }
            }
        }
        CatchClause { parameter, block }
    }

    pub(crate) fn build_switch_case(&self, node: RedNode<JavaLanguage>, source: &str) -> SwitchCase {
        let mut label = None;
        let mut body = Vec::new();
        for child in node.children() {
            if let RedTree::Node(sub_node) = child {
                if label.is_none() && sub_node.green.kind != JavaElementType::BlockStatement {
                    label = Some(self.build_expression(sub_node, source))
                }
                else if sub_node.green.kind == JavaElementType::BlockStatement {
                    body.extend(self.build_block(sub_node, source))
                }
                else {
                    body.push(self.build_statement(sub_node, source))
                }
            }
        }
        SwitchCase { label: label.unwrap_or(Expression::Literal(Literal::Integer(0))), body }
    }

    pub(crate) fn build_statement(&self, node: RedNode<JavaLanguage>, source: &str) -> Statement {
        match node.green.kind {
            JavaElementType::ExpressionStatement => {
                let mut expr_node = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        expr_node = Some(sub_node);
                        break;
                    }
                }
                if let Some(sub_node) = expr_node {
                    let expr = self.build_expression(sub_node.clone(), source);
                    return Statement::Expression(expr);
                }
                Statement::Block(vec![])
            }
            JavaElementType::BlockStatement => Statement::Block(self.build_block(node, source)),
            JavaElementType::ReturnStatement => {
                let mut expr = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        expr = Some(self.build_expression(sub_node, source));
                        break;
                    }
                }
                Statement::Return(expr)
            }
            JavaElementType::IfStatement => {
                let mut condition = None;
                let mut then_branch = None;
                let mut else_branch = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if condition.is_none() {
                            condition = Some(self.build_expression(sub_node, source))
                        }
                        else if then_branch.is_none() {
                            then_branch = Some(Box::new(self.build_statement(sub_node, source)))
                        }
                        else {
                            else_branch = Some(Box::new(self.build_statement(sub_node, source)))
                        }
                    }
                }
                Statement::If { condition: condition.unwrap_or(Expression::Literal(Literal::Boolean(true))), then_branch: then_branch.unwrap_or(Box::new(Statement::Block(vec![]))), else_branch }
            }
            JavaElementType::WhileStatement => {
                let mut condition = None;
                let mut body = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if condition.is_none() { condition = Some(self.build_expression(sub_node, source)) } else { body = Some(Box::new(self.build_statement(sub_node, source))) }
                    }
                }
                Statement::While { condition: condition.unwrap_or(Expression::Literal(Literal::Boolean(true))), body: body.unwrap_or(Box::new(Statement::Block(vec![]))) }
            }
            JavaElementType::DoWhileStatement => {
                let mut condition = None;
                let mut body = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if body.is_none() { body = Some(Box::new(self.build_statement(sub_node, source))) } else { condition = Some(self.build_expression(sub_node, source)) }
                    }
                }
                Statement::DoWhile { condition: condition.unwrap_or(Expression::Literal(Literal::Boolean(true))), body: body.unwrap_or(Box::new(Statement::Block(vec![]))) }
            }
            JavaElementType::ForStatement => {
                let mut init = None;
                let mut condition = None;
                let mut update = None;
                let mut body = None;
                let mut semicolon_count = 0;
                let mut after_right_paren = false;

                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == JavaTokenType::Semicolon {
                                semicolon_count += 1
                            }
                            else if leaf.kind == JavaTokenType::RightParen {
                                after_right_paren = true
                            }
                        }
                        RedTree::Node(sub_node) => {
                            if after_right_paren {
                                if body.is_none() {
                                    body = Some(Box::new(self.build_statement(sub_node, source)))
                                }
                            }
                            else {
                                match semicolon_count {
                                    0 => init = Some(Box::new(self.build_statement(sub_node, source))),
                                    1 => condition = Some(self.build_expression(sub_node, source)),
                                    2 => update = Some(self.build_expression(sub_node, source)),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                Statement::For { init, condition, update, body: body.unwrap_or(Box::new(Statement::Block(vec![]))) }
            }
            JavaElementType::SwitchStatement => {
                let mut selector = None;
                let mut cases = Vec::new();
                let mut default = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        match sub_node.green.kind {
                            JavaElementType::SwitchCase => cases.push(self.build_switch_case(sub_node, source)),
                            JavaElementType::DefaultCase => default = Some(self.build_block(sub_node, source)),
                            _ => {
                                if selector.is_none() {
                                    selector = Some(self.build_expression(sub_node, source))
                                }
                            }
                        }
                    }
                }
                Statement::Switch { selector: selector.unwrap_or(Expression::Identifier("unknown_selector".to_string())), cases, default }
            }
            JavaElementType::Break => Statement::Break,
            JavaElementType::Continue => Statement::Continue,
            JavaElementType::TryStatement => {
                let mut block = Vec::new();
                let mut catches = Vec::new();
                let mut finally = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        match sub_node.green.kind {
                            JavaElementType::BlockStatement => {
                                if block.is_empty() {
                                    block = self.build_block(sub_node, source);
                                }
                                else {
                                    finally = Some(self.build_block(sub_node, source));
                                }
                            }
                            JavaElementType::CatchClause => catches.push(self.build_catch_clause(sub_node, source)),
                            _ => {}
                        }
                    }
                }
                Statement::Try(TryStatement { block, catches, finally })
            }
            JavaElementType::ThrowStatement => {
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        return Statement::Throw(self.build_expression(sub_node, source));
                    }
                }
                Statement::Throw(Expression::Identifier("unknown_throw".to_string()))
            }
            JavaElementType::VariableDeclaration => {
                let mut r#type = String::new();
                let mut name = String::new();
                let mut initializer = None;

                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => match leaf.kind {
                            JavaTokenType::Identifier => {
                                if r#type.is_empty() {
                                    r#type = self.get_text(leaf.span, source).to_string()
                                }
                                else if name.is_empty() {
                                    name = self.get_text(leaf.span, source).to_string()
                                }
                            }
                            JavaTokenType::Int | JavaTokenType::Boolean | JavaTokenType::Void | JavaTokenType::Long | JavaTokenType::Float | JavaTokenType::Double | JavaTokenType::Char | JavaTokenType::Byte | JavaTokenType::Short => {
                                if r#type.is_empty() {
                                    r#type = self.get_text(leaf.span, source).to_string()
                                }
                            }
                            JavaTokenType::LeftBracket => {
                                if !r#type.is_empty() {
                                    r#type.push_str("[]")
                                }
                            }
                            _ => {}
                        },
                        RedTree::Node(sub_node) => {
                            if sub_node.green.kind == JavaElementType::Identifier {
                                if r#type.is_empty() {
                                    r#type = self.extract_identifier(sub_node, source)
                                }
                                else if name.is_empty() {
                                    name = self.extract_identifier(sub_node, source)
                                }
                                else {
                                    initializer = Some(self.build_expression(sub_node, source))
                                }
                            }
                            else {
                                initializer = Some(self.build_expression(sub_node, source))
                            }
                        }
                    }
                }
                Statement::LocalVariable { r#type, name, initializer }
            }
            _ => Statement::Block(vec![]),
        }
    }
}
