use crate::{
    ast::{AugmentedOperator, ExceptHandler, Expression, ImportName, Literal, Statement, WithItem},
    language::PythonLanguage,
    lexer::token_type::PythonTokenType,
    parser::PythonElementType,
};
use oak_core::{GreenNode, GreenTree, OakError, Source, SourceText};

impl<'config> super::PythonBuilder<'config> {
    /// Builds a statement from a green node.
    pub(crate) fn build_statement(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Option<Statement>, OakError> {
        match node.kind {
            PythonElementType::FunctionDef | PythonElementType::AsyncFunctionDef => {
                let is_async = node.kind == PythonElementType::AsyncFunctionDef;
                let mut name = String::new();
                let mut parameters = Vec::new();
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => name = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string(),
                        GreenTree::Node(n) if n.kind == PythonElementType::Arguments => parameters = self.build_parameters(n, current_offset, source)?,
                        GreenTree::Node(n) if n.kind == PythonElementType::Suite => body = self.build_suite(n, current_offset, source)?,
                        _ => {}
                    }
                    current_offset += child_len
                }
                if is_async { Ok(Some(Statement::AsyncFunctionDef { decorators: Vec::new(), name, parameters, return_type: None, body })) } else { Ok(Some(Statement::FunctionDef { decorators: Vec::new(), name, parameters, return_type: None, body })) }
            }
            PythonElementType::ClassDef => {
                let mut name = String::new();
                let mut bases = Vec::new();
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => name = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string(),
                        GreenTree::Node(n) if n.kind == PythonElementType::Tuple || n.kind == PythonElementType::List => {
                            // Simple bases handling: if it's a tuple or list in class def, treat as bases
                            let expr = self.build_expression(n, current_offset, source)?;
                            match expr {
                                Expression::Tuple { elts } => bases = elts,
                                Expression::List { elts } => bases = elts,
                                _ => bases.push(expr),
                            }
                        }
                        GreenTree::Node(n) if n.kind == PythonElementType::Suite => body = self.build_suite(n, current_offset, source)?,
                        GreenTree::Node(n) => {
                            // Try to see if it's an expression (base class)
                            if !n.kind.is_trivia() && n.kind != PythonElementType::Suite {
                                let expr = self.build_expression(n, current_offset, source)?;
                                bases.push(expr)
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::ClassDef { decorators: Vec::new(), name, bases, body }))
            }
            PythonElementType::ReturnStmt | PythonElementType::Return => {
                let mut value = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            value = Some(self.build_expression(n, current_offset, source)?)
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Return(value)))
            }
            PythonElementType::AssignStmt => {
                let mut left = None;
                let mut right = None;
                let mut operator = None;
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_trivia() {
                                if left.is_none() {
                                    left = Some(self.build_expression(n, current_offset, source)?);
                                }
                                else {
                                    right = Some(self.build_expression(n, current_offset, source)?);
                                }
                            }
                        }
                        GreenTree::Leaf(leaf) => {
                            match leaf.kind {
                                PythonTokenType::Assign => {} // Ignore =
                                PythonTokenType::PlusAssign => operator = Some(AugmentedOperator::Add),
                                PythonTokenType::MinusAssign => operator = Some(AugmentedOperator::Sub),
                                PythonTokenType::StarAssign => operator = Some(AugmentedOperator::Mult),
                                PythonTokenType::SlashAssign => operator = Some(AugmentedOperator::Div),
                                PythonTokenType::DoubleSlashAssign => operator = Some(AugmentedOperator::FloorDiv),
                                PythonTokenType::PercentAssign => operator = Some(AugmentedOperator::Mod),
                                PythonTokenType::DoubleStarAssign => operator = Some(AugmentedOperator::Pow),
                                PythonTokenType::LeftShiftAssign => operator = Some(AugmentedOperator::LShift),
                                PythonTokenType::RightShiftAssign => operator = Some(AugmentedOperator::RShift),
                                PythonTokenType::AmpersandAssign => operator = Some(AugmentedOperator::BitAnd),
                                PythonTokenType::PipeAssign => operator = Some(AugmentedOperator::BitOr),
                                PythonTokenType::CaretAssign => operator = Some(AugmentedOperator::BitXor),
                                _ => {}
                            }
                        }
                    }
                    current_offset += child_len
                }

                if let (Some(target), Some(value)) = (left, right) { if let Some(op) = operator { Ok(Some(Statement::AugmentedAssignment { target, operator: op, value })) } else { Ok(Some(Statement::Assignment { target, value })) } } else { Ok(None) }
            }

            PythonElementType::ExprStmt | PythonElementType::Expr => {
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_trivia() {
                                if n.kind == PythonElementType::AssignStmt {
                                    return self.build_statement(n, current_offset, source);
                                }
                                return Ok(Some(Statement::Expression(self.build_expression(n, current_offset, source)?)));
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_len
                }
                Ok(None)
            }
            PythonElementType::IfStmt | PythonElementType::If => {
                let mut test = None;
                let mut body = Vec::new();
                let mut orelse = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if test.is_none() && n.kind != PythonElementType::Suite {
                                test = Some(self.build_expression(n, current_offset, source)?)
                            }
                            else if body.is_empty() && n.kind == PythonElementType::Suite {
                                body = self.build_suite(n, current_offset, source)?
                            }
                            else if n.kind == PythonElementType::Suite {
                                orelse = self.build_suite(n, current_offset, source)?
                            }
                        }
                    }
                    current_offset += child_len
                }

                Ok(Some(Statement::If { test: test.unwrap_or(Expression::Literal(Literal::Boolean(true))), body, orelse }))
            }
            PythonElementType::WhileStmt | PythonElementType::While => {
                let mut test = None;
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if test.is_none() && n.kind != PythonElementType::Suite {
                                test = Some(self.build_expression(n, current_offset, source)?)
                            }
                            else if n.kind == PythonElementType::Suite {
                                body = self.build_suite(n, current_offset, source)?
                            }
                        }
                    }
                    current_offset += child_len
                }

                Ok(Some(Statement::While { test: test.unwrap_or(Expression::Literal(Literal::Boolean(true))), body, orelse: Vec::new() }))
            }
            PythonElementType::ForStmt | PythonElementType::For | PythonElementType::AsyncFor => {
                let is_async = node.kind == PythonElementType::AsyncFor;
                let mut target = None;
                let mut iter = None;
                let mut body = Vec::new();
                let mut orelse = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if target.is_none() && n.kind != PythonElementType::Suite && n.kind != PythonElementType::InKeyword.into() {
                                target = Some(self.build_expression(n, current_offset, source)?)
                            }
                            else if iter.is_none() && n.kind != PythonElementType::Suite && n.kind != PythonElementType::InKeyword.into() {
                                iter = Some(self.build_expression(n, current_offset, source)?)
                            }
                            else if n.kind == PythonElementType::Suite {
                                if body.is_empty() { body = self.build_suite(n, current_offset, source)? } else { orelse = self.build_suite(n, current_offset, source)? }
                            }
                        }
                    }
                    current_offset += child_len
                }

                let target = target.unwrap_or(Expression::Name("invalid_target".to_string()));
                let iter = iter.unwrap_or(Expression::Name("invalid_iter".to_string()));
                if is_async { Ok(Some(Statement::AsyncFor { target, iter, body, orelse })) } else { Ok(Some(Statement::For { target, iter, body, orelse })) }
            }
            PythonElementType::PassStmt | PythonElementType::Pass => Ok(Some(Statement::Pass)),
            PythonElementType::BreakStmt | PythonElementType::Break => Ok(Some(Statement::Break)),
            PythonElementType::ContinueStmt | PythonElementType::Continue => Ok(Some(Statement::Continue)),
            PythonElementType::RaiseStmt | PythonElementType::Raise => {
                let mut exc = None;
                let mut cause = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            let expr = self.build_expression(n, current_offset, source)?;
                            if exc.is_none() { exc = Some(expr) } else { cause = Some(expr) }
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Raise { exc, cause }))
            }
            PythonElementType::AssertStmt | PythonElementType::Assert => {
                let mut test = None;
                let mut msg = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            let expr = self.build_expression(n, current_offset, source)?;
                            if test.is_none() { test = Some(expr) } else { msg = Some(expr) }
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Assert { test: test.unwrap_or(Expression::Literal(Literal::Boolean(true))), msg }))
            }
            PythonElementType::ImportStmt | PythonElementType::Import => {
                let mut names = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if n.kind == PythonElementType::Alias {
                            names.push(self.build_import_name(n, current_offset, source)?)
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Import { names }))
            }
            PythonElementType::ImportFromStmt | PythonElementType::ImportFrom => {
                let mut module = None;
                let mut names = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => {
                            if module.is_none() {
                                module = Some(source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string())
                            }
                        }
                        GreenTree::Node(n) if n.kind == PythonElementType::Alias => names.push(self.build_import_name(n, current_offset, source)?),
                        _ => {}
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::ImportFrom { module, names }))
            }
            PythonElementType::GlobalStmt | PythonElementType::Global => {
                let mut names = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Leaf(leaf) = child {
                        if leaf.kind == PythonTokenType::Identifier {
                            names.push(source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string())
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Global { names }))
            }
            PythonElementType::NonlocalStmt | PythonElementType::Nonlocal => {
                let mut names = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Leaf(leaf) = child {
                        if leaf.kind == PythonTokenType::Identifier {
                            names.push(source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string())
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Nonlocal { names }))
            }
            PythonElementType::TryStmt | PythonElementType::Try => {
                let mut body = Vec::new();
                let mut handlers = Vec::new();
                let mut orelse = Vec::new();
                let mut finalbody = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            match n.kind {
                                PythonElementType::Suite if body.is_empty() && handlers.is_empty() => body = self.build_suite(n, current_offset, source)?,
                                PythonElementType::ExceptHandler => handlers.push(self.build_except_handler(n, current_offset, source)?),
                                PythonElementType::Suite if !handlers.is_empty() && orelse.is_empty() => orelse = self.build_suite(n, current_offset, source)?,
                                PythonElementType::Suite if finalbody.is_empty() => finalbody = self.build_suite(n, current_offset, source)?,
                                _ => {}
                            }
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Try { body, handlers, orelse, finalbody }))
            }
            PythonElementType::WithStmt | PythonElementType::With | PythonElementType::AsyncWith => {
                let mut items = Vec::new();
                let mut body = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if n.kind == PythonElementType::WithItem {
                                items.push(self.build_with_item(n, current_offset, source)?)
                            }
                            else if n.kind == PythonElementType::Suite {
                                body = self.build_suite(n, current_offset, source)?
                            }
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::With { items, body }))
            }
            PythonElementType::DeleteStmt | PythonElementType::Delete => {
                let mut targets = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            targets.push(self.build_expression(n, current_offset, source)?);
                        }
                    }
                    current_offset += child_len
                }
                Ok(Some(Statement::Delete { targets }))
            }
            PythonElementType::Suite => {
                // Suites are handled by build_suite
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    /// Builds a list of statements from a list of green nodes, handling decorators.
    pub(crate) fn build_statement_list<'a>(&self, children: &'a [GreenTree<'a, PythonLanguage>], mut current_offset: usize, source: &SourceText) -> Result<Vec<Statement>, OakError> {
        let mut statements = Vec::new();
        let mut pending_decorators = Vec::new();

        for child in children {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Node(node) => {
                    if !node.kind.is_trivia() {
                        if node.kind == PythonElementType::Decorator {
                            // Collect decorator expression
                            let mut decorator_offset = current_offset;
                            for d_child in node.children() {
                                if let GreenTree::Node(expr_node) = d_child {
                                    if !expr_node.kind.is_trivia() {
                                        pending_decorators.push(self.build_expression(expr_node, decorator_offset, source)?)
                                    }
                                }
                                decorator_offset += d_child.len() as usize
                            }
                        }
                        else if let Some(mut stmt) = self.build_statement(node, current_offset, source)? {
                            // Attach decorators to function or class def
                            match &mut stmt {
                                Statement::FunctionDef { decorators, .. } | Statement::AsyncFunctionDef { decorators, .. } | Statement::ClassDef { decorators, .. } => {
                                    *decorators = std::mem::take(&mut pending_decorators);
                                }
                                _ => {
                                    // If it's not a function or class def, decorators are invalid here
                                    pending_decorators.clear()
                                }
                            }
                            statements.push(stmt)
                        }
                    }
                }
                _ => {}
            }
            current_offset += child_len
        }
        Ok(statements)
    }

    /// Builds a suite (list of statements) from a green node.
    pub(crate) fn build_suite(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Vec<Statement>, OakError> {
        self.build_statement_list(node.children(), offset, source)
    }

    /// Builds an import name (alias) from a green node.
    pub(crate) fn build_import_name(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<ImportName, OakError> {
        let mut name = String::new();
        let mut asname = None;
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => {
                    let text = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                    if name.is_empty() {
                        name = text;
                    }
                    else {
                        asname = Some(text);
                    }
                }
                _ => {}
            }
            current_offset += child_len;
        }

        Ok(ImportName { name, asname })
    }

    /// Builds an exception handler from a green node.
    pub(crate) fn build_except_handler(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<ExceptHandler, OakError> {
        let mut type_ = None;
        let mut name = None;
        let mut body = Vec::new();
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Node(n) => {
                    if !n.kind.is_trivia() {
                        if n.kind == PythonElementType::Suite {
                            body = self.build_suite(n, current_offset, source)?;
                        }
                        else if type_.is_none() {
                            type_ = Some(self.build_expression(n, current_offset, source)?);
                        }
                    }
                }
                GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => {
                    name = Some(source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string());
                }
                _ => {}
            }
            current_offset += child_len;
        }
        Ok(ExceptHandler { type_, name, body })
    }

    /// Builds a `with` item from a green node.
    pub(crate) fn build_with_item(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<WithItem, OakError> {
        let mut context_expr = None;
        let mut optional_vars = None;
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            if let GreenTree::Node(n) = child {
                if !n.kind.is_trivia() {
                    let expr = self.build_expression(n, current_offset, source)?;
                    if context_expr.is_none() { context_expr = Some(expr) } else { optional_vars = Some(expr) }
                }
            }
            current_offset += child_len
        }
        Ok(WithItem { context_expr: context_expr.unwrap_or(Expression::Literal(Literal::None)), optional_vars })
    }
}
