use crate::{
    ast::{BinaryOperator, CompareOperator, Expression, Keyword, Literal, Parameter},
    language::PythonLanguage,
    lexer::token_type::PythonTokenType,
    parser::PythonElementType,
};
use oak_core::{GreenNode, GreenTree, OakError, Source, SourceText};

impl<'config> super::PythonBuilder<'config> {
    /// Builds an expression from a green node.
    pub(crate) fn build_expression(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Expression, OakError> {
        match node.kind {
            PythonElementType::Constant => {
                let mut current_offset = offset;
                for child in node.children() {
                    if let GreenTree::Leaf(leaf) = child {
                        if !leaf.kind.is_trivia() {
                            let text = source.get_text_in((current_offset..current_offset + leaf.length as usize).into());
                            if let Ok(val) = text.parse::<i64>() {
                                return Ok(Expression::Literal(Literal::Integer(val)));
                            }
                            else if let Ok(val) = text.parse::<f64>() {
                                return Ok(Expression::Literal(Literal::Float(val)));
                            }
                            else if text == "True" {
                                return Ok(Expression::Literal(Literal::Boolean(true)));
                            }
                            else if text == "False" {
                                return Ok(Expression::Literal(Literal::Boolean(false)));
                            }
                            else if text == "None" {
                                return Ok(Expression::Literal(Literal::None));
                            }
                            else {
                                let s = text.to_string();
                                if s.starts_with('b') || s.starts_with('B') {
                                    let content = if (s.starts_with("b\"") || s.starts_with("B\"") || s.starts_with("b'")) && (s.ends_with('"') || s.ends_with('\'')) { &s[2..s.len() - 1] } else { &s[1..] };
                                    return Ok(Expression::Literal(Literal::Bytes(content.as_bytes().to_vec())));
                                }
                                let mut s = s;
                                if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
                                    s = s[1..s.len() - 1].to_string()
                                }
                                return Ok(Expression::Literal(Literal::String(s)));
                            }
                        }
                    }
                    current_offset += child.len() as usize
                }
                Ok(Expression::Name("invalid_constant".to_string()))
            }
            PythonElementType::Name => {
                let mut current_offset = offset;
                for child in node.children() {
                    if let GreenTree::Leaf(leaf) = child {
                        if !leaf.kind.is_trivia() {
                            let text = source.get_text_in((current_offset..current_offset + leaf.length as usize).into());
                            return Ok(Expression::Name(text.to_string()));
                        }
                    }
                    current_offset += child.len() as usize;
                }
                Ok(Expression::Name("invalid_name".to_string()))
            }
            PythonElementType::BinOp => {
                let mut left = None;
                let mut operator = None;
                let mut right = None;
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_trivia() {
                                let expr = self.build_expression(n, current_offset, source)?;
                                if left.is_none() {
                                    left = Some(Box::new(expr));
                                }
                                else if right.is_none() {
                                    right = Some(Box::new(expr));
                                }
                            }
                        }
                        GreenTree::Leaf(leaf) => {
                            if !leaf.kind.is_trivia() {
                                let op = match leaf.kind {
                                    PythonTokenType::Plus => Some(BinaryOperator::Add),
                                    PythonTokenType::Minus => Some(BinaryOperator::Sub),
                                    PythonTokenType::Star => Some(BinaryOperator::Mult),
                                    PythonTokenType::Slash => Some(BinaryOperator::Div),
                                    PythonTokenType::Percent => Some(BinaryOperator::Mod),
                                    PythonTokenType::DoubleStar => Some(BinaryOperator::Pow),
                                    PythonTokenType::LeftShift => Some(BinaryOperator::LShift),
                                    PythonTokenType::RightShift => Some(BinaryOperator::RShift),
                                    PythonTokenType::Pipe => Some(BinaryOperator::BitOr),
                                    PythonTokenType::Caret => Some(BinaryOperator::BitXor),
                                    PythonTokenType::Ampersand => Some(BinaryOperator::BitAnd),
                                    PythonTokenType::DoubleSlash => Some(BinaryOperator::FloorDiv),
                                    _ => None,
                                };
                                if let Some(op) = op {
                                    operator = Some(op);
                                }
                            }
                        }
                    }
                    current_offset += child_len;
                }

                if let (Some(l), Some(op), Some(r)) = (left, operator, right) { Ok(Expression::BinaryOp { left: l, operator: op, right: r }) } else { Ok(Expression::Name(format!("invalid_binop_at_{}", offset))) }
            }
            PythonElementType::Compare => {
                let mut left = None;
                let mut ops = Vec::new();
                let mut comparators = Vec::new();
                let mut pending_op: Option<CompareOperator> = None;
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_trivia() {
                                let expr = self.build_expression(n, current_offset, source)?;
                                if left.is_none() {
                                    left = Some(Box::new(expr));
                                }
                                else {
                                    if let Some(op) = pending_op.take() {
                                        ops.push(op);
                                    }
                                    comparators.push(expr);
                                }
                            }
                        }
                        GreenTree::Leaf(leaf) => {
                            if !leaf.kind.is_trivia() {
                                pending_op = match leaf.kind {
                                    PythonTokenType::Equal => Some(CompareOperator::Eq),
                                    PythonTokenType::NotEqual => Some(CompareOperator::NotEq),
                                    PythonTokenType::Less => Some(CompareOperator::Lt),
                                    PythonTokenType::LessEqual => Some(CompareOperator::LtE),
                                    PythonTokenType::Greater => Some(CompareOperator::Gt),
                                    PythonTokenType::GreaterEqual => Some(CompareOperator::GtE),
                                    PythonTokenType::IsKeyword => Some(CompareOperator::Is),
                                    PythonTokenType::InKeyword => Some(CompareOperator::In),
                                    _ => pending_op,
                                };
                            }
                        }
                    }
                    current_offset += child_len;
                }

                if let Some(l) = left { Ok(Expression::Compare { left: l, ops, comparators }) } else { Ok(Expression::Name(format!("invalid_compare_at_{}", offset))) }
            }
            PythonElementType::Call => {
                let mut func = None;
                let mut args = Vec::new();
                let mut keywords = Vec::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if func.is_none() {
                                func = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else if n.kind == PythonElementType::Keyword {
                                keywords.push(self.build_keyword(n, current_offset, source)?);
                            }
                            else if n.kind == PythonElementType::Starred {
                                let expr = self.build_starred(n, current_offset, source)?;
                                args.push(expr);
                            }
                            else {
                                args.push(self.build_expression(n, current_offset, source)?);
                            }
                        }
                    }
                    current_offset += child_len;
                }

                if let Some(f) = func { Ok(Expression::Call { func: f, args, keywords }) } else { Ok(Expression::Name("invalid_call".to_string())) }
            }
            PythonElementType::Attribute => {
                let mut value = None;
                let mut attr = String::new();
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_trivia() {
                                value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                        }
                        GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => {
                            attr = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }

                if let Some(v) = value { Ok(Expression::Attribute { value: v, attr }) } else { Ok(Expression::Name("invalid_attribute".to_string())) }
            }
            PythonElementType::Expr => {
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            return self.build_expression(n, current_offset, source);
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Name("invalid_expr".to_string()))
            }
            PythonElementType::Tuple => {
                let mut exprs = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            exprs.push(self.build_expression(n, current_offset, source)?);
                        }
                    }
                    current_offset += child_len;
                }
                if exprs.len() == 1 { Ok(exprs.remove(0)) } else { Ok(Expression::Tuple { elts: exprs }) }
            }
            PythonElementType::List => {
                let mut exprs = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            exprs.push(self.build_expression(n, current_offset, source)?);
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::List { elts: exprs })
            }
            PythonElementType::Subscript => {
                let mut value = None;
                let mut slice = None;
                let mut current_offset = offset;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if value.is_none() {
                                value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else {
                                slice = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                        }
                    }
                    current_offset += child_len;
                }

                if let (Some(v), Some(s)) = (value, slice) { Ok(Expression::Subscript { value: v, slice: s }) } else { Ok(Expression::Name("invalid_subscript".to_string())) }
            }
            PythonElementType::JoinedStr => {
                let mut current_offset = offset;
                let mut values = Vec::new();
                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Node(n) => {
                            if !n.kind.is_trivia() {
                                values.push(self.build_expression(n, current_offset, source)?);
                            }
                        }
                        GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::FString => {
                            let text = source.get_text_in((current_offset..current_offset + leaf.length as usize).into());
                            values.extend(self.parse_fstring_text(&text)?);
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Expression::JoinedStr { values })
            }
            PythonElementType::FormattedValue => {
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            let value = Box::new(self.build_expression(n, current_offset, source)?);
                            return Ok(Expression::FormattedValue { value, conversion: 0, format_spec: None });
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Name("invalid_formatted_value".to_string()))
            }
            PythonElementType::Yield => {
                let mut value = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Yield(value))
            }
            PythonElementType::YieldFrom => {
                let mut value = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::YieldFrom(value.unwrap_or(Box::new(Expression::Name("invalid_yield_from".to_string())))))
            }
            PythonElementType::Starred => self.build_starred(node, offset, source),
            PythonElementType::Await => {
                let mut value = None;
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Await(value.unwrap_or(Box::new(Expression::Name("invalid_await".to_string())))))
            }
            PythonElementType::Dict => {
                let mut keys = Vec::new();
                let mut values = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if n.kind == PythonElementType::Starred {
                                let expr = self.build_starred(n, current_offset, source)?;
                                keys.push(None);
                                values.push(expr);
                            }
                            else if keys.len() == values.len() {
                                // We are expecting a key
                                let expr = self.build_expression(n, current_offset, source)?;
                                keys.push(Some(expr));
                            }
                            else {
                                // We are expecting a value
                                let expr = self.build_expression(n, current_offset, source)?;
                                values.push(expr);
                            }
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Dict { keys, values })
            }
            PythonElementType::Set => {
                let mut elts = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            elts.push(self.build_expression(n, current_offset, source)?);
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Set { elts })
            }
            PythonElementType::ListComp => {
                let mut elt = None;
                let mut generators = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if elt.is_none() {
                                elt = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else if n.kind == PythonElementType::Comprehension {
                                generators.push(self.build_comprehension(n, current_offset, source)?);
                            }
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::ListComp { elt: elt.unwrap_or(Box::new(Expression::Name("invalid_elt".to_string()))), generators })
            }
            PythonElementType::SetComp => {
                let mut elt = None;
                let mut generators = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if elt.is_none() {
                                elt = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else if n.kind == PythonElementType::Comprehension {
                                generators.push(self.build_comprehension(n, current_offset, source)?);
                            }
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::SetComp { elt: elt.unwrap_or(Box::new(Expression::Name("invalid_elt".to_string()))), generators })
            }
            PythonElementType::DictComp => {
                let mut key = None;
                let mut value = None;
                let mut generators = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if key.is_none() {
                                key = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else if value.is_none() {
                                value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else if n.kind == PythonElementType::Comprehension {
                                generators.push(self.build_comprehension(n, current_offset, source)?);
                            }
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::DictComp { key: key.unwrap_or(Box::new(Expression::Name("invalid_key".to_string()))), value: value.unwrap_or(Box::new(Expression::Name("invalid_value".to_string()))), generators })
            }
            PythonElementType::GeneratorExp => {
                let mut elt = None;
                let mut generators = Vec::new();
                let mut current_offset = offset;
                for child in node.children() {
                    let child_len = child.len() as usize;
                    if let GreenTree::Node(n) = child {
                        if !n.kind.is_trivia() {
                            if elt.is_none() {
                                elt = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            }
                            else if n.kind == PythonElementType::Comprehension {
                                generators.push(self.build_comprehension(n, current_offset, source)?);
                            }
                        }
                    }
                    current_offset += child_len;
                }
                Ok(Expression::GeneratorExp { elt: elt.unwrap_or(Box::new(Expression::Name("invalid_elt".to_string()))), generators })
            }
            PythonElementType::Slice => {
                let mut lower = None;
                let mut upper = None;
                let mut step = None;
                let mut current_offset = offset;
                let mut colon_count = 0;

                for child in node.children() {
                    let child_len = child.len() as usize;
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Colon => {
                            colon_count += 1;
                        }
                        GreenTree::Node(n) if !n.kind.is_trivia() => {
                            let expr = Some(Box::new(self.build_expression(n, current_offset, source)?));
                            if colon_count == 0 {
                                lower = expr;
                            }
                            else if colon_count == 1 {
                                upper = expr;
                            }
                            else {
                                step = expr;
                            }
                        }
                        _ => {}
                    }
                    current_offset += child_len;
                }
                Ok(Expression::Slice { lower, upper, step })
            }
            _ => Ok(Expression::Name(format!("unsupported_kind_{:?}", node.kind))),
        }
    }

    /// Builds a keyword argument from a green node.
    pub(crate) fn build_keyword(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Keyword, OakError> {
        let mut arg = None;
        let mut value = None;
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Identifier => {
                    arg = Some(source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string());
                }
                GreenTree::Node(n) if !n.kind.is_trivia() => {
                    value = Some(self.build_expression(n, current_offset, source)?);
                }
                _ => {}
            }
            current_offset += child_len;
        }

        if let Some(v) = value { Ok(Keyword { arg, value: v }) } else { Err(OakError::custom_error("Invalid keyword".to_string())) }
    }

    /// Builds a starred expression from a green node.
    pub(crate) fn build_starred(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Expression, OakError> {
        let mut value = None;
        let mut is_double = false;
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::Star => {
                    is_double = false;
                }
                GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::DoubleStar => {
                    is_double = true;
                }
                GreenTree::Node(n) if !n.kind.is_trivia() => {
                    value = Some(Box::new(self.build_expression(n, current_offset, source)?));
                }
                _ => {}
            }
            current_offset += child_len;
        }

        if let Some(v) = value { Ok(Expression::Starred { value: v, is_double }) } else { Err(OakError::custom_error("Invalid starred expression".to_string())) }
    }

    /// Builds a comprehension from a green node.
    pub(crate) fn build_comprehension(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<crate::ast::Comprehension, OakError> {
        let mut target = None;
        let mut iter = None;
        let mut ifs = Vec::new();
        let mut is_async = false;
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Leaf(leaf) if leaf.kind == PythonTokenType::AsyncKeyword => is_async = true,
                GreenTree::Node(n) if !n.kind.is_trivia() && n.kind != PythonElementType::ForKeyword && n.kind != PythonElementType::InKeyword && n.kind != PythonElementType::IfKeyword => {
                    let expr = self.build_expression(n, current_offset, source)?;
                    if target.is_none() {
                        target = Some(expr)
                    }
                    else if iter.is_none() {
                        iter = Some(expr)
                    }
                    else {
                        ifs.push(expr)
                    }
                }
                _ => {}
            }
            current_offset += child_len
        }

        Ok(crate::ast::Comprehension { target: target.unwrap_or(Expression::Name("invalid_target".to_string())), iter: iter.unwrap_or(Expression::Name("invalid_iter".to_string())), ifs, is_async })
    }

    /// Builds a list of parameters from a green node.
    pub(crate) fn build_parameters(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Vec<Parameter>, OakError> {
        let mut parameters = Vec::new();
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            if let GreenTree::Node(n) = child {
                if n.kind == PythonElementType::Arg {
                    parameters.push(self.build_parameter(n, current_offset, source)?);
                }
            }
            current_offset += child_len;
        }
        Ok(parameters)
    }

    /// Builds a single parameter from a green node.
    pub(crate) fn build_parameter(&self, node: &GreenNode<PythonLanguage>, offset: usize, source: &SourceText) -> Result<Parameter, OakError> {
        let mut name = String::new();
        let mut default = None;
        let mut is_vararg = false;
        let mut is_kwarg = false;
        let mut current_offset = offset;

        for child in node.children() {
            let child_len = child.len() as usize;
            match child {
                GreenTree::Leaf(leaf) => {
                    if leaf.kind == PythonTokenType::Identifier {
                        name = source.get_text_in((current_offset..current_offset + leaf.length as usize).into()).trim().to_string();
                    }
                    else if leaf.kind == PythonTokenType::Star {
                        is_vararg = true;
                    }
                    else if leaf.kind == PythonTokenType::DoubleStar {
                        is_kwarg = true;
                    }
                }
                GreenTree::Node(n) => {
                    if !n.kind.is_trivia() {
                        default = Some(self.build_expression(n, current_offset, source)?);
                    }
                }
            }
            current_offset += child_len;
        }

        Ok(Parameter { name, annotation: None, default, is_vararg, is_kwarg })
    }
}
