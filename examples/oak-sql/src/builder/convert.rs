use crate::{ast::*, language::SqlLanguage, lexer::SqlTokenType, parser::element_type::SqlElementType, SqlBuilder};
use oak_core::{GreenNode, OakError, RedNode, RedTree, SourceText};

impl<'config> SqlBuilder<'config> {
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, SqlLanguage>, source: &SourceText) -> Result<SqlRoot, OakError> {
        let root_node = RedNode::new(green_tree, 0);
        let mut statements = Vec::new();

        for child in root_node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    SqlElementType::SelectStatement => statements.push(Statement::Select(self.build_select(n, source)?)),
                    SqlElementType::InsertStatement => statements.push(Statement::Insert(self.build_insert(n, source)?)),
                    SqlElementType::UpdateStatement => statements.push(Statement::Update(self.build_update(n, source)?)),
                    SqlElementType::DeleteStatement => statements.push(Statement::Delete(self.build_delete(n, source)?)),
                    SqlElementType::CreateStatement => statements.push(Statement::Create(self.build_create(n, source)?)),
                    SqlElementType::DropStatement => statements.push(Statement::Drop(self.build_drop(n, source)?)),
                    SqlElementType::AlterStatement => statements.push(Statement::Alter(self.build_alter(n, source)?)),
                    _ => {}
                }
            }
        }

        Ok(SqlRoot {
            statements,
            span: root_node.span(),
        })
    }

    fn build_select(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<SelectStatement, OakError> {
        let mut select_items = Vec::new();
        let mut from = None;
        let mut joins = Vec::new();
        let mut r#where = None;
        let mut group_by = None;
        let mut having = None;
        let mut order_by = None;
        let mut limit = None;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    SqlElementType::SelectItem => select_items.push(self.build_select_item(n, source)?),
                    SqlElementType::TableName => from = Some(self.build_table_name(n, source)?),
                    SqlElementType::JoinClause => joins.push(self.build_join_clause(n, source)?),
                    SqlElementType::Expression => {
                        // This could be WHERE or other expressions. 
                        // In a real implementation, we'd check the preceding token or context.
                        // For simplicity, assume the first one after FROM is WHERE.
                        if r#where.is_none() {
                            r#where = Some(self.build_expression(n, source)?);
                        }
                    }
                    SqlElementType::GroupByClause => group_by = Some(self.build_group_by(n, source)?),
                    SqlElementType::HavingClause => having = Some(self.build_having(n, source)?),
                    SqlElementType::OrderByClause => order_by = Some(self.build_order_by(n, source)?),
                    SqlElementType::LimitClause => limit = Some(self.build_limit(n, source)?),
                    _ => {}
                }
            }
        }

        Ok(SelectStatement {
            select_items,
            from,
            joins,
            r#where,
            group_by,
            having,
            order_by,
            limit,
            span: node.span(),
        })
    }

    fn build_select_item(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<SelectItem, OakError> {
        let mut expression = None;
        let mut alias = None;
        let mut is_star = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if t.kind == SqlTokenType::Star {
                        is_star = true;
                    } else if t.kind == SqlTokenType::Identifier_ {
                        alias = Some(text(source, t.span.clone()));
                    }
                }
                RedTree::Node(n) => {
                    if n.green.kind == SqlElementType::Expression {
                        expression = Some(self.build_expression(n, source)?);
                    }
                }
                _ => {}
            }
        }

        if is_star {
            Ok(SelectItem::Star { span: node.span() })
        } else {
            Ok(SelectItem::Expression {
                expression: expression.unwrap_or(Expression::Identifier { name: "".to_string(), span: node.span() }),
                alias,
                span: node.span(),
            })
        }
    }

    fn build_table_name(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<TableName, OakError> {
        let mut name = String::new();
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == SqlTokenType::Identifier_ {
                    name = text(source, t.span.clone());
                }
            }
        }
        Ok(TableName { name, span: node.span() })
    }

    fn build_expression(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<Expression, OakError> {
        // Basic expression building. Real implementation would be recursive and handle all operators.
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    match t.kind {
                        SqlTokenType::Identifier_ => return Ok(Expression::Identifier { name: text(source, t.span.clone()), span: t.span.clone() }),
                        SqlTokenType::NumberLiteral | SqlTokenType::StringLiteral => return Ok(Expression::Literal { value: text(source, t.span.clone()), span: t.span.clone() }),
                        _ => {}
                    }
                }
                RedTree::Node(n) => {
                    // For now, just return the first sub-expression
                    return self.build_expression(n, source);
                }
                _ => {}
            }
        }
        Ok(Expression::Identifier { name: "unknown".to_string(), span: node.span() })
    }

    fn build_join_clause(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<JoinClause, OakError> {
        let mut join_type = "JOIN".to_string();
        let mut table = None;
        let mut on = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if matches!(t.kind, SqlTokenType::Left | SqlTokenType::Right | SqlTokenType::Inner | SqlTokenType::Outer | SqlTokenType::Full) {
                        join_type = text(source, t.span.clone());
                    }
                }
                RedTree::Node(n) => {
                    if n.green.kind == SqlElementType::TableName {
                        table = Some(self.build_table_name(n, source)?);
                    } else if n.green.kind == SqlElementType::Expression {
                        on = Some(self.build_expression(n, source)?);
                    }
                }
                _ => {}
            }
        }

        Ok(JoinClause {
            join_type,
            table: table.unwrap_or(TableName { name: "".to_string(), span: node.span() }),
            on,
            span: node.span(),
        })
    }

    fn build_group_by(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<GroupByClause, OakError> {
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

    fn build_having(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<HavingClause, OakError> {
        let mut condition = None;
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == SqlElementType::Expression {
                    condition = Some(self.build_expression(n, source)?);
                }
            }
        }
        Ok(HavingClause {
            condition: condition.unwrap_or(Expression::Identifier { name: "".to_string(), span: node.span() }),
            span: node.span(),
        })
    }

    fn build_order_by(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<OrderByClause, OakError> {
        let mut items = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == SqlElementType::Expression {
                    items.push(OrderByItem {
                        expression: self.build_expression(n, source)?,
                        direction: None, // Need to handle ASC/DESC
                    });
                }
            }
        }
        Ok(OrderByClause { items, span: node.span() })
    }

    fn build_limit(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<LimitClause, OakError> {
        let mut limit = 0;
        let mut offset = None;

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                if t.kind == SqlTokenType::NumberLiteral {
                    let val = text(source, t.span.clone()).parse::<usize>().unwrap_or(0);
                    if limit == 0 {
                        limit = val;
                    } else {
                        offset = Some(val);
                    }
                }
            }
        }

        Ok(LimitClause { limit, offset, span: node.span() })
    }

    fn build_insert(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<InsertStatement, OakError> {
        let mut table = None;
        let mut columns = Vec::new();
        let mut values = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    SqlElementType::TableName => table = Some(self.build_table_name(n, source)?),
                    SqlElementType::ColumnName => {
                        for sub in n.children() {
                            if let RedTree::Leaf(t) = sub {
                                if t.kind == SqlTokenType::Identifier_ {
                                    columns.push(text(source, t.span.clone()));
                                }
                            }
                        }
                    }
                    SqlElementType::ValueList => {
                        let mut row = Vec::new();
                        for sub in n.children() {
                            if let RedTree::Node(sn) = sub {
                                if sn.green.kind == SqlElementType::Expression {
                                    row.push(self.build_expression(sn, source)?);
                                }
                            }
                        }
                        if !row.is_empty() {
                            values.push(row);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(InsertStatement {
            table: table.unwrap_or(TableName { name: "".to_string(), span: node.span() }),
            columns,
            values,
            span: node.span(),
        })
    }

    fn build_update(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<UpdateStatement, OakError> {
        let mut table = None;
        let mut assignments = Vec::new();
        let mut r#where = None;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    SqlElementType::TableName => table = Some(self.build_table_name(n, source)?),
                    SqlElementType::Assignment => {
                        let mut column = String::new();
                        let mut value = None;
                        for sub in n.children() {
                            match sub {
                                RedTree::Node(sn) => {
                                    if sn.green.kind == SqlElementType::ColumnName {
                                        for ssub in sn.children() {
                                            if let RedTree::Leaf(t) = ssub {
                                                if t.kind == SqlTokenType::Identifier_ {
                                                    column = text(source, t.span.clone());
                                                }
                                            }
                                        }
                                    } else if sn.green.kind == SqlElementType::Expression {
                                        value = Some(self.build_expression(sn, source)?);
                                    }
                                }
                                _ => {}
                            }
                        }
                        if let Some(v) = value {
                            assignments.push(Assignment { column, value: v });
                        }
                    }
                    SqlElementType::Expression => r#where = Some(self.build_expression(n, source)?),
                    _ => {}
                }
            }
        }

        Ok(UpdateStatement {
            table: table.unwrap_or(TableName { name: "".to_string(), span: node.span() }),
            assignments,
            r#where,
            span: node.span(),
        })
    }

    fn build_delete(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<DeleteStatement, OakError> {
        let mut table = None;
        let mut r#where = None;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    SqlElementType::TableName => table = Some(self.build_table_name(n, source)?),
                    SqlElementType::Expression => r#where = Some(self.build_expression(n, source)?),
                    _ => {}
                }
            }
        }

        Ok(DeleteStatement {
            table: table.unwrap_or(TableName { name: "".to_string(), span: node.span() }),
            r#where,
            span: node.span(),
        })
    }

    fn build_create(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<CreateStatement, OakError> {
        let mut object_type = "TABLE".to_string();
        let mut name = String::new();
        let mut columns = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if matches!(t.kind, SqlTokenType::Table | SqlTokenType::View | SqlTokenType::Index) {
                        object_type = text(source, t.span.clone());
                    }
                }
                RedTree::Node(n) => {
                    match n.green.kind {
                        SqlElementType::TableName => {
                            for sub in n.children() {
                                if let RedTree::Leaf(t) = sub {
                                    if t.kind == SqlTokenType::Identifier_ {
                                        name = text(source, t.span.clone());
                                    }
                                }
                            }
                        }
                        SqlElementType::ColumnDefinition => {
                            let mut col_name = String::new();
                            let mut data_type = "TEXT".to_string();
                            let mut constraints = Vec::new();
                            
                            for sub in n.children() {
                                match sub {
                                    RedTree::Node(sn) => {
                                        if sn.green.kind == SqlElementType::ColumnName {
                                            for ssub in sn.children() {
                                                if let RedTree::Leaf(t) = ssub {
                                                    if t.kind == SqlTokenType::Identifier_ {
                                                        col_name = text(source, t.span.clone());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    RedTree::Leaf(t) => {
                                        // Simple heuristic for type and constraints
                                        if t.kind == SqlTokenType::Identifier_ {
                                            data_type = text(source, t.span.clone());
                                        } else if matches!(t.kind, SqlTokenType::Primary | SqlTokenType::Key | SqlTokenType::Not | SqlTokenType::Null | SqlTokenType::Unique) {
                                            constraints.push(text(source, t.span.clone()));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            columns.push(ColumnDefinition { name: col_name, data_type, constraints });
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        Ok(CreateStatement {
            object_type,
            name,
            columns,
            span: node.span(),
        })
    }

    fn build_drop(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<DropStatement, OakError> {
        let mut object_type = "TABLE".to_string();
        let mut name = String::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => {
                    if matches!(t.kind, SqlTokenType::Table | SqlTokenType::View | SqlTokenType::Index) {
                        object_type = text(source, t.span.clone());
                    }
                }
                RedTree::Node(n) => {
                    if n.green.kind == SqlElementType::TableName {
                        for sub in n.children() {
                            if let RedTree::Leaf(t) = sub {
                                if t.kind == SqlTokenType::Identifier_ {
                                    name = text(source, t.span.clone());
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(DropStatement { object_type, name, span: node.span() })
    }

    fn build_alter(&self, node: RedNode<SqlLanguage>, _source: &SourceText) -> Result<AlterStatement, OakError> {
        Ok(AlterStatement { span: node.span() })
    }
}

fn text(source: &SourceText, span: core::range::Range<usize>) -> String {
    source.get_text_in(span).to_string()
}
