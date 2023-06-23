use crate::{
    ast::*,
    language::LuaLanguage,
    lexer::{LuaLexer, token_type::LuaTokenType},
    parser::{LuaElementType, LuaParser},
};
use oak_core::{
    Builder, BuilderCache, Lexer, Parser, Source, TextEdit,
    builder::BuildOutput,
    parser::session::ParseSession,
    tree::{GreenNode, GreenTree},
};

/// Lua AST 构建器
#[derive(Clone)]
pub struct LuaBuilder<'config> {
    config: &'config LuaLanguage,
}

impl<'config> LuaBuilder<'config> {
    pub fn new(config: &'config LuaLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<LuaLanguage> for LuaBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<LuaLanguage>) -> BuildOutput<LuaLanguage> {
        let parser = LuaParser::new(self.config);
        let lexer = LuaLexer::new(self.config);

        let mut session = ParseSession::<LuaLanguage>::default();
        lexer.lex(source, edits, &mut session);
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => match self.build_root(green_tree.clone(), source) {
                Ok(ast_root) => oak_core::errors::OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                Err(build_error) => {
                    let mut diagnostics = parse_result.diagnostics;
                    diagnostics.push(build_error.clone());
                    oak_core::errors::OakDiagnostics { result: Err(build_error), diagnostics }
                }
            },
            Err(parse_error) => oak_core::errors::OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> LuaBuilder<'config> {
    fn build_root<S: Source + ?Sized>(&self, green_tree: GreenNode<LuaLanguage>, source: &S) -> Result<LuaRoot, oak_core::OakError> {
        let mut statements = Vec::new();
        let mut offset = 0;
        for child in green_tree.children() {
            match child {
                GreenTree::Node(node) => {
                    if let Some(stmt) = self.build_statement(node, source, offset)? {
                        statements.push(stmt)
                    }
                }
                GreenTree::Leaf(_) => {}
            }
            offset += child.len() as usize
        }
        Ok(LuaRoot { statements, span: (0..offset).into() })
    }

    fn build_statement<S: Source + ?Sized>(&self, node: &GreenNode<LuaLanguage>, source: &S, offset: usize) -> Result<Option<LuaStatement>, oak_core::OakError> {
        match node.kind {
            LuaElementType::LocalStatement => {
                let mut names = Vec::new();
                let mut values = Vec::new();
                let mut child_offset = offset;
                for child in node.children() {
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == LuaTokenType::Identifier => names.push(source.get_text_in(oak_core::Range { start: child_offset, end: child_offset + leaf.length as usize }).to_string()),
                        GreenTree::Node(child_node) => {
                            if let Some(expr) = self.build_expression(child_node, source, child_offset)? {
                                values.push(expr)
                            }
                        }
                        _ => {}
                    }
                    child_offset += child.len() as usize
                }
                Ok(Some(LuaStatement::Local(LuaLocalStatement { names, values })))
            }
            LuaElementType::AssignmentStatement => {
                let mut targets = Vec::new();
                let mut values = Vec::new();
                let mut child_offset = offset;
                let mut after_eq = false;
                for child in node.children() {
                    match child {
                        GreenTree::Node(child_node) => {
                            if let Some(expr) = self.build_expression(child_node, source, child_offset)? {
                                if after_eq { values.push(expr) } else { targets.push(expr) }
                            }
                        }
                        GreenTree::Leaf(leaf) if leaf.kind == LuaTokenType::Eq => after_eq = true,
                        _ => {}
                    }
                    child_offset += child.len() as usize
                }
                Ok(Some(LuaStatement::Assignment(LuaAssignmentStatement { targets, values })))
            }
            LuaElementType::ExpressionStatement => {
                let mut child_offset = offset;
                for child in node.children() {
                    if let GreenTree::Node(child_node) = child {
                        if let Some(expr) = self.build_expression(child_node, source, child_offset)? {
                            return Ok(Some(LuaStatement::Expression(expr)));
                        }
                    }
                    child_offset += child.len() as usize
                }
                Ok(None)
            }
            LuaElementType::ReturnStatement => {
                let mut values = Vec::new();
                let mut child_offset = offset;
                for child in node.children() {
                    if let GreenTree::Node(child_node) = child {
                        if let Some(expr) = self.build_expression(child_node, source, child_offset)? {
                            values.push(expr)
                        }
                    }
                    child_offset += child.len() as usize
                }
                Ok(Some(LuaStatement::Return(LuaReturnStatement { values })))
            }
            LuaElementType::BreakStatement => Ok(Some(LuaStatement::Break)),
            LuaElementType::GotoStatement => {
                let mut child_offset = offset;
                for child in node.children() {
                    if let GreenTree::Leaf(leaf) = child {
                        if leaf.kind == LuaTokenType::Identifier {
                            let text = source.get_text_in(oak_core::Range { start: child_offset, end: child_offset + leaf.length as usize });
                            return Ok(Some(LuaStatement::Goto(text.to_string())));
                        }
                    }
                    child_offset += child.len() as usize
                }
                Ok(None)
            }
            LuaElementType::LabelStatement => {
                let mut child_offset = offset;
                for child in node.children() {
                    if let GreenTree::Leaf(leaf) = child {
                        if leaf.kind == LuaTokenType::Identifier {
                            let text = source.get_text_in(oak_core::Range { start: child_offset, end: child_offset + leaf.length as usize });
                            return Ok(Some(LuaStatement::Label(text.to_string())));
                        }
                    }
                    child_offset += child.len() as usize
                }
                Ok(None)
            }
            LuaElementType::IfStatement => {
                let mut condition = None;
                let mut then_block = Vec::new();
                let mut else_ifs = Vec::new();
                let mut else_block = None;

                let mut child_offset = offset;
                let mut state = 0; // 0: if-cond, 1: then-block, 2: elseif-cond, 3: elseif-block, 4: else-block

                for child in node.children() {
                    match child {
                        GreenTree::Leaf(leaf) => match leaf.kind {
                            LuaTokenType::If => state = 0,
                            LuaTokenType::Then => {
                                if state == 0 {
                                    state = 1
                                }
                                else if state == 2 {
                                    state = 3
                                }
                            }
                            LuaTokenType::Elseif => state = 2,
                            LuaTokenType::Else => {
                                state = 4;
                                else_block = Some(Vec::new())
                            }
                            _ => {}
                        },
                        GreenTree::Node(child_node) => match state {
                            0 => {
                                if let Some(expr) = self.build_expression(child_node, source, child_offset)? {
                                    condition = Some(expr)
                                }
                            }
                            1 => {
                                if let Some(stmt) = self.build_statement(child_node, source, child_offset)? {
                                    then_block.push(stmt)
                                }
                            }
                            2 => {
                                if let Some(expr) = self.build_expression(child_node, source, child_offset)? {
                                    else_ifs.push((expr, Vec::new()))
                                }
                            }
                            3 => {
                                if let Some(stmt) = self.build_statement(child_node, source, child_offset)? {
                                    if let Some(last) = else_ifs.last_mut() {
                                        last.1.push(stmt)
                                    }
                                }
                            }
                            4 => {
                                if let Some(stmt) = self.build_statement(child_node, source, child_offset)? {
                                    if let Some(block) = &mut else_block {
                                        block.push(stmt)
                                    }
                                }
                            }
                            _ => {}
                        },
                    }
                    child_offset += child.len() as usize
                }

                if let Some(cond) = condition { Ok(Some(LuaStatement::If(LuaIfStatement { condition: cond, then_block, else_ifs, else_block }))) } else { Ok(None) }
            }
            LuaElementType::WhileStatement => {
                let mut condition = None;
                let mut block = Vec::new();
                let mut child_offset = offset;
                let mut in_block = false;

                for child in node.children() {
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == LuaTokenType::Do => in_block = true,
                        GreenTree::Node(child_node) => {
                            if in_block {
                                if let Some(stmt) = self.build_statement(child_node, source, child_offset)? {
                                    block.push(stmt)
                                }
                            }
                            else {
                                if let Some(expr) = self.build_expression(child_node, source, child_offset)? {
                                    condition = Some(expr)
                                }
                            }
                        }
                        _ => {}
                    }
                    child_offset += child.len() as usize
                }
                if let Some(cond) = condition { Ok(Some(LuaStatement::While(LuaWhileStatement { condition: cond, block }))) } else { Ok(None) }
            }
            LuaElementType::ForStatement => {
                let mut variables = Vec::new();
                let mut expressions = Vec::new();
                let mut block = Vec::new();
                let mut child_offset = offset;
                let mut in_block = false;
                let mut after_in = false;
                let mut after_eq = false;

                for child in node.children() {
                    match child {
                        GreenTree::Leaf(leaf) => match leaf.kind {
                            LuaTokenType::Identifier => variables.push(source.get_text_in(oak_core::Range { start: child_offset, end: child_offset + leaf.length as usize }).to_string()),
                            LuaTokenType::Eq => after_eq = true,
                            LuaTokenType::In => after_in = true,
                            LuaTokenType::Do => in_block = true,
                            _ => {}
                        },
                        GreenTree::Node(child_node) => {
                            if in_block {
                                if let Some(stmt) = self.build_statement(child_node, source, child_offset)? {
                                    block.push(stmt)
                                }
                            }
                            else {
                                if let Some(expr) = self.build_expression(child_node, source, child_offset)? {
                                    expressions.push(expr)
                                }
                            }
                        }
                    }
                    child_offset += child.len() as usize
                }

                if after_in {
                    Ok(Some(LuaStatement::For(LuaForStatement::Generic { variables, iterators: expressions, block })))
                }
                else if after_eq && !variables.is_empty() && expressions.len() >= 2 {
                    Ok(Some(LuaStatement::For(LuaForStatement::Numeric { variable: variables[0].clone(), start: expressions[0].clone(), end: expressions[1].clone(), step: expressions.get(2).cloned(), block })))
                }
                else {
                    Ok(None)
                }
            }
            LuaElementType::RepeatStatement => {
                let mut block = Vec::new();
                let mut condition = None;
                let mut child_offset = offset;
                let mut after_until = false;

                for child in node.children() {
                    match child {
                        GreenTree::Leaf(leaf) if leaf.kind == LuaTokenType::Until => after_until = true,
                        GreenTree::Node(child_node) => {
                            if after_until {
                                if let Some(expr) = self.build_expression(child_node, source, child_offset)? {
                                    condition = Some(expr)
                                }
                            }
                            else {
                                if let Some(stmt) = self.build_statement(child_node, source, child_offset)? {
                                    block.push(stmt)
                                }
                            }
                        }
                        _ => {}
                    }
                    child_offset += child.len() as usize
                }
                if let Some(cond) = condition { Ok(Some(LuaStatement::Repeat(LuaRepeatStatement { block, condition: cond }))) } else { Ok(None) }
            }
            LuaElementType::DoStatement => {
                let mut block = Vec::new();
                let mut child_offset = offset;
                for child in node.children() {
                    if let GreenTree::Node(child_node) = child {
                        if let Some(stmt) = self.build_statement(child_node, source, child_offset)? {
                            block.push(stmt)
                        }
                    }
                    child_offset += child.len() as usize
                }
                Ok(Some(LuaStatement::Do(block)))
            }
            LuaElementType::FunctionDeclaration => {
                let mut name = Vec::new();
                let mut receiver = None;
                let mut parameters = Vec::new();
                let mut is_vararg = false;
                let mut block = Vec::new();

                let mut child_offset = offset;
                let mut state = 0; // 0: name, 1: parameters, 2: block

                for child in node.children() {
                    match child {
                        GreenTree::Leaf(leaf) => {
                            match leaf.kind {
                                LuaTokenType::Identifier => {
                                    let text = source.get_text_in(oak_core::Range { start: child_offset, end: child_offset + leaf.length as usize }).to_string();
                                    if state == 0 {
                                        name.push(text)
                                    }
                                    else if state == 1 {
                                        parameters.push(text)
                                    }
                                }
                                LuaTokenType::Colon => {
                                    // Receiver follows - in Lua function name: method,
                                    // the part before colon is name parts, part after colon is method name
                                    // But usually it's obj:method, where obj is name parts.
                                    if state == 0 {
                                        // The current name parts we have are the receiver parts.
                                        // We'll move them to receiver and start fresh for method name?
                                        // Actually LuaFunctionStatement has name: Vec<String> and receiver: Option<String>.
                                        // Usually it's function a.b:c() -> name=["a", "b"], receiver=Some("c")
                                        if !name.is_empty() {
                                            receiver = name.pop()
                                        }
                                    }
                                }
                                LuaTokenType::LeftParen => state = 1,
                                LuaTokenType::RightParen => state = 2,
                                LuaTokenType::DotDotDot => is_vararg = true,
                                _ => {}
                            }
                        }
                        GreenTree::Node(child_node) => {
                            if state == 2 {
                                if let Some(stmt) = self.build_statement(child_node, source, child_offset)? {
                                    block.push(stmt)
                                }
                            }
                            else if child_node.kind == LuaElementType::FunctionName {
                                // Recursive name building if nested
                                let mut name_offset = child_offset;
                                for name_child in child_node.children() {
                                    match name_child {
                                        GreenTree::Leaf(leaf) => {
                                            if leaf.kind == LuaTokenType::Identifier {
                                                name.push(source.get_text_in(oak_core::Range { start: name_offset, end: name_offset + leaf.length as usize }).to_string())
                                            }
                                            else if leaf.kind == LuaTokenType::Colon {
                                                if !name.is_empty() {
                                                    receiver = name.pop()
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                    name_offset += name_child.len() as usize
                                }
                            }
                        }
                    }
                    child_offset += child.len() as usize
                }

                Ok(Some(LuaStatement::Function(LuaFunctionStatement { name, receiver, parameters, is_vararg, block })))
            }
            _ => Ok(None),
        }
    }

    fn build_expression<S: Source + ?Sized>(&self, node: &GreenNode<LuaLanguage>, source: &S, offset: usize) -> Result<Option<LuaExpression>, oak_core::OakError> {
        match node.kind {
            LuaElementType::LiteralExpression => {
                for child in node.children() {
                    if let GreenTree::Leaf(leaf) = child {
                        match leaf.kind {
                            LuaTokenType::Number => {
                                let text = source.get_text_in(oak_core::Range { start: offset, end: offset + leaf.length as usize });
                                if let Ok(val) = text.parse::<f64>() {
                                    return Ok(Some(LuaExpression::Number(val)));
                                }
                            }
                            LuaTokenType::String => {
                                let text = source.get_text_in(oak_core::Range { start: offset, end: offset + leaf.length as usize });
                                // Simple quote removal
                                let s = if text.len() >= 2 { &text[1..text.len() - 1] } else { &text };
                                return Ok(Some(LuaExpression::String(s.to_string())));
                            }
                            LuaTokenType::True => return Ok(Some(LuaExpression::Boolean(true))),
                            LuaTokenType::False => return Ok(Some(LuaExpression::Boolean(false))),
                            LuaTokenType::Nil => return Ok(Some(LuaExpression::Nil)),
                            _ => {}
                        }
                    }
                }
                Ok(None)
            }
            LuaElementType::IdentifierExpression => {
                for child in node.children() {
                    if let GreenTree::Leaf(leaf) = child {
                        if leaf.kind == LuaTokenType::Identifier {
                            let name = source.get_text_in(oak_core::Range { start: offset, end: offset + leaf.length as usize }).to_string();
                            return Ok(Some(LuaExpression::Identifier(name)));
                        }
                    }
                }
                Ok(None)
            }
            LuaElementType::BinaryExpression => {
                let mut left = None;
                let mut op = String::new();
                let mut right = None;
                let mut child_offset = offset;

                for child in node.children() {
                    match child {
                        GreenTree::Node(child_node) => {
                            if let Some(expr) = self.build_expression(child_node, source, child_offset)? {
                                if left.is_none() { left = Some(expr) } else { right = Some(expr) }
                            }
                        }
                        GreenTree::Leaf(leaf) => {
                            // Binary operators
                            match leaf.kind {
                                LuaTokenType::Plus
                                | LuaTokenType::Minus
                                | LuaTokenType::Star
                                | LuaTokenType::Slash
                                | LuaTokenType::Percent
                                | LuaTokenType::Caret
                                | LuaTokenType::Lt
                                | LuaTokenType::LtEq
                                | LuaTokenType::Gt
                                | LuaTokenType::GtEq
                                | LuaTokenType::EqEq
                                | LuaTokenType::TildeEq
                                | LuaTokenType::Ampersand
                                | LuaTokenType::Pipe
                                | LuaTokenType::Tilde
                                | LuaTokenType::LtLt
                                | LuaTokenType::GtGt
                                | LuaTokenType::SlashSlash
                                | LuaTokenType::And
                                | LuaTokenType::Or
                                | LuaTokenType::DotDot => op = source.get_text_in(oak_core::Range { start: child_offset, end: child_offset + leaf.length as usize }).to_string(),
                                _ => {}
                            }
                        }
                    }
                    child_offset += child.len() as usize
                }

                if let (Some(l), Some(r)) = (left, right) { Ok(Some(LuaExpression::Binary(Box::new(LuaBinaryExpression { left: l, op, right: r })))) } else { Ok(None) }
            }
            LuaElementType::UnaryExpression => {
                let mut op = String::new();
                let mut operand = None;
                let mut child_offset = offset;

                for child in node.children() {
                    match child {
                        GreenTree::Node(child_node) => operand = self.build_expression(child_node, source, child_offset)?,
                        GreenTree::Leaf(leaf) => match leaf.kind {
                            LuaTokenType::Minus | LuaTokenType::Not | LuaTokenType::Hash | LuaTokenType::Tilde => op = source.get_text_in(oak_core::Range { start: child_offset, end: child_offset + leaf.length as usize }).to_string(),
                            _ => {}
                        },
                    }
                    child_offset += child.len() as usize
                }

                if let Some(o) = operand { Ok(Some(LuaExpression::Unary(Box::new(LuaUnaryExpression { op, operand: o })))) } else { Ok(None) }
            }
            LuaElementType::CallExpression => {
                let mut function = None;
                let mut arguments = Vec::new();
                let mut child_offset = offset;

                for child in node.children() {
                    if let GreenTree::Node(child_node) = child {
                        if let Some(expr) = self.build_expression(child_node, source, child_offset)? {
                            if function.is_none() { function = Some(expr) } else { arguments.push(expr) }
                        }
                    }
                    else if let GreenTree::Leaf(leaf) = child {
                        if leaf.kind == LuaTokenType::String {
                            let text = source.get_text_in(oak_core::Range { start: child_offset, end: child_offset + leaf.length as usize });
                            let s = if text.len() >= 2 { &text[1..text.len() - 1] } else { &text };
                            arguments.push(LuaExpression::String(s.to_string()))
                        }
                    }
                    child_offset += child.len() as usize
                }

                if let Some(f) = function { Ok(Some(LuaExpression::Call(Box::new(LuaCallExpression { function: f, arguments })))) } else { Ok(None) }
            }
            LuaElementType::TableConstructorExpression => {
                let mut fields = Vec::new();
                let mut child_offset = offset;
                for child in node.children() {
                    if let GreenTree::Node(child_node) = child {
                        if child_node.kind == LuaElementType::TableField {
                            let mut key = None;
                            let mut value = None;
                            let mut name = None;
                            let mut field_offset = child_offset;
                            let mut after_eq = false;

                            for field_child in child_node.children() {
                                match field_child {
                                    GreenTree::Leaf(leaf) => {
                                        if leaf.kind == LuaTokenType::Identifier && !after_eq {
                                            name = Some(source.get_text_in(oak_core::Range { start: field_offset, end: field_offset + leaf.length as usize }).to_string())
                                        }
                                        else if leaf.kind == LuaTokenType::Eq {
                                            after_eq = true
                                        }
                                    }
                                    GreenTree::Node(field_node) => {
                                        if let Some(expr) = self.build_expression(field_node, source, field_offset)? {
                                            if key.is_none() && !after_eq && name.is_none() { key = Some(expr) } else { value = Some(expr) }
                                        }
                                    }
                                }
                                field_offset += field_child.len() as usize
                            }

                            if let Some(v) = value {
                                if let Some(k) = key {
                                    fields.push(LuaTableField::Keyed { key: k, value: v })
                                }
                                else if let Some(n) = name {
                                    fields.push(LuaTableField::Named { name: n, value: v })
                                }
                                else {
                                    fields.push(LuaTableField::List { value: v })
                                }
                            }
                            else if let Some(k) = key {
                                fields.push(LuaTableField::List { value: k })
                            }
                        }
                    }
                    child_offset += child.len() as usize
                }
                Ok(Some(LuaExpression::Table(LuaTableConstructor { fields })))
            }
            LuaElementType::FunctionExpression => {
                let mut parameters = Vec::new();
                let mut is_vararg = false;
                let mut block = Vec::new();
                let mut child_offset = offset;
                let mut state = 0; // 0: params, 1: block

                for child in node.children() {
                    match child {
                        GreenTree::Leaf(leaf) => match leaf.kind {
                            LuaTokenType::Identifier if state == 0 => parameters.push(source.get_text_in(oak_core::Range { start: child_offset, end: child_offset + leaf.length as usize }).to_string()),
                            LuaTokenType::DotDotDot => is_vararg = true,
                            LuaTokenType::RightParen => state = 1,
                            _ => {}
                        },
                        GreenTree::Node(child_node) => {
                            if state == 1 {
                                if let Some(stmt) = self.build_statement(child_node, source, child_offset)? {
                                    block.push(stmt)
                                }
                            }
                        }
                    }
                    child_offset += child.len() as usize
                }
                Ok(Some(LuaExpression::Function(LuaFunctionExpression { parameters, is_vararg, block })))
            }
            LuaElementType::MemberExpression => {
                let mut table = None;
                let mut member = String::new();
                let mut is_method = false;
                let mut child_offset = offset;

                for child in node.children() {
                    match child {
                        GreenTree::Node(child_node) => {
                            if table.is_none() {
                                table = self.build_expression(child_node, source, child_offset)?
                            }
                        }
                        GreenTree::Leaf(leaf) => {
                            if leaf.kind == LuaTokenType::Identifier {
                                member = source.get_text_in(oak_core::Range { start: child_offset, end: child_offset + leaf.length as usize }).to_string()
                            }
                            else if leaf.kind == LuaTokenType::Colon {
                                is_method = true
                            }
                        }
                    }
                    child_offset += child.len() as usize
                }

                if let Some(t) = table { Ok(Some(LuaExpression::Member(Box::new(LuaMemberExpression { table: t, member, is_method })))) } else { Ok(None) }
            }
            LuaElementType::IndexExpression => {
                let mut table = None;
                let mut index = None;
                let mut child_offset = offset;

                for child in node.children() {
                    if let GreenTree::Node(child_node) = child {
                        if table.is_none() { table = self.build_expression(child_node, source, child_offset)? } else { index = self.build_expression(child_node, source, child_offset)? }
                    }
                    child_offset += child.len() as usize
                }

                if let (Some(t), Some(i)) = (table, index) { Ok(Some(LuaExpression::Index(Box::new(LuaIndexExpression { table: t, index: i })))) } else { Ok(None) }
            }
            LuaElementType::VarargExpression => Ok(Some(LuaExpression::Vararg)),
            LuaElementType::ParenthesizedExpression => {
                let mut child_offset = offset;
                for child in node.children() {
                    if let GreenTree::Node(child_node) = child {
                        if let Some(expr) = self.build_expression(child_node, source, child_offset)? {
                            return Ok(Some(expr));
                        }
                    }
                    child_offset += child.len() as usize
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}
