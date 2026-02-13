use crate::{SqlElementType, SqlLanguage, SqlParser, ast, ast::*, lexer::token_type::SqlTokenType};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, TokenType, builder::BuildOutput, source::Source};

#[derive(Clone)]
pub struct SqlBuilder<'config> {
    config: &'config SqlLanguage,
}

impl<'config> SqlBuilder<'config> {
    pub fn new(config: &'config SqlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<SqlLanguage> for SqlBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<SqlLanguage>) -> BuildOutput<SqlLanguage> {
        let parser = SqlParser::new(self.config);
        let mut parse_cache = oak_core::parser::session::ParseSession::<SqlLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

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

impl<'config> SqlBuilder<'config> {
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, SqlLanguage>, source: &SourceText) -> Result<SqlRoot, OakError> {
        let root_node = RedNode::new(green_tree, 0);
        let mut statements = Vec::new();

        for child in root_node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    SqlElementType::SelectStatement => statements.push(SqlStatement::Select(self.build_select_statement(n, source)?)),
                    SqlElementType::InsertStatement => statements.push(SqlStatement::Insert(self.build_insert_statement(n, source)?)),
                    SqlElementType::UpdateStatement => statements.push(SqlStatement::Update(self.build_update_statement(n, source)?)),
                    SqlElementType::DeleteStatement => statements.push(SqlStatement::Delete(self.build_delete_statement(n, source)?)),
                    SqlElementType::CreateStatement => statements.push(SqlStatement::Create(self.build_create_statement(n, source)?)),
                    SqlElementType::DropStatement => statements.push(SqlStatement::Drop(self.build_drop_statement(n, source)?)),
                    SqlElementType::AlterStatement => statements.push(SqlStatement::Alter(self.build_alter_statement(n, source)?)),
                    _ => {}
                }
            }
        }

        Ok(SqlRoot { statements, span: root_node.span() })
    }

    fn build_select_statement<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<SelectStatement, OakError> {
        let mut items = Vec::new();
        let mut from = None;
        let mut joins = Vec::new();
        let mut selection = None;
        let mut group_by = None;
        let mut having = None;
        let mut order_by = None;
        let mut limit = None;

        let mut where_found = false;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    SqlElementType::SelectItem => items.push(self.build_select_item(n, source)?),
                    SqlElementType::TableName => from = Some(self.build_table_name(n, source)?),
                    SqlElementType::JoinClause => joins.push(self.build_join_clause(n, source)?),
                    SqlElementType::GroupByClause => group_by = Some(self.build_group_by_clause(n, source)?),
                    SqlElementType::HavingClause => having = Some(self.build_having_clause(n, source)?),
                    SqlElementType::OrderByClause => order_by = Some(self.build_order_by_clause(n, source)?),
                    SqlElementType::LimitClause => limit = Some(self.build_limit_clause(n, source)?),
                    SqlElementType::Expression => {
                        if where_found && selection.is_none() {
                            selection = Some(self.build_expression(n, source)?);
                        }
                    }
                    _ => {}
                },
                RedTree::Leaf(t) => {
                    if t.kind == SqlTokenType::Where {
                        where_found = true;
                    }
                }
            }
        }

        Ok(SelectStatement { items, from, joins, selection, group_by, having, order_by, limit, span: node.span() })
    }

    fn build_select_item<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<SelectItem, OakError> {
        let mut expr = None;
        let mut alias = None;
        let mut is_star = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    SqlTokenType::Star => is_star = true,
                    SqlTokenType::Identifier_ => {
                        if expr.is_none() {
                            expr = Some(Expression::Identifier(Identifier { name: self.get_text(t.span.clone(), source), span: t.span.clone() }));
                        }
                        else {
                            alias = Some(Identifier { name: self.get_text(t.span.clone(), source), span: t.span.clone() });
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    SqlElementType::Expression => expr = Some(self.build_expression(n, source)?),
                    SqlElementType::Identifier => {
                        if expr.is_none() {
                            expr = Some(Expression::Identifier(self.build_identifier(n, source)?));
                        }
                        else {
                            alias = Some(self.build_identifier(n, source)?);
                        }
                    }
                    SqlElementType::Alias => alias = Some(self.build_identifier(n, source)?),
                    _ => {}
                },
            }
        }

        if is_star { Ok(SelectItem::Star { span: node.span() }) } else { Ok(SelectItem::Expression { expr: expr.ok_or_else(|| OakError::custom_error("Missing expression in select item"))?, alias, span: node.span() }) }
    }

    fn build_insert_statement<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<InsertStatement, OakError> {
        let mut table_name = None;
        let mut columns = Vec::new();
        let mut values = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    SqlElementType::TableName => table_name = Some(self.build_table_name(n, source)?),
                    SqlElementType::ColumnName => {
                        // In INSERT, ColumnName might contain Identifier node or Identifier_ token
                        for sub in n.children() {
                            match sub {
                                RedTree::Node(sn) if sn.green.kind == SqlElementType::Identifier => {
                                    columns.push(self.build_identifier(sn, source)?);
                                }
                                RedTree::Leaf(st) if st.kind == SqlTokenType::Identifier_ => {
                                    columns.push(Identifier { name: self.get_text(st.span.clone(), source), span: st.span.clone() });
                                }
                                _ => {}
                            }
                        }
                    }
                    SqlElementType::ValueList => {
                        self.collect_expressions(n, source, &mut values)?;
                    }
                    _ => {}
                }
            }
        }

        Ok(InsertStatement { table_name: table_name.ok_or_else(|| OakError::custom_error("Missing table name in INSERT"))?, columns, values, span: node.span() })
    }

    fn collect_expressions<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText, out: &mut Vec<Expression>) -> Result<(), OakError> {
        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    SqlElementType::Expression => out.push(self.build_expression(n, source)?),
                    SqlElementType::Identifier => out.push(Expression::Identifier(self.build_identifier(n, source)?)),
                    SqlElementType::ValueList => self.collect_expressions(n, source, out)?,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn build_update_statement<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<UpdateStatement, OakError> {
        let mut table_name: Option<ast::TableName> = None;
        let mut assignments = Vec::new();
        let mut selection = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    SqlTokenType::Set => {}
                    SqlTokenType::Identifier_ => {
                        if table_name.is_none() {
                            let ident = Identifier { name: self.get_text(t.span.clone(), source), span: t.span.clone() };
                            table_name = Some(ast::TableName { name: ident, span: t.span.clone() });
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) => {
                    match n.green.kind {
                        SqlElementType::TableName => table_name = Some(self.build_table_name(n, source)?),
                        SqlElementType::Assignment => assignments.push(self.build_assignment(n, source)?),
                        SqlElementType::Expression => selection = Some(self.build_expression(n, source)?),
                        _ => {}
                    }
                }
            }
        }

        Ok(UpdateStatement { table_name: table_name.ok_or_else(|| OakError::custom_error("Missing table name in UPDATE"))?, assignments, selection, span: node.span() })
    }

    fn build_assignment<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<Assignment, OakError> {
        let mut column = None;
        let mut value = None;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    SqlElementType::ColumnName => column = Some(self.build_identifier(n, source)?),
                    SqlElementType::Expression => value = Some(self.build_expression(n, source)?),
                    _ => {}
                }
            }
        }

        Ok(Assignment {
            column: column.ok_or_else(|| OakError::custom_error("Missing column in assignment"))?,
            value: value.ok_or_else(|| OakError::custom_error("Missing value in assignment"))?,
            span: node.span(),
        })
    }

    fn build_delete_statement<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<DeleteStatement, OakError> {
        let mut table_name = None;
        let mut selection = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == SqlTokenType::From {
                        // skip
                    }
                },
                RedTree::Node(n) => {
                    match n.green.kind {
                        SqlElementType::TableName => table_name = Some(self.build_table_name(n, source)?),
                        SqlElementType::Expression => selection = Some(self.build_expression(n, source)?),
                        _ => {}
                    }
                }
            }
        }

        Ok(DeleteStatement {
            table_name: table_name.ok_or_else(|| OakError::custom_error("Missing table name in DELETE"))?,
            selection,
            span: node.span(),
        })
    }

    fn build_create_statement<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<CreateStatement, OakError> {
        let mut object_type = CreateObjectType::Table;
        let mut name = None;
        let mut if_not_exists = false;
        let mut columns = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    SqlTokenType::Table => object_type = CreateObjectType::Table,
                    SqlTokenType::View => object_type = CreateObjectType::View,
                    SqlTokenType::Index => object_type = CreateObjectType::Index,
                    SqlTokenType::Exists => if_not_exists = true,
                    _ => {}
                },
                RedTree::Node(n) => {
                    if n.green.kind == SqlElementType::TableName || n.green.kind == SqlElementType::Identifier {
                        name = Some(self.build_identifier(n, source)?);
                    }
                    else if n.green.kind == SqlElementType::ColumnDefinition {
                        columns.push(self.build_column_definition(n, source)?);
                    }
                }
            }
        }

        Ok(CreateStatement { object_type, name: name.ok_or_else(|| OakError::custom_error("Missing name in CREATE"))?, if_not_exists, columns, span: node.span() })
    }

    fn build_column_definition<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<ColumnDefinition, OakError> {
        let mut name = None;
        let mut data_type = String::new();
        let mut constraints = Vec::new();

        // Use a flag to track if we are currently parsing the data type
        let mut parsing_data_type = false;

        for child in node.children() {
            match child {
                RedTree::Node(n) => match n.green.kind {
                    SqlElementType::ColumnName | SqlElementType::Identifier => {
                        if name.is_none() {
                            name = Some(self.build_identifier(n, source)?);
                            parsing_data_type = true;
                        }
                        else if parsing_data_type {
                            if !data_type.is_empty() {
                                data_type.push(' ');
                            }
                            data_type.push_str(&self.get_text(n.span(), source).trim());
                        }
                    }
                    _ => {}
                },
                RedTree::Leaf(t) => {
                    let text = self.get_text(t.span.clone(), source).to_uppercase();

                    match t.kind {
                        SqlTokenType::Primary => {
                            constraints.push(ColumnConstraint::PrimaryKey);
                            parsing_data_type = false;
                        }
                        SqlTokenType::Key => {
                            // Part of PRIMARY KEY, stop parsing data type if we haven't already
                            parsing_data_type = false;
                        }
                        SqlTokenType::Not => {
                            constraints.push(ColumnConstraint::NotNull);
                            parsing_data_type = false;
                        }
                        SqlTokenType::Null => {
                            // Check if NOT NULL was just added
                            let mut is_not_null = false;
                            if let Some(ColumnConstraint::NotNull) = constraints.last() {
                                is_not_null = true;
                            }

                            if !is_not_null {
                                constraints.push(ColumnConstraint::Nullable);
                            }
                            parsing_data_type = false;
                        }
                        SqlTokenType::Unique => {
                            constraints.push(ColumnConstraint::Unique);
                            parsing_data_type = false;
                        }
                        SqlTokenType::AutoIncrement => {
                            constraints.push(ColumnConstraint::AutoIncrement);
                            parsing_data_type = false;
                        }
                        SqlTokenType::Default => {
                            parsing_data_type = false;
                            // The expression follows Default token in column definition
                            if let Some(expr_node) = self.find_next_node_in_parent(node.clone(), SqlElementType::Expression, t.span.end) {
                                constraints.push(ColumnConstraint::Default(self.build_expression(expr_node, source)?));
                            }
                        }
                        SqlTokenType::Check => {
                            parsing_data_type = false;
                            // The expression follows Check token in column definition
                            if let Some(expr_node) = self.find_next_node_in_parent(node.clone(), SqlElementType::Expression, t.span.end) {
                                constraints.push(ColumnConstraint::Check(self.build_expression(expr_node, source)?));
                            }
                        }
                        _ if parsing_data_type => match t.kind {
                            SqlTokenType::NumberLiteral | SqlTokenType::FloatLiteral => data_type.push_str(&text),
                            SqlTokenType::LeftParen => data_type.push('('),
                            SqlTokenType::RightParen => data_type.push(')'),
                            SqlTokenType::Comma => data_type.push(','),
                            _ if t.kind.role() == oak_core::UniversalTokenRole::Keyword || t.kind == SqlTokenType::Identifier_ => {
                                if !data_type.is_empty() && !data_type.ends_with('(') && !data_type.ends_with(',') {
                                    data_type.push(' ');
                                }
                                data_type.push_str(&text);
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }

        Ok(ColumnDefinition { name: name.ok_or_else(|| OakError::custom_error("Missing column name"))?, data_type: data_type.trim().to_uppercase(), constraints, span: node.span() })
    }

    /// Helper to find the next node of a specific kind after a certain position
    fn find_next_node_in_parent<'a>(&self, parent: RedNode<'a, SqlLanguage>, kind: SqlElementType, after_pos: usize) -> Option<RedNode<'a, SqlLanguage>> {
        for child in parent.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == kind && n.span().start >= after_pos {
                    return Some(n);
                }
            }
        }
        None
    }

    fn build_drop_statement<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<DropStatement, OakError> {
        let mut object_type = DropObjectType::Table;
        let mut name = None;
        let mut if_exists = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    SqlTokenType::Table => object_type = DropObjectType::Table,
                    SqlTokenType::View => object_type = DropObjectType::View,
                    SqlTokenType::Index => object_type = DropObjectType::Index,
                    SqlTokenType::Exists => if_exists = true,
                    _ => {}
                },
                RedTree::Node(n) => {
                    if n.green.kind == SqlElementType::TableName || n.green.kind == SqlElementType::Identifier {
                        name = Some(self.build_identifier(n, source)?);
                    }
                }
            }
        }

        Ok(DropStatement { object_type, name: name.ok_or_else(|| OakError::custom_error("Missing name in DROP"))?, if_exists, span: node.span() })
    }

    fn build_alter_statement<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<AlterStatement, OakError> {
        let mut table_name = None;
        let mut action = None;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == SqlElementType::TableName {
                    table_name = Some(self.build_table_name(n, source)?);
                }
                else if n.green.kind == SqlElementType::AlterAction {
                    action = Some(self.build_alter_action(n, source)?);
                }
            }
        }

        Ok(AlterStatement { table_name: table_name.ok_or_else(|| OakError::custom_error("Missing table name in ALTER"))?, action, span: node.span() })
    }

    fn build_alter_action<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<ast::AlterAction, OakError> {
        use crate::lexer::SqlTokenType::*;
        let mut is_add = false;
        let mut is_drop = false;
        let mut is_rename = false;
        let mut identifier: Option<ast::Identifier> = None;
        let mut data_type_tokens = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    Add => is_add = true,
                    Drop => is_drop = true,
                    Rename => is_rename = true,
                    Identifier_ => {
                        if identifier.is_none() {
                            identifier = Some(ast::Identifier { name: self.get_text(t.span.clone(), source), span: t.span.clone() });
                        } else if is_add {
                            data_type_tokens.push(t.span.clone());
                        }
                    }
                    LeftParen | RightParen | NumberLiteral | Comma | Int | Integer | Varchar | Char | Text | Date | Time | Timestamp | Decimal | Float | Double | Boolean => {
                        if is_add && identifier.is_some() {
                            data_type_tokens.push(t.span.clone());
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) if n.green.kind == SqlElementType::Identifier => {
                    identifier = Some(self.build_identifier(n, source)?);
                }
                _ => {}
            }
        }

        let data_type = if data_type_tokens.is_empty() {
            None
        } else {
            let start = data_type_tokens[0].start;
            let end = data_type_tokens.last().unwrap().end;
            Some(self.get_text(core::range::Range { start, end }, source))
        };

        if is_add {
            Ok(ast::AlterAction::AddColumn { name: identifier.ok_or_else(|| OakError::custom_error("Missing column name in ALTER TABLE ADD"))?, data_type })
        }
        else if is_drop {
            Ok(ast::AlterAction::DropColumn { name: identifier.ok_or_else(|| OakError::custom_error("Missing column name in ALTER TABLE DROP"))? })
        }
        else if is_rename {
            Ok(ast::AlterAction::RenameTo { new_name: identifier.ok_or_else(|| OakError::custom_error("Missing new name in ALTER TABLE RENAME"))? })
        }
        else {
            Err(OakError::custom_error("Unknown ALTER action"))
        }
    }

    fn build_expression<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<Expression, OakError> {
        let mut left = None;
        let mut op_token = None;
        let mut right = None;

        // Special case for expression wrapped in another expression (e.g. from parentheses or Pratt recursion)
        // Check if there is ONLY ONE child node that is an Expression or Identifier
        let children: Vec<_> = node.children().collect();
        if children.len() == 1 {
            if let RedTree::Node(n) = &children[0] {
                if n.green.kind == SqlElementType::Expression || n.green.kind == SqlElementType::Identifier {
                    return self.build_expression(n.clone(), source);
                }
            }
        }

        for child in children {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    SqlTokenType::NumberLiteral | SqlTokenType::FloatLiteral => {
                        let expr = Expression::Literal(Literal::Number(self.get_text(t.span.clone(), source).trim().to_string(), t.span.clone()));
                        if left.is_none() {
                            left = Some(expr);
                        }
                        else {
                            right = Some(expr);
                        }
                    }
                    SqlTokenType::StringLiteral => {
                        let text = self.get_text(t.span.clone(), source).trim().to_string();
                        let content = if (text.starts_with('\'') && text.ends_with('\'')) || (text.starts_with('"') && text.ends_with('"')) { if text.len() >= 2 { &text[1..text.len() - 1] } else { "" } } else { &text };
                        let expr = Expression::Literal(Literal::String(content.to_string(), t.span.clone()));
                        if left.is_none() {
                            left = Some(expr);
                        }
                        else {
                            right = Some(expr);
                        }
                    }
                    SqlTokenType::True | SqlTokenType::False => {
                        let expr = Expression::Literal(Literal::Boolean(t.kind == SqlTokenType::True, t.span.clone()));
                        if left.is_none() {
                            left = Some(expr);
                        }
                        else {
                            right = Some(expr);
                        }
                    }
                    SqlTokenType::Null => {
                        let expr = Expression::Literal(Literal::Null(t.span.clone()));
                        if left.is_none() {
                            left = Some(expr);
                        }
                        else {
                            right = Some(expr);
                        }
                    }
                    SqlTokenType::Plus | SqlTokenType::Minus | SqlTokenType::Star | SqlTokenType::Slash | SqlTokenType::Equal | SqlTokenType::NotEqual | SqlTokenType::Less | SqlTokenType::Greater | SqlTokenType::LessEqual | SqlTokenType::GreaterEqual | SqlTokenType::And | SqlTokenType::Or => {
                        op_token = Some(t.clone());
                    }
                    _ => {}
                },
                RedTree::Node(n) => {
                    let expr = if n.green.kind == SqlElementType::Expression {
                        self.build_expression(n, source)?
                    }
                    else if n.green.kind == SqlElementType::Identifier {
                        Expression::Identifier(self.build_identifier(n, source)?)
                    }
                    else {
                        continue;
                    };

                    if left.is_none() {
                        left = Some(expr);
                    }
                    else {
                        right = Some(expr);
                    }
                }
            }
        }

        if let (Some(l), Some(op), Some(r)) = (left.clone(), op_token, right) {
            if let Some(binary_op) = self.map_binary_op(op.kind) {
                return Ok(Expression::Binary { left: Box::new(l), op: binary_op, right: Box::new(r), span: node.span() });
            }
        }

        left.ok_or_else(|| OakError::custom_error("Empty expression"))
    }

    fn build_table_name<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<TableName, OakError> {
        let mut name = None;
        for child in node.children() {
            match child {
                RedTree::Leaf(t) if t.kind == SqlTokenType::Identifier_ => {
                    name = Some(Identifier { name: self.get_text(t.span.clone(), source), span: t.span.clone() });
                }
                RedTree::Node(n) if n.green.kind == SqlElementType::Identifier => {
                    name = Some(self.build_identifier(n, source)?);
                }
                _ => {}
            }
        }
        Ok(TableName { name: name.ok_or_else(|| OakError::custom_error("Missing table name"))?, span: node.span() })
    }

    fn build_column_name<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<ColumnName, OakError> {
        let mut name = None;
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    SqlTokenType::Identifier_ => {
                        name = Some(Identifier { name: self.get_text(t.span.clone(), source), span: t.span.clone() });
                    }
                    _ => {}
                },
                RedTree::Node(n) if n.green.kind == SqlElementType::Identifier => {
                    name = Some(self.build_identifier(n, source)?);
                }
                _ => {}
            }
        }
        Ok(ColumnName { name: name.ok_or_else(|| OakError::custom_error("Missing column name"))?, span: node.span() })
    }

    fn build_identifier<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<Identifier, OakError> {
        Ok(Identifier { name: self.get_text(node.span(), source).trim().to_string(), span: node.span() })
    }

    fn build_join_clause<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<JoinClause, OakError> {
        let mut join_type = JoinType::Inner;
        let mut table = None;
        let mut on = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    SqlTokenType::Inner => join_type = JoinType::Inner,
                    SqlTokenType::Left => join_type = JoinType::Left,
                    SqlTokenType::Right => join_type = JoinType::Right,
                    SqlTokenType::Full => join_type = JoinType::Full,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    SqlElementType::TableName => table = Some(self.build_table_name(n, source)?),
                    SqlElementType::Expression => on = Some(self.build_expression(n, source)?),
                    _ => {}
                },
            }
        }

        Ok(JoinClause { join_type, table: table.ok_or_else(|| OakError::custom_error("Missing table in JOIN"))?, on, span: node.span() })
    }

    fn build_group_by_clause<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<GroupByClause, OakError> {
        let mut columns = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == SqlElementType::Expression {
                    columns.push(self.build_expression(n, source)?);
                }
            }
        }
        Ok(GroupByClause { columns, span: node.span() })
    }

    fn build_having_clause<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<HavingClause, OakError> {
        let mut condition = None;
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == SqlElementType::Expression {
                    condition = Some(self.build_expression(n, source)?);
                }
            }
        }
        Ok(HavingClause { condition: condition.ok_or_else(|| OakError::custom_error("Missing condition in HAVING"))?, span: node.span() })
    }

    fn build_order_by_clause<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<OrderByClause, OakError> {
        let mut items = Vec::new();
        let mut current_expr = None;

        for child in node.children() {
            match child {
                RedTree::Node(n) => {
                    if n.green.kind == SqlElementType::Expression {
                        if let Some(expr) = current_expr.take() {
                            items.push(OrderByItem { expr, direction: OrderDirection::Asc });
                        }
                        current_expr = Some(self.build_expression(n, source)?);
                    }
                }
                RedTree::Leaf(t) => match t.kind {
                    SqlTokenType::Asc => {
                        if let Some(expr) = current_expr.take() {
                            items.push(OrderByItem { expr, direction: OrderDirection::Asc });
                        }
                    }
                    SqlTokenType::Desc => {
                        if let Some(expr) = current_expr.take() {
                            items.push(OrderByItem { expr, direction: OrderDirection::Desc });
                        }
                    }
                    _ => {}
                },
            }
        }

        if let Some(expr) = current_expr {
            items.push(OrderByItem { expr, direction: OrderDirection::Asc });
        }

        Ok(OrderByClause { items, span: node.span() })
    }

    fn build_limit_clause<'a>(&self, node: RedNode<'a, SqlLanguage>, source: &SourceText) -> Result<LimitClause, OakError> {
        let mut limit = None;
        let mut offset = None;

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == SqlTokenType::NumberLiteral {
                    let expr = Expression::Literal(Literal::Number(self.get_text(t.span.clone(), source), t.span.clone()));
                    if limit.is_none() {
                        limit = Some(expr);
                    }
                    else {
                        offset = Some(expr);
                    }
                }
            }
            else if let RedTree::Node(n) = child {
                if n.green.kind == SqlElementType::Expression {
                    let expr = self.build_expression(n, source)?;
                    if limit.is_none() {
                        limit = Some(expr);
                    }
                    else {
                        offset = Some(expr);
                    }
                }
            }
        }

        Ok(LimitClause { limit: limit.ok_or_else(|| OakError::custom_error("Missing limit value"))?, offset, span: node.span() })
    }

    fn get_text(&self, span: core::range::Range<usize>, source: &SourceText) -> String {
        source.get_text_in(span).to_string()
    }

    fn is_binary_op(&self, kind: SqlTokenType) -> bool {
        matches!(
            kind,
            SqlTokenType::Plus
                | SqlTokenType::Minus
                | SqlTokenType::Star
                | SqlTokenType::Slash
                | SqlTokenType::Equal
                | SqlTokenType::NotEqual
                | SqlTokenType::Less
                | SqlTokenType::Greater
                | SqlTokenType::LessEqual
                | SqlTokenType::GreaterEqual
                | SqlTokenType::And
                | SqlTokenType::Or
                | SqlTokenType::Like
        )
    }

    fn map_binary_op(&self, kind: SqlTokenType) -> Option<BinaryOperator> {
        match kind {
            SqlTokenType::Plus => Some(BinaryOperator::Plus),
            SqlTokenType::Minus => Some(BinaryOperator::Minus),
            SqlTokenType::Star => Some(BinaryOperator::Star),
            SqlTokenType::Slash => Some(BinaryOperator::Slash),
            SqlTokenType::Equal => Some(BinaryOperator::Equal),
            SqlTokenType::NotEqual => Some(BinaryOperator::NotEqual),
            SqlTokenType::Less => Some(BinaryOperator::Less),
            SqlTokenType::Greater => Some(BinaryOperator::Greater),
            SqlTokenType::LessEqual => Some(BinaryOperator::LessEqual),
            SqlTokenType::GreaterEqual => Some(BinaryOperator::GreaterEqual),
            SqlTokenType::And => Some(BinaryOperator::And),
            SqlTokenType::Or => Some(BinaryOperator::Or),
            SqlTokenType::Like => Some(BinaryOperator::Like),
            _ => None,
        }
    }
}
