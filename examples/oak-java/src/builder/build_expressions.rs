use crate::{ast::*, builder::JavaBuilder, language::JavaLanguage, lexer::token_type::JavaTokenType, parser::element_type::JavaElementType};
use oak_core::{
    TokenType,
    tree::red_tree::{RedNode, RedTree},
};

impl<'config> JavaBuilder<'config> {
    /// Builds an expression from the AST node.
    pub(crate) fn build_expression(&self, node: RedNode<JavaLanguage>, source: &str) -> Expression {
        match node.green.kind {
            JavaElementType::AssignmentExpression => {
                let mut left = None;
                let mut right = None;
                let mut op = String::new();
                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => {
                            if left.is_none() {
                                left = Some(Box::new(self.build_expression(sub_node, source)))
                            }
                            else {
                                right = Some(Box::new(self.build_expression(sub_node, source)))
                            }
                        }
                        RedTree::Leaf(leaf) => {
                            if leaf.kind.role() == oak_core::UniversalTokenRole::Operator || leaf.kind == JavaTokenType::Instanceof {
                                op = self.get_text(leaf.span, source).trim().to_string()
                            }
                        }
                    }
                }
                if let (Some(left), Some(right)) = (left, right) { Expression::Assignment { left, op, right } } else { Expression::Identifier("unknown_assignment".to_string()) }
            }
            JavaElementType::UnaryExpression => {
                let mut expr = None;
                let mut op = String::new();
                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => expr = Some(Box::new(self.build_expression(sub_node, source))),
                        RedTree::Leaf(leaf) => {
                            if leaf.kind.role() == oak_core::UniversalTokenRole::Operator || leaf.kind == JavaTokenType::Instanceof {
                                op = self.get_text(leaf.span, source).trim().to_string();
                            }
                        }
                    }
                }
                if let Some(expression) = expr { if op == "++" || op == "--" { Expression::Update { expression, op, is_prefix: true } } else { Expression::Unary { op, expression } } } else { Expression::Identifier("unknown_unary".to_string()) }
            }
            JavaElementType::PostfixExpression => {
                let mut expr = None;
                let mut op = String::new();
                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => expr = Some(Box::new(self.build_expression(sub_node, source))),
                        RedTree::Leaf(leaf) => {
                            if leaf.kind.role() == oak_core::UniversalTokenRole::Operator || leaf.kind == JavaTokenType::Instanceof {
                                op = self.get_text(leaf.span, source).trim().to_string();
                            }
                        }
                    }
                }
                if let Some(expression) = expr { Expression::Update { expression, op, is_prefix: false } } else { Expression::Identifier("unknown_postfix".to_string()) }
            }
            JavaElementType::TernaryExpression => {
                let mut condition = None;
                let mut then_branch = None;
                let mut else_branch = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if condition.is_none() {
                            condition = Some(Box::new(self.build_expression(sub_node, source)));
                        }
                        else if then_branch.is_none() {
                            then_branch = Some(Box::new(self.build_expression(sub_node, source)));
                        }
                        else {
                            else_branch = Some(Box::new(self.build_expression(sub_node, source)));
                        }
                    }
                }
                if let (Some(condition), Some(then_branch), Some(else_branch)) = (condition, then_branch, else_branch) { Expression::Ternary { condition, then_branch, else_branch } } else { Expression::Identifier("unknown_ternary".to_string()) }
            }
            JavaElementType::CastExpression => {
                let mut target_type = String::new();
                let mut expr = None;
                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => {
                            if target_type.is_empty() {
                                target_type = self.extract_identifier(sub_node, source);
                            }
                            else {
                                expr = Some(Box::new(self.build_expression(sub_node, source)));
                            }
                        }
                        RedTree::Leaf(leaf) => {
                            if target_type.is_empty() && (leaf.kind == JavaTokenType::Identifier || leaf.kind.role() == oak_core::UniversalTokenRole::Keyword) {
                                target_type = self.get_text(leaf.span, source).trim().to_string();
                            }
                        }
                    }
                }
                if let Some(expression) = expr { Expression::Cast { target_type, expression } } else { Expression::Identifier("unknown_cast".to_string()) }
            }
            JavaElementType::BinaryExpression => {
                let mut left = None;
                let mut right = None;
                let mut op = String::new();
                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => {
                            if left.is_none() {
                                left = Some(Box::new(self.build_expression(sub_node, source)))
                            }
                            else {
                                right = Some(Box::new(self.build_expression(sub_node, source)))
                            }
                        }
                        RedTree::Leaf(leaf) => {
                            if leaf.kind.role() == oak_core::UniversalTokenRole::Operator || leaf.kind == JavaTokenType::Instanceof {
                                op = self.get_text(leaf.span, source).trim().to_string();
                            }
                        }
                    }
                }
                if let (Some(left), Some(right)) = (left, right) { Expression::Binary { left, op, right } } else { Expression::Identifier("unknown_binary".to_string()) }
            }
            JavaElementType::LiteralExpression => {
                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => match leaf.kind {
                            JavaTokenType::IntegerLiteral => {
                                let text = self.get_text(leaf.span, source).trim().to_string();
                                let val = text.parse().unwrap_or(0);
                                return Expression::Literal(Literal::Integer(val));
                            }
                            JavaTokenType::FloatingPointLiteral => {
                                let text = self.get_text(leaf.span, source).trim().to_string();
                                let val = text.parse().unwrap_or(0.0);
                                return Expression::Literal(Literal::Float(val));
                            }
                            JavaTokenType::StringLiteral => {
                                let text = self.get_text(leaf.span, source).trim();
                                let content = if text.len() >= 2 { &text[1..text.len() - 1] } else { text };
                                return Expression::Literal(Literal::String(content.to_string()));
                            }
                            JavaTokenType::BooleanLiteral => {
                                let val = self.get_text(leaf.span, source).trim() == "true";
                                return Expression::Literal(Literal::Boolean(val));
                            }
                            _ => {}
                        },
                        RedTree::Node(sub_node) => {
                            let expr = self.build_expression(sub_node, source);
                            if let Expression::Literal(_) = expr {
                                return expr;
                            }
                        }
                    }
                }
                let text = self.get_text(node.span(), source).trim().to_string();
                if let Ok(val) = text.parse::<i64>() {
                    return Expression::Literal(Literal::Integer(val));
                }
                if let Ok(val) = text.parse::<f64>() {
                    return Expression::Literal(Literal::Float(val));
                }
                Expression::Identifier("unknown".to_string())
            }
            JavaElementType::MethodCall => {
                let mut callee = None;
                let mut arguments = Vec::new();

                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => {
                            if callee.is_none() {
                                callee = Some(self.build_expression(sub_node, source));
                            }
                            else {
                                arguments.push(self.build_expression(sub_node, source));
                            }
                        }
                        _ => {}
                    }
                }

                if let Some(expr) = callee {
                    match expr {
                        Expression::FieldAccess(fa) => Expression::MethodCall(MethodCall { target: Some(fa.target), name: fa.name, arguments }),
                        Expression::Identifier(name) => Expression::MethodCall(MethodCall { target: None, name, arguments }),
                        _ => Expression::MethodCall(MethodCall { target: Some(Box::new(expr)), name: "unknown".to_string(), arguments }),
                    }
                }
                else {
                    Expression::Identifier("unknown".to_string())
                }
            }
            JavaElementType::MemberSelect => {
                let mut target = None;
                let mut name = String::new();
                for child in node.children() {
                    match child {
                        RedTree::Node(sub_node) => target = Some(Box::new(self.build_expression(sub_node, source))),
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == JavaTokenType::Identifier {
                                name = self.get_text(leaf.span, source).trim().to_string();
                            }
                        }
                    }
                }
                if let Some(target) = target { Expression::FieldAccess(FieldAccess { target, name }) } else { Expression::Identifier(name) }
            }
            JavaElementType::ArrayAccess => {
                let mut target = None;
                let mut index = None;
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        if target.is_none() {
                            target = Some(Box::new(self.build_expression(sub_node, source)));
                        }
                        else {
                            index = Some(Box::new(self.build_expression(sub_node, source)));
                        }
                    }
                }
                if let (Some(target), Some(index)) = (target, index) { Expression::ArrayAccess(ArrayAccess { target, index }) } else { Expression::Identifier("unknown_array_access".to_string()) }
            }
            JavaElementType::ArrayCreation => {
                let mut element_type = String::new();
                let mut dimensions = Vec::new();
                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == JavaTokenType::Identifier || leaf.kind.role() == oak_core::UniversalTokenRole::Keyword {
                                if element_type.is_empty() {
                                    element_type = self.get_text(leaf.span, source).to_string()
                                }
                            }
                        }
                        RedTree::Node(sub_node) => dimensions.push(self.build_expression(sub_node, source)),
                    }
                }
                Expression::ArrayCreation(ArrayCreation { element_type, dimensions })
            }
            JavaElementType::Identifier => {
                let name = self.get_text(node.span(), source).trim().to_string();
                match name.as_str() {
                    "this" => Expression::This,
                    "super" => Expression::Super,
                    _ => Expression::Identifier(name),
                }
            }
            JavaElementType::NewExpression => {
                let mut r#type = String::new();
                let mut arguments = Vec::new();
                for child in node.children() {
                    match child {
                        RedTree::Leaf(leaf) => {
                            if leaf.kind == JavaTokenType::Identifier && r#type.is_empty() {
                                r#type = self.get_text(leaf.span, source).to_string()
                            }
                        }
                        RedTree::Node(sub_node) => {
                            if r#type.is_empty() {
                                r#type = self.extract_identifier(sub_node, source)
                            }
                            else {
                                arguments.push(self.build_expression(sub_node, source));
                            }
                        }
                    }
                }
                Expression::New(NewExpression { r#type, arguments })
            }
            JavaElementType::ParenthesizedExpression => {
                for child in node.children() {
                    if let RedTree::Node(sub_node) = child {
                        return self.build_expression(sub_node, source);
                    }
                }
                Expression::Identifier("unknown_parenthesized".to_string())
            }
            _ => {
                for child in node.children() {
                    if let RedTree::Leaf(leaf) = child {
                        if leaf.kind == JavaTokenType::Identifier {
                            let name = self.get_text(leaf.span, source).trim().to_string();
                            return Expression::Identifier(name);
                        }
                    }
                    else if let RedTree::Node(sub_node) = child {
                        return self.build_expression(sub_node, source);
                    }
                }
                Expression::Identifier("unknown".to_string())
            }
        }
    }
}
