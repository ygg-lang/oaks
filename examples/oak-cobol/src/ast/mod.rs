use crate::{kind::CobolSyntaxKind, language::CobolLanguage};
use oak_core::tree::{RedLeaf, RedNode};

pub type CobolNode = RedNode<CobolSyntaxKind>;
pub type CobolToken = RedLeaf<CobolSyntaxKind>;

#[derive(Debug, Clone)]
pub struct CobolRoot {
    pub syntax: CobolNode,
}

impl CobolRoot {
    pub fn new(syntax: CobolNode) -> Self {
        Self { syntax }
    }

    pub fn kind(&self) -> CobolSyntaxKind {
        self.syntax.green.kind
    }
}

/// COBOL 程序结构
#[derive(Debug, Clone)]
pub struct CobolProgram {
    pub identification_division: Option<IdentificationDivision>,
    pub environment_division: Option<EnvironmentDivision>,
    pub data_division: Option<DataDivision>,
    pub procedure_division: Option<ProcedureDivision>,
}

/// 标识部
#[derive(Debug, Clone)]
pub struct IdentificationDivision {
    pub program_id: String,
}

/// 环境部
#[derive(Debug, Clone)]
pub struct EnvironmentDivision {
    pub configuration_section: Option<ConfigurationSection>,
    pub input_output_section: Option<InputOutputSection>,
}

/// 配置节
#[derive(Debug, Clone)]
pub struct ConfigurationSection {
    pub source_computer: Option<String>,
    pub object_computer: Option<String>,
}

/// 输入输出节
#[derive(Debug, Clone)]
pub struct InputOutputSection {
    pub file_control: Vec<FileControlEntry>,
}

/// 文件控制项
#[derive(Debug, Clone)]
pub struct FileControlEntry {
    pub file_name: String,
    pub assign_clause: String,
    pub organization: Option<String>,
    pub access_mode: Option<String>,
}

/// 数据部
#[derive(Debug, Clone)]
pub struct DataDivision {
    pub file_section: Option<FileSection>,
    pub working_storage_section: Option<WorkingStorageSection>,
    pub linkage_section: Option<LinkageSection>,
}

/// 文件节
#[derive(Debug, Clone)]
pub struct FileSection {
    pub file_descriptions: Vec<FileDescription>,
}

/// 文件描述
#[derive(Debug, Clone)]
pub struct FileDescription {
    pub file_name: String,
    pub record_descriptions: Vec<RecordDescription>,
}

/// 工作存储节
#[derive(Debug, Clone)]
pub struct WorkingStorageSection {
    pub data_items: Vec<DataItem>,
}

/// 联结节
#[derive(Debug, Clone)]
pub struct LinkageSection {
    pub data_items: Vec<DataItem>,
}

/// 记录描述
#[derive(Debug, Clone)]
pub struct RecordDescription {
    pub level: u8,
    pub name: String,
    pub picture: Option<String>,
    pub value: Option<String>,
    pub occurs: Option<u32>,
    pub redefines: Option<String>,
}

/// 数据项
#[derive(Debug, Clone)]
pub struct DataItem {
    pub level: u8,
    pub name: String,
    pub picture: Option<String>,
    pub value: Option<String>,
    pub occurs: Option<u32>,
    pub redefines: Option<String>,
    pub usage: Option<String>,
}

/// 过程部
#[derive(Debug, Clone)]
pub struct ProcedureDivision {
    pub sections: Vec<Section>,
    pub paragraphs: Vec<Paragraph>,
}

/// 节
#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub paragraphs: Vec<Paragraph>,
}

/// 段落
#[derive(Debug, Clone)]
pub struct Paragraph {
    pub name: String,
    pub statements: Vec<Statement>,
}

/// 语句
#[derive(Debug, Clone)]
pub enum Statement {
    Accept(AcceptStatement),
    Add(AddStatement),
    Call(CallStatement),
    Compute(ComputeStatement),
    Display(DisplayStatement),
    Divide(DivideStatement),
    Evaluate(EvaluateStatement),
    If(IfStatement),
    Move(MoveStatement),
    Multiply(MultiplyStatement),
    Perform(PerformStatement),
    Read(ReadStatement),
    Stop(StopStatement),
    Subtract(SubtractStatement),
    Write(WriteStatement),
}

/// ACCEPT 语句
#[derive(Debug, Clone)]
pub struct AcceptStatement {
    pub target: String,
    pub from: Option<String>,
}

/// ADD 语句
#[derive(Debug, Clone)]
pub struct AddStatement {
    pub operands: Vec<String>,
    pub to: Option<String>,
    pub giving: Option<String>,
}

/// CALL 语句
#[derive(Debug, Clone)]
pub struct CallStatement {
    pub program_name: String,
    pub using: Vec<String>,
}

/// COMPUTE 语句
#[derive(Debug, Clone)]
pub struct ComputeStatement {
    pub target: String,
    pub expression: Expression,
}

/// DISPLAY 语句
#[derive(Debug, Clone)]
pub struct DisplayStatement {
    pub items: Vec<String>,
}

/// DIVIDE 语句
#[derive(Debug, Clone)]
pub struct DivideStatement {
    pub dividend: String,
    pub divisor: String,
    pub quotient: Option<String>,
    pub remainder: Option<String>,
}

/// EVALUATE 语句
#[derive(Debug, Clone)]
pub struct EvaluateStatement {
    pub subject: String,
    pub when_clauses: Vec<WhenClause>,
    pub when_other: Option<Vec<Statement>>,
}

/// WHEN 子句
#[derive(Debug, Clone)]
pub struct WhenClause {
    pub condition: String,
    pub statements: Vec<Statement>,
}

/// IF 语句
#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Condition,
    pub then_statements: Vec<Statement>,
    pub else_statements: Option<Vec<Statement>>,
}

/// 条件
#[derive(Debug, Clone)]
pub enum Condition {
    Comparison { left: String, operator: ComparisonOperator, right: String },
    Logical { left: Box<Condition>, operator: LogicalOperator, right: Box<Condition> },
    Not(Box<Condition>),
}

/// 比较操作符
#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

/// 逻辑操作符
#[derive(Debug, Clone)]
pub enum LogicalOperator {
    And,
    Or,
}

/// MOVE 语句
#[derive(Debug, Clone)]
pub struct MoveStatement {
    pub source: String,
    pub targets: Vec<String>,
}

/// MULTIPLY 语句
#[derive(Debug, Clone)]
pub struct MultiplyStatement {
    pub multiplicand: String,
    pub multiplier: String,
    pub product: Option<String>,
}

/// PERFORM 语句
#[derive(Debug, Clone)]
pub struct PerformStatement {
    pub target: String,
    pub times: Option<u32>,
    pub until: Option<Condition>,
    pub varying: Option<VaryingClause>,
}

/// VARYING 子句
#[derive(Debug, Clone)]
pub struct VaryingClause {
    pub variable: String,
    pub from: String,
    pub by: String,
    pub until: Condition,
}

/// READ 语句
#[derive(Debug, Clone)]
pub struct ReadStatement {
    pub file_name: String,
    pub into: Option<String>,
    pub key: Option<String>,
}

/// STOP 语句
#[derive(Debug, Clone)]
pub struct StopStatement {
    pub run: bool,
}

/// SUBTRACT 语句
#[derive(Debug, Clone)]
pub struct SubtractStatement {
    pub subtrahends: Vec<String>,
    pub from: String,
    pub giving: Option<String>,
}

/// WRITE 语句
#[derive(Debug, Clone)]
pub struct WriteStatement {
    pub record_name: String,
    pub from: Option<String>,
}

/// 表达式
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    Binary { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression> },
    Unary { operator: UnaryOperator, operand: Box<Expression> },
}

/// 字面量
#[derive(Debug, Clone)]
pub enum Literal {
    Numeric(String),
    String(String),
    Figurative(FigurativeConstant),
}

/// 形象常数
#[derive(Debug, Clone)]
pub enum FigurativeConstant {
    Zero,
    Space,
    HighValue,
    LowValue,
    Quote,
    All(String),
}

/// 二元操作符
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

/// 一元操作符
#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
}
