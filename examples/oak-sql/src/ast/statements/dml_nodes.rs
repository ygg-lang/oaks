use crate::ast::expr::{Expression, Identifier, TableName};
use core::range::Range;
use oak_core::source::{SourceBuffer, ToSource};

/// Represents an INSERT statement.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertStatement {
    /// The table to insert into.
    pub table_name: TableName,
    /// The columns to insert into.
    pub columns: Vec<Identifier>,
    /// The values to insert.
    pub values: Vec<Expression>,
    /// The span of the INSERT statement.
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl ToSource for InsertStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("INSERT");
        buffer.push("INTO");
        self.table_name.to_source(buffer);
        if !self.columns.is_empty() {
            buffer.push("(");
            for (i, col) in self.columns.iter().enumerate() {
                if i > 0 {
                    buffer.push(",");
                }
                col.to_source(buffer);
            }
            buffer.push(")");
        }
        buffer.push("VALUES");
        buffer.push("(");
        for (i, val) in self.values.iter().enumerate() {
            if i > 0 {
                buffer.push(",");
            }
            val.to_source(buffer);
        }
        buffer.push(")");
    }
}

/// Represents an UPDATE statement.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateStatement {
    /// The table to update.
    pub table_name: TableName,
    /// The assignments to perform.
    pub assignments: Vec<Assignment>,
    /// The WHERE clause selection expression.
    pub selection: Option<Expression>, // WHERE clause
    /// The span of the UPDATE statement.
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl ToSource for UpdateStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("UPDATE");
        self.table_name.to_source(buffer);
        buffer.push("SET");
        for (i, assignment) in self.assignments.iter().enumerate() {
            if i > 0 {
                buffer.push(",");
            }
            assignment.to_source(buffer);
        }
        if let Some(selection) = &self.selection {
            buffer.push("WHERE");
            selection.to_source(buffer);
        }
    }
}

/// Represents an assignment in an UPDATE statement.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Assignment {
    /// The column to assign to.
    pub column: Identifier,
    /// The expression to assign.
    pub value: Expression,
    /// The span of the assignment.
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl ToSource for Assignment {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        self.column.to_source(buffer);
        buffer.push("=");
        self.value.to_source(buffer);
    }
}

/// Represents a DELETE statement.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteStatement {
    /// The table to delete from.
    pub table_name: TableName,
    /// The WHERE clause selection expression.
    pub selection: Option<Expression>, // WHERE clause
    /// The span of the DELETE statement.
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

impl ToSource for DeleteStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("DELETE");
        buffer.push("FROM");
        self.table_name.to_source(buffer);
        if let Some(selection) = &self.selection {
            buffer.push("WHERE");
            selection.to_source(buffer);
        }
    }
}
