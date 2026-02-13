#![doc = include_str!("readme.md")]
use core::range::Range;
use oak_core::source::{SourceBuffer, ToSource};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// SQL 根节点
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SqlRoot {
    pub statements: Vec<SqlStatement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for SqlRoot {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        for (i, stmt) in self.statements.iter().enumerate() {
            if i > 0 {
                buffer.push(";");
            }
            stmt.to_source(buffer);
        }
        if !self.statements.is_empty() {
            buffer.push(";");
        }
    }
}

/// SQL 语句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SqlStatement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
    Create(CreateStatement),
    Drop(DropStatement),
    Alter(AlterStatement),
    Unknown {
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

impl ToSource for SqlStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            SqlStatement::Select(s) => s.to_source(buffer),
            SqlStatement::Insert(s) => s.to_source(buffer),
            SqlStatement::Update(s) => s.to_source(buffer),
            SqlStatement::Delete(s) => s.to_source(buffer),
            SqlStatement::Create(s) => s.to_source(buffer),
            SqlStatement::Drop(s) => s.to_source(buffer),
            SqlStatement::Alter(s) => s.to_source(buffer),
            SqlStatement::Unknown { .. } => {}
        }
    }
}

/// SELECT 语句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SelectStatement {
    pub items: Vec<SelectItem>,
    pub from: Option<TableName>,
    pub joins: Vec<JoinClause>,
    pub selection: Option<Expression>, // WHERE clause
    pub group_by: Option<GroupByClause>,
    pub having: Option<HavingClause>,
    pub order_by: Option<OrderByClause>,
    pub limit: Option<LimitClause>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for SelectStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("SELECT");
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                buffer.push(",");
            }
            item.to_source(buffer);
        }
        if let Some(from) = &self.from {
            buffer.push("FROM");
            from.to_source(buffer);
        }
        for join in &self.joins {
            join.to_source(buffer);
        }
        if let Some(selection) = &self.selection {
            buffer.push("WHERE");
            selection.to_source(buffer);
        }
        if let Some(group_by) = &self.group_by {
            group_by.to_source(buffer);
        }
        if let Some(having) = &self.having {
            having.to_source(buffer);
        }
        if let Some(order_by) = &self.order_by {
            order_by.to_source(buffer);
        }
        if let Some(limit) = &self.limit {
            limit.to_source(buffer);
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SelectItem {
    Star {
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    Expression {
        expr: Expression,
        alias: Option<Identifier>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

impl ToSource for SelectItem {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            SelectItem::Star { .. } => buffer.push("*"),
            SelectItem::Expression { expr, alias, .. } => {
                expr.to_source(buffer);
                if let Some(alias) = alias {
                    buffer.push("AS");
                    alias.to_source(buffer);
                }
            }
        }
    }
}

/// INSERT 语句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InsertStatement {
    pub table_name: TableName,
    pub columns: Vec<Identifier>,
    pub values: Vec<Expression>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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

/// UPDATE 语句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateStatement {
    pub table_name: TableName,
    pub assignments: Vec<Assignment>,
    pub selection: Option<Expression>, // WHERE clause
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Assignment {
    pub column: Identifier,
    pub value: Expression,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for Assignment {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        self.column.to_source(buffer);
        buffer.push("=");
        self.value.to_source(buffer);
    }
}

/// DELETE 语句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeleteStatement {
    pub table_name: TableName,
    pub selection: Option<Expression>, // WHERE clause
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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

/// CREATE 语句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateStatement {
    pub object_type: CreateObjectType,
    pub name: Identifier,
    pub if_not_exists: bool,
    pub columns: Vec<ColumnDefinition>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for CreateStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("CREATE");
        self.object_type.to_source(buffer);
        if self.if_not_exists {
            buffer.push("IF");
            buffer.push("NOT");
            buffer.push("EXISTS");
        }
        self.name.to_source(buffer);
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
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ColumnDefinition {
    pub name: Identifier,
    pub data_type: String,
    pub constraints: Vec<ColumnConstraint>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for ColumnDefinition {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        self.name.to_source(buffer);
        buffer.push(&self.data_type);
        for constraint in &self.constraints {
            constraint.to_source(buffer);
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ColumnConstraint {
    PrimaryKey,
    NotNull,
    Nullable,
    Unique,
    Default(Expression),
    Check(Expression),
    AutoIncrement,
}

impl ToSource for ColumnConstraint {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            ColumnConstraint::PrimaryKey => {
                buffer.push("PRIMARY");
                buffer.push("KEY");
            }
            ColumnConstraint::NotNull => {
                buffer.push("NOT");
                buffer.push("NULL");
            }
            ColumnConstraint::Nullable => buffer.push("NULL"),
            ColumnConstraint::Unique => buffer.push("UNIQUE"),
            ColumnConstraint::Default(expr) => {
                buffer.push("DEFAULT");
                expr.to_source(buffer);
            }
            ColumnConstraint::Check(expr) => {
                buffer.push("CHECK");
                buffer.push("(");
                expr.to_source(buffer);
                buffer.push(")");
            }
            ColumnConstraint::AutoIncrement => buffer.push("AUTOINCREMENT"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CreateObjectType {
    Table,
    View,
    Index,
}

impl ToSource for CreateObjectType {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            CreateObjectType::Table => buffer.push("TABLE"),
            CreateObjectType::View => buffer.push("VIEW"),
            CreateObjectType::Index => buffer.push("INDEX"),
        }
    }
}

/// DROP 语句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DropStatement {
    pub object_type: DropObjectType,
    pub name: Identifier,
    pub if_exists: bool,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for DropStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("DROP");
        self.object_type.to_source(buffer);
        if self.if_exists {
            buffer.push("IF");
            buffer.push("EXISTS");
        }
        self.name.to_source(buffer);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DropObjectType {
    Table,
    View,
    Index,
}

impl ToSource for DropObjectType {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            DropObjectType::Table => buffer.push("TABLE"),
            DropObjectType::View => buffer.push("VIEW"),
            DropObjectType::Index => buffer.push("INDEX"),
        }
    }
}

/// ALTER 语句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AlterStatement {
    pub table_name: TableName,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for AlterStatement {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push("ALTER");
        buffer.push("TABLE");
        self.table_name.to_source(buffer);
        // TODO: Add alter actions (add column, rename, etc.)
    }
}

/// 表达式
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Binary {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    Unary {
        op: UnaryOperator,
        expr: Box<Expression>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    FunctionCall {
        name: Identifier,
        args: Vec<Expression>,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    InList {
        expr: Box<Expression>,
        list: Vec<Expression>,
        negated: bool,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    Between {
        expr: Box<Expression>,
        low: Box<Expression>,
        high: Box<Expression>,
        negated: bool,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

impl ToSource for Expression {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            Expression::Identifier(id) => id.to_source(buffer),
            Expression::Literal(lit) => lit.to_source(buffer),
            Expression::Binary { left, op, right, .. } => {
                left.to_source(buffer);
                op.to_source(buffer);
                right.to_source(buffer);
            }
            Expression::Unary { op, expr, .. } => {
                op.to_source(buffer);
                expr.to_source(buffer);
            }
            Expression::FunctionCall { name, args, .. } => {
                name.to_source(buffer);
                buffer.push("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        buffer.push(",");
                    }
                    arg.to_source(buffer);
                }
                buffer.push(")");
            }
            Expression::InList { expr, list, negated, .. } => {
                expr.to_source(buffer);
                if *negated {
                    buffer.push("NOT");
                }
                buffer.push("IN");
                buffer.push("(");
                for (i, item) in list.iter().enumerate() {
                    if i > 0 {
                        buffer.push(",");
                    }
                    item.to_source(buffer);
                }
                buffer.push(")");
            }
            Expression::Between { expr, low, high, negated, .. } => {
                expr.to_source(buffer);
                if *negated {
                    buffer.push("NOT");
                }
                buffer.push("BETWEEN");
                low.to_source(buffer);
                buffer.push("AND");
                high.to_source(buffer);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BinaryOperator {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    And,
    Or,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Like,
}

impl ToSource for BinaryOperator {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            BinaryOperator::Plus => buffer.push("+"),
            BinaryOperator::Minus => buffer.push("-"),
            BinaryOperator::Star => buffer.push("*"),
            BinaryOperator::Slash => buffer.push("/"),
            BinaryOperator::Percent => buffer.push("%"),
            BinaryOperator::And => buffer.push("AND"),
            BinaryOperator::Or => buffer.push("OR"),
            BinaryOperator::Equal => buffer.push("="),
            BinaryOperator::NotEqual => buffer.push("<>"),
            BinaryOperator::Less => buffer.push("<"),
            BinaryOperator::Greater => buffer.push(">"),
            BinaryOperator::LessEqual => buffer.push("<="),
            BinaryOperator::GreaterEqual => buffer.push(">="),
            BinaryOperator::Like => buffer.push("LIKE"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
}

impl ToSource for UnaryOperator {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            UnaryOperator::Plus => buffer.push("+"),
            UnaryOperator::Minus => buffer.push("-"),
            UnaryOperator::Not => buffer.push("NOT"),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Literal {
    Number(String, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] Range<usize>),
    String(String, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] Range<usize>),
    Boolean(bool, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] Range<usize>),
    Null(#[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] Range<usize>),
}

impl ToSource for Literal {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        match self {
            Literal::Number(n, _) => buffer.push(n),
            Literal::String(s, _) => {
                buffer.push("'");
                buffer.push(s);
                buffer.push("'");
            }
            Literal::Boolean(b, _) => buffer.push(if *b { "TRUE" } else { "FALSE" }),
            Literal::Null(_) => buffer.push("NULL"),
        }
    }
}

/// 标识符
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Identifier {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for Identifier {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        buffer.push(&self.name);
    }
}

/// 表名
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TableName {
    pub name: Identifier,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl ToSource for TableName {
    fn to_source(&self, buffer: &mut SourceBuffer) {
        self.name.to_source(buffer);
    }
}

/// JOIN 子句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JoinClause {
    pub join_type: JoinType,
    pub table: TableName,
    pub on: Option<Expression>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

/// GROUP BY 子句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GroupByClause {
    pub columns: Vec<Expression>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// HAVING 子句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HavingClause {
    pub condition: Expression,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// ORDER BY 子句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OrderByClause {
    pub items: Vec<OrderByItem>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OrderByItem {
    pub expr: Expression,
    pub direction: OrderDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OrderDirection {
    Asc,
    Desc,
}

/// LIMIT 子句
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LimitClause {
    pub limit: Expression,
    pub offset: Option<Expression>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
