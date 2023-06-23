use crate::{
    MojoLanguage,
    ast::{MojoExpression, MojoLiteral, MojoStatement},
    parser::MojoElementType,
};
use oak_core::{GreenNode, GreenTree, OakError, Source, source::SourceText};

/// Mojo 语法树构建器
pub struct MojoBuilder<'a> {
    source: &'a SourceText,
}

impl<'a> MojoBuilder<'a> {
    /// 创建新的构建器
    pub fn new(source: &'a SourceText) -> Self {
        Self { source }
    }

    /// 从 GreenNode 构建 AST
    pub fn build_root(&self, green: &GreenNode<MojoLanguage>) -> Result<Vec<MojoStatement>, OakError> {
        let mut statements = Vec::new();
        let mut offset = 0;

        for child in green.children() {
            if let GreenTree::Node(node) = child {
                if !node.kind.is_trivia() {
                    if let Some(stmt) = self.build_statement(node, offset)? {
                        statements.push(stmt);
                    }
                }
            }
            offset += child.len() as usize;
        }

        Ok(statements)
    }

    fn build_statement(&self, node: &GreenNode<MojoLanguage>, offset: usize) -> Result<Option<MojoStatement>, OakError> {
        match node.kind {
            MojoElementType::FunctionDef => self.build_function_def(node, offset).map(Some),
            MojoElementType::VariableDecl => self.build_variable_decl(node, offset).map(Some),
            MojoElementType::IfStatement => self.build_if_stmt(node, offset).map(Some),
            MojoElementType::WhileStatement => self.build_while_stmt(node, offset).map(Some),
            MojoElementType::ReturnStatement => self.build_return_stmt(node, offset).map(Some),
            MojoElementType::ExpressionStatement => self.build_expression_stmt(node, offset).map(Some),
            _ => Ok(None),
        }
    }

    fn build_function_def(&self, node: &GreenNode<MojoLanguage>, offset: usize) -> Result<MojoStatement, OakError> {
        let mut name = String::new();
        let mut params = Vec::new();
        let mut return_type = None;
        let mut body = Vec::new();
        let mut current_offset = offset;

        for child in node.children() {
            if let GreenTree::Node(child_node) = child {
                match child_node.kind {
                    MojoElementType::Identifier => {
                        name = self.source.get_text_in((current_offset..current_offset + child.len() as usize).into()).to_string();
                    }
                    MojoElementType::ParamList => {
                        // TODO: Parse parameters
                    }
                    MojoElementType::Block => {
                        body = self.build_root(child_node)?;
                    }
                    _ => {}
                }
            }
            current_offset += child.len() as usize;
        }

        Ok(MojoStatement::Function { name, params, return_type, body })
    }

    fn build_variable_decl(&self, node: &GreenNode<MojoLanguage>, offset: usize) -> Result<MojoStatement, OakError> {
        let mut name = String::new();
        let mut ty = None;
        let mut value = None;
        let mut is_let = false;
        let mut current_offset = offset;

        for child in node.children() {
            if let GreenTree::Node(child_node) = child {
                match child_node.kind {
                    MojoElementType::Var => is_let = false,
                    MojoElementType::Let => is_let = true,
                    MojoElementType::Identifier => {
                        name = self.source.get_text_in((current_offset..current_offset + child.len() as usize).into()).to_string();
                    }
                    MojoElementType::BinaryExpr | MojoElementType::LiteralExpr | MojoElementType::IdentifierExpr | MojoElementType::Grouping => {
                        value = Some(self.build_expression(child_node, current_offset)?);
                    }
                    _ => {}
                }
            }
            current_offset += child.len() as usize;
        }

        Ok(MojoStatement::Variable { name, ty, value, is_let })
    }

    fn build_if_stmt(&self, node: &GreenNode<MojoLanguage>, offset: usize) -> Result<MojoStatement, OakError> {
        let mut condition = MojoExpression::Literal(MojoLiteral::None);
        let mut then_body = Vec::new();
        let mut else_body = None;
        let mut current_offset = offset;

        for child in node.children() {
            if let GreenTree::Node(child_node) = child {
                match child_node.kind {
                    MojoElementType::BinaryExpr | MojoElementType::LiteralExpr | MojoElementType::IdentifierExpr | MojoElementType::Grouping => {
                        condition = self.build_expression(child_node, current_offset)?;
                    }
                    MojoElementType::Block => {
                        if then_body.is_empty() {
                            then_body = self.build_root(child_node)?;
                        }
                        else {
                            else_body = Some(self.build_root(child_node)?);
                        }
                    }
                    MojoElementType::IfStatement => {
                        // Elif case, but Mojo usually handles this via nested Ifs in Block
                        // or we could transform it here.
                    }
                    _ => {}
                }
            }
            current_offset += child.len() as usize;
        }

        Ok(MojoStatement::If { condition, then_body, else_body })
    }

    fn build_while_stmt(&self, node: &GreenNode<MojoLanguage>, offset: usize) -> Result<MojoStatement, OakError> {
        let mut condition = MojoExpression::Literal(MojoLiteral::None);
        let mut body = Vec::new();
        let mut current_offset = offset;

        for child in node.children() {
            if let GreenTree::Node(child_node) = child {
                match child_node.kind {
                    MojoElementType::BinaryExpr | MojoElementType::LiteralExpr | MojoElementType::IdentifierExpr | MojoElementType::Grouping => {
                        condition = self.build_expression(child_node, current_offset)?;
                    }
                    MojoElementType::Block => {
                        body = self.build_root(child_node)?;
                    }
                    _ => {}
                }
            }
            current_offset += child.len() as usize;
        }

        Ok(MojoStatement::While { condition, body })
    }

    fn build_return_stmt(&self, node: &GreenNode<MojoLanguage>, offset: usize) -> Result<MojoStatement, OakError> {
        let mut value = None;
        let mut current_offset = offset;

        for child in node.children() {
            if let GreenTree::Node(child_node) = child {
                if !child_node.kind.is_trivia() && child_node.kind != MojoElementType::Return {
                    value = Some(self.build_expression(child_node, current_offset)?);
                }
            }
            current_offset += child.len() as usize;
        }

        Ok(MojoStatement::Return(value))
    }

    fn build_expression_stmt(&self, node: &GreenNode<MojoLanguage>, offset: usize) -> Result<MojoStatement, OakError> {
        let mut expr = MojoExpression::Literal(MojoLiteral::None);
        let mut current_offset = offset;

        for child in node.children() {
            if let GreenTree::Node(child_node) = child {
                if !child_node.kind.is_trivia() {
                    expr = self.build_expression(child_node, current_offset)?;
                }
            }
            current_offset += child.len() as usize;
        }

        Ok(MojoStatement::Expression(expr))
    }

    fn build_expression(&self, node: &GreenNode<MojoLanguage>, offset: usize) -> Result<MojoExpression, OakError> {
        match node.kind {
            MojoElementType::LiteralExpr => {
                let text = self.source.get_text_in((offset..offset + node.text_len() as usize).into());
                if text.contains('.') {
                    Ok(MojoExpression::Literal(MojoLiteral::Float(text.parse().unwrap_or(0.0))))
                }
                else if text.starts_with('"') || text.starts_with('\'') {
                    Ok(MojoExpression::Literal(MojoLiteral::String(text[1..text.len() - 1].to_string())))
                }
                else if text == "True" {
                    Ok(MojoExpression::Literal(MojoLiteral::Bool(true)))
                }
                else if text == "False" {
                    Ok(MojoExpression::Literal(MojoLiteral::Bool(false)))
                }
                else if text == "None" {
                    Ok(MojoExpression::Literal(MojoLiteral::None))
                }
                else {
                    Ok(MojoExpression::Literal(MojoLiteral::Int(text.parse().unwrap_or(0))))
                }
            }
            MojoElementType::IdentifierExpr => {
                let text = self.source.get_text_in((offset..offset + node.text_len() as usize).into());
                Ok(MojoExpression::Identifier(text.to_string()))
            }
            MojoElementType::BinaryExpr => {
                let mut left = None;
                let mut op = String::new();
                let mut right = None;
                let mut current_offset = offset;

                for child in node.children() {
                    if let GreenTree::Node(child_node) = child {
                        if !child_node.kind.is_trivia() {
                            if left.is_none() {
                                left = Some(Box::new(self.build_expression(child_node, current_offset)?));
                            }
                            else if right.is_none()
                                && (child_node.kind == MojoElementType::BinaryExpr || child_node.kind == MojoElementType::LiteralExpr || child_node.kind == MojoElementType::IdentifierExpr || child_node.kind == MojoElementType::Grouping)
                            {
                                right = Some(Box::new(self.build_expression(child_node, current_offset)?));
                            }
                            else {
                                // Assume it's the operator
                                op = self.source.get_text_in((current_offset..current_offset + child.len() as usize).into()).to_string();
                            }
                        }
                    }
                    current_offset += child.len() as usize;
                }

                Ok(MojoExpression::Binary { left: left.unwrap_or_else(|| Box::new(MojoExpression::Literal(MojoLiteral::None))), op, right: right.unwrap_or_else(|| Box::new(MojoExpression::Literal(MojoLiteral::None))) })
            }
            MojoElementType::Grouping => {
                let mut expr = MojoExpression::Literal(MojoLiteral::None);
                let mut current_offset = offset;
                for child in node.children() {
                    if let GreenTree::Node(child_node) = child {
                        if !child_node.kind.is_trivia() && child_node.kind != MojoElementType::LeftParen && child_node.kind != MojoElementType::RightParen {
                            expr = self.build_expression(child_node, current_offset)?;
                        }
                    }
                    current_offset += child.len() as usize;
                }
                Ok(expr)
            }
            _ => Ok(MojoExpression::Literal(MojoLiteral::None)),
        }
    }
}
