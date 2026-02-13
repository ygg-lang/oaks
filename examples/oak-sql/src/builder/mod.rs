use crate::{
    SqlElementType, SqlLanguage, SqlLexer, SqlParser,
    ast::*,
    lexer::token_type::SqlTokenType,
};
use oak_core::{
    Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree,
    SourceText, TextEdit, builder::BuildOutput, source::Source,
};

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
                    Ok(ast_root) => OakDiagnostics {
                        result: Ok(ast_root),
                        diagnostics: parse_result.diagnostics,
                    },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics {
                            result: Err(build_error),
                            diagnostics,
                        }
                    }
                }
            }
            Err(parse_error) => OakDiagnostics {
                result: Err(parse_error),
                diagnostics: parse_result.diagnostics,
            },
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

        Ok(SqlRoot {
            statements,
            span: root_node.span(),
        })
    }

    fn build_select_statement(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<SelectStatement, OakError> {
        let mut items = Vec::new();
        let mut from = None;
        let mut joins = Vec::new();
        let mut selection = None;
        let mut group_by = None;
        let mut having = None;
        let mut order_by = None;
        let mut limit = None;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    SqlElementType::Expression => {
                        let expr = self.build_expression(n, source)?;
                        if from.is_none() {
                            items.push(expr);
                        } else if selection.is_none() {
                            selection = Some(expr);
                        }
                    }
                    SqlElementType::TableName => from = Some(self.build_table_name(n, source)?),
                    SqlElementType::JoinClause => joins.push(self.build_join_clause(n, source)?),
                    SqlElementType::GroupByClause => group_by = Some(self.build_group_by_clause(n, source)?),
                    SqlElementType::HavingClause => having = Some(self.build_having_clause(n, source)?),
                    SqlElementType::OrderByClause => order_by = Some(self.build_order_by_clause(n, source)?),
                    SqlElementType::LimitClause => limit = Some(self.build_limit_clause(n, source)?),
                    _ => {}
                }
            }
        }

        Ok(SelectStatement {
            items,
            from,
            joins,
            selection,
            group_by,
            having,
            order_by,
            limit,
            span: node.span(),
        })
    }

    fn build_insert_statement(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<InsertStatement, OakError> {
        let mut table_name = None;
        let mut columns = Vec::new();
        let mut values = Vec::new();

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    SqlElementType::TableName => table_name = Some(self.build_table_name(n, source)?),
                    SqlElementType::ColumnName => columns.push(self.build_identifier(n, source)?),
                    SqlElementType::Expression => values.push(self.build_expression(n, source)?),
                    _ => {}
                }
            }
        }

        Ok(InsertStatement {
            table_name: table_name.ok_or_else(|| OakError::custom("Missing table name in INSERT"))?,
            columns,
            values,
            span: node.span(),
        })
    }

    fn build_update_statement(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<UpdateStatement, OakError> {
        let mut table_name = None;
        let mut assignments = Vec::new();
        let mut selection = None;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    SqlElementType::TableName => table_name = Some(self.build_table_name(n, source)?),
                    SqlElementType::Assignment => assignments.push(self.build_assignment(n, source)?),
                    SqlElementType::Expression => selection = Some(self.build_expression(n, source)?),
                    _ => {}
                }
            }
        }

        Ok(UpdateStatement {
            table_name: table_name.ok_or_else(|| OakError::custom("Missing table name in UPDATE"))?,
            assignments,
            selection,
            span: node.span(),
        })
    }

    fn build_assignment(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<Assignment, OakError> {
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
            column: column.ok_or_else(|| OakError::custom("Missing column in assignment"))?,
            value: value.ok_or_else(|| OakError::custom("Missing value in assignment"))?,
            span: node.span(),
        })
    }

    fn build_delete_statement(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<DeleteStatement, OakError> {
        let mut table_name = None;
        let mut selection = None;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    SqlElementType::TableName => table_name = Some(self.build_table_name(n, source)?),
                    SqlElementType::Expression => selection = Some(self.build_expression(n, source)?),
                    _ => {}
                }
            }
        }

        Ok(DeleteStatement {
            table_name: table_name.ok_or_else(|| OakError::custom("Missing table name in DELETE"))?,
            selection,
            span: node.span(),
        })
    }

    fn build_create_statement(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<CreateStatement, OakError> {
        let mut object_type = CreateObjectType::Table;
        let mut name = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    SqlTokenType::Table => object_type = CreateObjectType::Table,
                    SqlTokenType::View => object_type = CreateObjectType::View,
                    SqlTokenType::Index => object_type = CreateObjectType::Index,
                    _ => {}
                },
                RedTree::Node(n) => if n.green.kind == SqlElementType::TableName || n.green.kind == SqlElementType::Identifier {
                    name = Some(self.build_identifier(n, source)?);
                },
                _ => {}
            }
        }

        Ok(CreateStatement {
            object_type,
            name: name.ok_or_else(|| OakError::custom("Missing name in CREATE"))?,
            span: node.span(),
        })
    }

    fn build_drop_statement(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<DropStatement, OakError> {
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
                RedTree::Node(n) => if n.green.kind == SqlElementType::TableName || n.green.kind == SqlElementType::Identifier {
                    name = Some(self.build_identifier(n, source)?);
                },
                _ => {}
            }
        }

        Ok(DropStatement {
            object_type,
            name: name.ok_or_else(|| OakError::custom("Missing name in DROP"))?,
            if_exists,
            span: node.span(),
        })
    }

    fn build_alter_statement(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<AlterStatement, OakError> {
        let mut table_name = None;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == SqlElementType::TableName {
                    table_name = Some(self.build_table_name(n, source)?);
                }
            }
        }

        Ok(AlterStatement {
            table_name: table_name.ok_or_else(|| OakError::custom("Missing table name in ALTER"))?,
            span: node.span(),
        })
    }

    fn build_expression(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<Expression, OakError> {
        // Simplified expression building
        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                match t.kind {
                    SqlTokenType::NumberLiteral => return Ok(Expression::Literal(Literal::Number(self.get_text(t.span.clone(), source), t.span.clone()))),
                    SqlTokenType::StringLiteral => return Ok(Expression::Literal(Literal::String(self.get_text(t.span.clone(), source), t.span.clone()))),
                    SqlTokenType::BooleanLiteral => return Ok(Expression::Literal(Literal::Boolean(self.get_text(t.span.clone(), source) == "TRUE", t.span.clone()))),
                    SqlTokenType::NullLiteral => return Ok(Expression::Literal(Literal::Null(t.span.clone()))),
                    SqlTokenType::Identifier_ => return Ok(Expression::Identifier(Identifier { name: self.get_text(t.span.clone(), source), span: t.span.clone() })),
                    _ => {}
                }
            } else if let RedTree::Node(n) = child {
                if n.green.kind == SqlElementType::Identifier {
                    return Ok(Expression::Identifier(self.build_identifier(n, source)?));
                }
            }
        }

        // Fallback for complex expressions - just treat as a literal for now
        Ok(Expression::Literal(Literal::String(self.get_text(node.span(), source), node.span())))
    }

    fn build_table_name(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<TableName, OakError> {
        Ok(TableName {
            name: self.build_identifier(node, source)?,
            span: node.span(),
        })
    }

    fn build_identifier(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<Identifier, OakError> {
        Ok(Identifier {
            name: self.get_text(node.span(), source),
            span: node.span(),
        })
    }

    fn build_join_clause(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<JoinClause, OakError> {
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
                _ => {}
            }
        }

        Ok(JoinClause {
            join_type,
            table: table.ok_or_else(|| OakError::custom("Missing table in JOIN"))?,
            on,
            span: node.span(),
        })
    }

    fn build_group_by_clause(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<GroupByClause, OakError> {
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

    fn build_having_clause(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<HavingClause, OakError> {
        let mut condition = None;
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == SqlElementType::Expression {
                    condition = Some(self.build_expression(n, source)?);
                }
            }
        }
        Ok(HavingClause {
            condition: condition.ok_or_else(|| OakError::custom("Missing condition in HAVING"))?,
            span: node.span(),
        })
    }

    fn build_order_by_clause(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<OrderByClause, OakError> {
        let mut items = Vec::new();
        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == SqlElementType::Expression {
                    items.push(OrderByItem {
                        expr: self.build_expression(n, source)?,
                        direction: OrderDirection::Asc, // Default
                    });
                }
            }
        }
        Ok(OrderByClause { items, span: node.span() })
    }

    fn build_limit_clause(&self, node: RedNode<SqlLanguage>, source: &SourceText) -> Result<LimitClause, OakError> {
        let mut limit = None;
        let mut offset = None;

        for child in node.children() {
            if let RedTree::Node(n) = child {
                if n.green.kind == SqlElementType::Expression {
                    if limit.is_none() {
                        limit = Some(self.build_expression(n, source)?);
                    } else {
                        offset = Some(self.build_expression(n, source)?);
                    }
                }
            }
        }

        Ok(LimitClause {
            limit: limit.ok_or_else(|| OakError::custom("Missing limit value"))?,
            offset,
            span: node.span(),
        })
    }

    fn get_text(&self, span: core::range::Range<usize>, source: &SourceText) -> String {
        source.text()[span.start..span.end].to_string()
    }
}
