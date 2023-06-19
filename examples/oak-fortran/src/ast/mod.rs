use std::{boxed::Box, string::String, vec::Vec};
use std::range::Range;
use serde::{Deserialize, Serialize};

/// Fortran AST 根节点
pub type FortranAst = ProgramNode;

/// 程序节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramNode {
    pub name: Option<String>,
    pub units: Vec<ProgramUnitKind>,
    pub span: Range<usize>,
}

/// Fortran 程序单元种类
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProgramUnitKind {
    /// 主程序
    MainProgram(MainProgramNode),
    /// 子程序
    Subroutine(SubroutineNode),
    /// 函数
    Function(FunctionNode),
    /// 模块
    Module(ModuleNode),
    /// 子模块
    Submodule(SubmoduleNode),
    /// 块数据
    BlockData(BlockDataNode),
}

/// 主程序节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MainProgramNode {
    pub name: Option<String>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub internal_subprograms: Vec<ProgramUnitKind>,
    pub span: Range<usize>,
}

/// 子程序节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubroutineNode {
    pub name: String,
    pub parameters: Vec<String>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub internal_subprograms: Vec<ProgramUnitKind>,
    pub span: Range<usize>,
}

/// 函数节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionNode {
    pub name: String,
    pub parameters: Vec<String>,
    pub result_name: Option<String>,
    pub return_type: Option<TypeSpec>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub internal_subprograms: Vec<ProgramUnitKind>,
    pub span: Range<usize>,
}

/// 模块节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleNode {
    pub name: String,
    pub specification_part: Vec<SpecificationStmt>,
    pub module_subprograms: Vec<ProgramUnitKind>,
    pub span: Range<usize>,
}

/// 子模块节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmoduleNode {
    pub parent_name: String,
    pub name: String,
    pub specification_part: Vec<SpecificationStmt>,
    pub module_subprograms: Vec<ProgramUnitKind>,
    pub span: Range<usize>,
}

/// 块数据节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockDataNode {
    pub name: Option<String>,
    pub specification_part: Vec<SpecificationStmt>,
    pub span: Range<usize>,
}

/// 规范语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpecificationStmt {
    /// 类型声明
    TypeDeclaration(TypeDeclarationNode),
    /// 参数声明
    Parameter(ParameterNode),
    /// 隐式声明
    Implicit(ImplicitNode),
    /// 使用语句
    Use(UseNode),
    /// 导入语句
    Import(ImportNode),
    /// 接口声明
    Interface(InterfaceNode),
    /// 过程声明
    Procedure(ProcedureNode),
    /// 泛型声明
    Generic(GenericNode),
}

/// 可执行语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutableStmt {
    /// 赋值语句
    Assignment(AssignmentNode),
    /// 调用语句
    Call(CallNode),
    /// If 构造
    IfConstruct(IfConstructNode),
    /// Do 循环
    DoConstruct(DoConstructNode),
    /// Select Case
    SelectCase(SelectCaseNode),
    /// Where 构造
    WhereConstruct(WhereConstructNode),
    /// Forall 构造
    ForallConstruct(ForallConstructNode),
    /// Associate 构造
    AssociateConstruct(AssociateConstructNode),
    /// Block 构造
    BlockConstruct(BlockConstructNode),
    /// Critical 构造
    CriticalConstruct(CriticalConstructNode),
    /// 分配语句
    Allocate(AllocateNode),
    /// 释放语句
    Deallocate(DeallocateNode),
    /// 空化语句
    Nullify(NullifyNode),
    /// 停止语句
    Stop(StopNode),
    /// 返回语句
    Return(ReturnNode),
    /// 继续语句
    Continue,
    /// 循环语句
    Cycle(Option<String>),
    /// 退出语句
    Exit(Option<String>),
    /// 读语句
    Read(ReadNode),
    /// 写语句
    Write(WriteNode),
    /// 打印语句
    Print(PrintNode),
}

/// 类型规范
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeSpec {
    /// 整数类型
    Integer(Option<KindSelector>),
    /// 实数类型
    Real(Option<KindSelector>),
    /// 双精度类型
    DoublePrecision,
    /// 复数类型
    Complex(Option<KindSelector>),
    /// 字符类型
    Character(Option<CharacterSelector>),
    /// 逻辑类型
    Logical(Option<KindSelector>),
    /// 派生类型
    Derived(String),
    /// 类类型
    Class(String),
    /// 类型星号
    TypeStar,
}

/// 种类选择器
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KindSelector {
    /// 种类参数
    Kind(ExprKind),
}

/// 字符选择器
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterSelector {
    pub length: Option<ExprKind>,
    pub kind: Option<KindSelector>,
}

/// 类型声明节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeDeclarationNode {
    pub type_spec: TypeSpec,
    pub attributes: Vec<AttributeSpec>,
    pub entities: Vec<EntityDecl>,
    pub span: Range<usize>,
}

/// 属性规范
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeSpec {
    /// 参数属性
    Parameter,
    /// 分配属性
    Allocatable,
    /// 异步属性
    Asynchronous,
    /// 绑定属性
    Bind(BindSpec),
    /// 数据属性
    Data,
    /// 维度属性
    Dimension(Vec<DimensionSpec>),
    /// 外部属性
    External,
    /// 意图属性
    Intent(IntentSpec),
    /// 内在属性
    Intrinsic,
    /// 可选属性
    Optional,
    /// 指针属性
    Pointer,
    /// 保护属性
    Protected,
    /// 保存属性
    Save,
    /// 目标属性
    Target,
    /// 值属性
    Value,
    /// 易失属性
    Volatile,
}

/// 意图规范
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IntentSpec {
    In,
    Out,
    InOut,
}

/// 维度规范
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DimensionSpec {
    pub lower_bound: Option<ExprKind>,
    pub upper_bound: Option<ExprKind>,
}

/// 绑定规范
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BindSpec {
    pub language: String,
    pub name: Option<String>,
}

/// 实体声明
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityDecl {
    pub name: String,
    pub array_spec: Option<Vec<DimensionSpec>>,
    pub char_length: Option<ExprKind>,
    pub initialization: Option<ExprKind>,
    pub span: Range<usize>,
}

/// 表达式种类
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExprKind {
    /// 标识符
    Identifier(String),
    /// 字面量
    Literal(LiteralKind),
    /// 数组引用
    ArrayRef(ArrayRefNode),
    /// 结构引用
    StructureRef(StructureRefNode),
    /// 函数调用
    FunctionCall(FunctionCallNode),
    /// 二元操作
    BinaryOp(BinaryOpNode),
    /// 一元操作
    UnaryOp(UnaryOpNode),
}

/// 字面量种类
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralKind {
    Integer(i64),
    Real(f64),
    String(String),
    Logical(bool),
}

/// 数组引用节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayRefNode {
    pub array: Box<ExprKind>,
    pub subscripts: Vec<SubscriptKind>,
    pub span: Range<usize>,
}

/// 下标种类
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SubscriptKind {
    /// 表达式下标
    Expression(ExprKind),
    /// 范围下标
    Range(Option<ExprKind>, Option<ExprKind>, Option<ExprKind>),
}

/// 结构引用节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructureRefNode {
    pub structure: Box<ExprKind>,
    pub component: String,
    pub span: Range<usize>,
}

/// 函数调用节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCallNode {
    pub function: Box<ExprKind>,
    pub arguments: Vec<ExprKind>,
    pub span: Range<usize>,
}

/// 二元操作节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryOpNode {
    pub left: Box<ExprKind>,
    pub operator: BinaryOperator,
    pub right: Box<ExprKind>,
    pub span: Range<usize>,
}

/// 二元操作符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    /// 加法
    Add,
    /// 减法
    Subtract,
    /// 乘法
    Multiply,
    /// 除法
    Divide,
    /// 幂运算
    Power,
    /// 字符串连接
    Concatenate,
    /// 等于
    Equal,
    /// 不等于
    NotEqual,
    /// 小于
    LessThan,
    /// 小于等于
    LessEqual,
    /// 大于
    GreaterThan,
    /// 大于等于
    GreaterEqual,
    /// 逻辑与
    And,
    /// 逻辑或
    Or,
    /// 等价
    Equivalent,
    /// 不等价
    NotEquivalent,
}

/// 一元操作节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryOpNode {
    pub operator: UnaryOperator,
    pub operand: Box<ExprKind>,
    pub span: Range<usize>,
}

/// 一元操作符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    /// 正号
    Plus,
    /// 负号
    Minus,
    /// 逻辑非
    Not,
}

/// 赋值节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssignmentNode {
    pub target: ExprKind,
    pub value: ExprKind,
    pub span: Range<usize>,
}

/// 指针赋值节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerAssignmentNode {
    pub target: ExprKind,
    pub value: ExprKind,
    pub span: Range<usize>,
}

/// 调用节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallNode {
    pub procedure: String,
    pub arguments: Vec<ExprKind>,
    pub span: Range<usize>,
}

/// If 构造节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfConstructNode {
    pub condition: ExprKind,
    pub then_part: Vec<ExecutableStmt>,
    pub else_if_parts: Vec<ElseIfNode>,
    pub else_part: Option<Vec<ExecutableStmt>>,
    pub span: Range<usize>,
}

/// ElseIf 节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElseIfNode {
    pub condition: ExprKind,
    pub statements: Vec<ExecutableStmt>,
    pub span: Range<usize>,
}

/// Do 构造节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoConstructNode {
    pub label: Option<String>,
    pub control: Option<DoControl>,
    pub body: Vec<ExecutableStmt>,
    pub span: Range<usize>,
}

/// Do 控制
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DoControl {
    /// 计数器控制
    Counter(CounterControl),
    /// While 控制
    While(ExprKind),
}

/// 计数器控制
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CounterControl {
    pub variable: String,
    pub start: ExprKind,
    pub end: ExprKind,
    pub step: Option<ExprKind>,
}

/// Select Case 节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectCaseNode {
    pub expression: ExprKind,
    pub cases: Vec<CaseNode>,
    pub default_case: Option<Vec<ExecutableStmt>>,
    pub span: Range<usize>,
}

/// Case 节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaseNode {
    pub selectors: Vec<CaseSelector>,
    pub statements: Vec<ExecutableStmt>,
    pub span: Range<usize>,
}

/// Case 选择器
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CaseSelector {
    /// 表达式选择器
    Expression(ExprKind),
    /// 范围选择器
    Range(Option<ExprKind>, Option<ExprKind>),
}

/// Where 构造节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhereConstructNode {
    pub mask: ExprKind,
    pub statements: Vec<ExecutableStmt>,
    pub elsewhere_parts: Vec<ElsewhereNode>,
    pub span: Range<usize>,
}

/// Elsewhere 节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElsewhereNode {
    pub mask: Option<ExprKind>,
    pub statements: Vec<ExecutableStmt>,
    pub span: Range<usize>,
}

/// Forall 构造节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForallConstructNode {
    pub header: ForallHeader,
    pub statements: Vec<ExecutableStmt>,
    pub span: Range<usize>,
}

/// Forall 头部
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForallHeader {
    pub triplets: Vec<ForallTriplet>,
    pub mask: Option<ExprKind>,
}

/// Forall 三元组
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForallTriplet {
    pub variable: String,
    pub start: ExprKind,
    pub end: ExprKind,
    pub step: Option<ExprKind>,
}

/// Associate 构造节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssociateConstructNode {
    pub associations: Vec<Association>,
    pub statements: Vec<ExecutableStmt>,
    pub span: Range<usize>,
}

/// 关联
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Association {
    pub name: String,
    pub selector: ExprKind,
}

/// Block 构造节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockConstructNode {
    pub label: Option<String>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub span: Range<usize>,
}

/// Critical 构造节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CriticalConstructNode {
    pub label: Option<String>,
    pub statements: Vec<ExecutableStmt>,
    pub span: Range<usize>,
}

/// 分配节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllocateNode {
    pub objects: Vec<ExprKind>,
    pub type_spec: Option<TypeSpec>,
    pub source: Option<ExprKind>,
    pub mold: Option<ExprKind>,
    pub stat: Option<String>,
    pub errmsg: Option<String>,
    pub span: Range<usize>,
}

/// 释放节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeallocateNode {
    pub objects: Vec<ExprKind>,
    pub stat: Option<String>,
    pub errmsg: Option<String>,
    pub span: Range<usize>,
}

/// 空化节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NullifyNode {
    pub objects: Vec<ExprKind>,
    pub span: Range<usize>,
}

/// 停止节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopNode {
    pub code: Option<ExprKind>,
    pub quiet: bool,
    pub span: Range<usize>,
}

/// 返回节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnNode {
    pub expression: Option<ExprKind>,
    pub span: Range<usize>,
}

/// 读节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadNode {
    pub unit: Option<ExprKind>,
    pub format: Option<ExprKind>,
    pub items: Vec<ExprKind>,
    pub iostat: Option<String>,
    pub err: Option<String>,
    pub end: Option<String>,
    pub span: Range<usize>,
}

/// 写节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteNode {
    pub unit: Option<ExprKind>,
    pub format: Option<ExprKind>,
    pub items: Vec<ExprKind>,
    pub iostat: Option<String>,
    pub err: Option<String>,
    pub span: Range<usize>,
}

/// 打印节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintNode {
    pub format: Option<ExprKind>,
    pub items: Vec<ExprKind>,
    pub span: Range<usize>,
}

/// 参数节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterNode {
    pub assignments: Vec<ParameterAssignment>,
    pub span: Range<usize>,
}

/// 参数赋值
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterAssignment {
    pub name: String,
    pub value: ExprKind,
}

/// 隐式节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImplicitNode {
    /// 隐式无
    None,
    /// 隐式规则
    Rules(Vec<ImplicitRule>),
}

/// 隐式规则
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImplicitRule {
    pub type_spec: TypeSpec,
    pub letter_specs: Vec<LetterSpec>,
}

/// 字母规范
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetterSpec {
    pub start: char,
    pub end: Option<char>,
}

/// 使用节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseNode {
    pub module_name: String,
    pub nature: Option<String>,
    pub only_list: Option<Vec<UseItem>>,
    pub rename_list: Option<Vec<UseRename>>,
    pub span: Range<usize>,
}

/// 使用项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UseItem {
    /// 操作符
    Operator(String),
    /// 名称
    Name(String),
}

/// 使用重命名
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseRename {
    pub local_name: String,
    pub use_name: String,
}

/// 导入节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportNode {
    pub items: Vec<String>,
    pub span: Range<usize>,
}

/// 接口节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceNode {
    pub generic_spec: Option<String>,
    pub interface_body: Vec<InterfaceBody>,
    pub span: Range<usize>,
}

/// 接口体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterfaceBody {
    /// 子程序
    Subroutine(SubroutineNode),
    /// 过程
    Procedure(ProcedureNode),
    /// 函数
    Function(FunctionNode),
}

/// 过程节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcedureNode {
    pub names: Vec<String>,
    pub interface: Option<String>,
    pub attributes: Vec<AttributeSpec>,
    pub span: Range<usize>,
}

/// 泛型节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericNode {
    pub spec: String,
    pub procedures: Vec<String>,
    pub span: Range<usize>,
}

/// 终结节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinalNode {
    pub procedures: Vec<String>,
    pub span: Range<usize>,
}
