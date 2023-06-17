use alloc::{boxed::Box, string::String, vec::Vec};

/// Fortran AST 根节pub type FortranAst = ProgramNode;

/// 程序节点
#[derive(Debug, Clone, PartialEq)]
pub struct ProgramNode {
    pub name: Option<String>,
    pub units: Vec<ProgramUnitKind>,
}

/// Fortran 程序单元种类
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramUnitKind {
    /// 主程    MainProgram(MainProgramNode),
    /// 子程    Subroutine(SubroutineNode),
    /// 函数
    Function(FunctionNode),
    /// 模块
    Module(ModuleNode),
    /// 子模    Submodule(SubmoduleNode),
    /// 块数    BlockData(BlockDataNode),
}

/// 主程序节#[derive(Debug, Clone, PartialEq)]
pub struct MainProgramNode {
    pub name: Option<String>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub internal_subprograms: Vec<ProgramUnitKind>,
}

/// 子程序节#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineNode {
    pub name: String,
    pub parameters: Vec<String>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub internal_subprograms: Vec<ProgramUnitKind>,
}

/// 函数节点
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionNode {
    pub name: String,
    pub parameters: Vec<String>,
    pub result_name: Option<String>,
    pub return_type: Option<TypeSpec>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub internal_subprograms: Vec<ProgramUnitKind>,
}

/// 模块节点
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleNode {
    pub name: String,
    pub specification_part: Vec<SpecificationStmt>,
    pub module_subprograms: Vec<ProgramUnitKind>,
}

/// 子模块节#[derive(Debug, Clone, PartialEq)]
pub struct SubmoduleNode {
    pub parent_name: String,
    pub name: String,
    pub specification_part: Vec<SpecificationStmt>,
    pub module_subprograms: Vec<ProgramUnitKind>,
}

/// 块数据节#[derive(Debug, Clone, PartialEq)]
pub struct BlockDataNode {
    pub name: Option<String>,
    pub specification_part: Vec<SpecificationStmt>,
}

/// 规范语句
#[derive(Debug, Clone, PartialEq)]
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
    /// 接口    Interface(InterfaceNode),
    /// 过程声明
    Procedure(ProcedureNode),
    /// 通用声明
    Generic(GenericNode),
    /// 最终声    Final(FinalNode),
}

/// 可执行语#[derive(Debug, Clone, PartialEq)]
pub enum ExecutableStmt {
    /// 赋值语    Assignment(AssignmentNode),
    /// 指针   PointerAssignment(PointerAssignmentNode),
    /// 过程调用
    Call(CallNode),
    /// If    If(IfConstructNode),
    /// Select Case    SelectCase(SelectCaseNode),
    /// Do    Do(DoConstructNode),
    /// Where    Where(WhereConstructNode),
    /// Forall    Forall(ForallConstructNode),
    /// Associate    Associate(AssociateConstructNode),
    /// Block    Block(BlockConstructNode),
    /// Critical    Critical(CriticalConstructNode),
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
    /// 退出语    Exit(Option<String>),
    /// 读语    Read(ReadNode),
    /// 写语    Write(WriteNode),
    /// 打印语句
    Print(PrintNode),
}

/// 类型规范
#[derive(Debug, Clone, PartialEq)]
pub enum TypeSpec {
    /// 整数类型
    Integer(Option<KindSelector>),
    /// 实数类型
    Real(Option<KindSelector>),
    /// 双精度类    DoublePrecision,
    /// 复数类型
    Complex(Option<KindSelector>),
    /// 字符类型
    Character(Option<CharacterSelector>),
    /// 逻辑类型
    Logical(Option<KindSelector>),
    /// 派生类型
    Derived(String),
    /// 类类    Class(String),
    /// 类型(*) 
    TypeStar,
}

/// 种类选择#[derive(Debug, Clone, PartialEq)]
pub enum KindSelector {
    /// 种类参数
    Kind(ExprKind),
    /// 字节    Bytes(i32),
}

/// 字符选择#[derive(Debug, Clone, PartialEq)]
pub struct CharacterSelector {
    pub length: Option<ExprKind>,
    pub kind: Option<KindSelector>,
}

/// 类型声明节点
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclarationNode {
    pub type_spec: TypeSpec,
    pub attributes: Vec<AttributeSpec>,
    pub entities: Vec<EntityDecl>,
}

/// 属性规#[derive(Debug, Clone, PartialEq)]
pub enum AttributeSpec {
    /// 参数   Parameter,
    /// 分配   Allocatable,
    /// 指针   Pointer,
    /// 目标   Target,
    /// 可选属    Optional,
    /// 意图   Intent(IntentSpec),
    /// 维度   Dimension(Vec<DimensionSpec>),
    /// 保存   Save,
    /// 外部   External,
    /// 内在   Intrinsic,
    /// 私有   Private,
    /// 公共   Public,
    /// 保护   Protected,
    /// 值属    Value,
    /// 易失   Volatile,
    /// 绑定   Bind(BindSpec),
}

/// 意图规范
#[derive(Debug, Clone, PartialEq)]
pub enum IntentSpec {
    In,
    Out,
    InOut,
}

/// 维度规范
#[derive(Debug, Clone, PartialEq)]
pub struct DimensionSpec {
    pub lower_bound: Option<ExprKind>,
    pub upper_bound: Option<ExprKind>,
}

/// 绑定规范
#[derive(Debug, Clone, PartialEq)]
pub struct BindSpec {
    pub language: String,
    pub name: Option<String>,
}

/// 实体声明
#[derive(Debug, Clone, PartialEq)]
pub struct EntityDecl {
    pub name: String,
    pub array_spec: Option<Vec<DimensionSpec>>,
    pub char_length: Option<ExprKind>,
    pub initialization: Option<ExprKind>,
}

/// 表达式种#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    /// 整数字面    IntegerLiteral(i64),
    /// 实数字面    RealLiteral(f64),
    /// 字符字面    CharacterLiteral(String),
    /// 逻辑字面    LogicalLiteral(bool),
    /// 标识    Identifier(String),
    /// 数组引用
    ArrayRef(ArrayRefNode),
    /// 结构引用
    StructureRef(StructureRefNode),
    /// 函数调用
    FunctionCall(FunctionCallNode),
    /// 二元操作
    BinaryOp(BinaryOpNode),
    /// 一元操    UnaryOp(UnaryOpNode),
    /// 括号表达    Parentheses(Box<ExprKind>),
}

/// 数组引用节点
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayRefNode {
    pub array: Box<ExprKind>,
    pub subscripts: Vec<SubscriptKind>,
}

/// 下标种类
#[derive(Debug, Clone, PartialEq)]
pub enum SubscriptKind {
    /// 单个表达    Expression(ExprKind),
    /// 范围
    Range(Option<ExprKind>, Option<ExprKind>, Option<ExprKind>),
}

/// 结构引用节点
#[derive(Debug, Clone, PartialEq)]
pub struct StructureRefNode {
    pub structure: Box<ExprKind>,
    pub component: String,
}

/// 函数调用节点
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCallNode {
    pub function: Box<ExprKind>,
    pub arguments: Vec<ExprKind>,
}

/// 二元操作节点
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOpNode {
    pub left: Box<ExprKind>,
    pub operator: BinaryOperator,
    pub right: Box<ExprKind>,
}

/// 二元操作#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    /// 加法
    Add,
    /// 减法
    Subtract,
    /// 乘法
    Multiply,
    /// 除法
    Divide,
    /// 幂运    Power,
    /// 连接
    Concatenate,
    /// 等于
    Equal,
    /// 不等    NotEqual,
    /// 小于
    LessThan,
    /// 小于等于
    LessEqual,
    /// 大于
    GreaterThan,
    /// 大于等于
    GreaterEqual,
    /// 逻辑    And,
    /// 逻辑    Or,
    /// 逻辑等价
    Equivalent,
    /// 逻辑不等    NotEquivalent,
}

/// 一元操作节#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOpNode {
    pub operator: UnaryOperator,
    pub operand: Box<ExprKind>,
}

/// 一元操作符
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    /// 正号
    Plus,
    /// 负号
    Minus,
    /// 逻辑    Not,
}

/// 赋值节#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentNode {
    pub target: ExprKind,
    pub value: ExprKind,
}

/// 指针赋值节#[derive(Debug, Clone, PartialEq)]
pub struct PointerAssignmentNode {
    pub target: ExprKind,
    pub value: ExprKind,
}

/// 调用节点
#[derive(Debug, Clone, PartialEq)]
pub struct CallNode {
    pub procedure: String,
    pub arguments: Vec<ExprKind>,
}

/// If 构造节#[derive(Debug, Clone, PartialEq)]
pub struct IfConstructNode {
    pub condition: ExprKind,
    pub then_part: Vec<ExecutableStmt>,
    pub else_if_parts: Vec<ElseIfNode>,
    pub else_part: Option<Vec<ExecutableStmt>>,
}

/// ElseIf 节点
#[derive(Debug, Clone, PartialEq)]
pub struct ElseIfNode {
    pub condition: ExprKind,
    pub statements: Vec<ExecutableStmt>,
}

/// Do 构造节#[derive(Debug, Clone, PartialEq)]
pub struct DoConstructNode {
    pub label: Option<String>,
    pub control: Option<DoControl>,
    pub body: Vec<ExecutableStmt>,
}

/// Do 控制
#[derive(Debug, Clone, PartialEq)]
pub enum DoControl {
    /// 计数控制
    Counter(CounterControl),
    /// While 控制
    While(ExprKind),
}

/// 计数器控#[derive(Debug, Clone, PartialEq)]
pub struct CounterControl {
    pub variable: String,
    pub start: ExprKind,
    pub end: ExprKind,
    pub step: Option<ExprKind>,
}

/// Select Case 构造节#[derive(Debug, Clone, PartialEq)]
pub struct SelectCaseNode {
    pub expression: ExprKind,
    pub cases: Vec<CaseNode>,
    pub default_case: Option<Vec<ExecutableStmt>>,
}

/// Case 节点
#[derive(Debug, Clone, PartialEq)]
pub struct CaseNode {
    pub selectors: Vec<CaseSelector>,
    pub statements: Vec<ExecutableStmt>,
}

/// Case 选择#[derive(Debug, Clone, PartialEq)]
pub enum CaseSelector {
    /// 单个    Value(ExprKind),
    /// 范围
    Range(Option<ExprKind>, Option<ExprKind>),
}

/// Where 构造节#[derive(Debug, Clone, PartialEq)]
pub struct WhereConstructNode {
    pub mask: ExprKind,
    pub statements: Vec<ExecutableStmt>,
    pub elsewhere_parts: Vec<ElsewhereNode>,
}

/// Elsewhere 节点
#[derive(Debug, Clone, PartialEq)]
pub struct ElsewhereNode {
    pub mask: Option<ExprKind>,
    pub statements: Vec<ExecutableStmt>,
}

/// Forall 构造节#[derive(Debug, Clone, PartialEq)]
pub struct ForallConstructNode {
    pub header: ForallHeader,
    pub statements: Vec<ExecutableStmt>,
}

/// Forall 头部
#[derive(Debug, Clone, PartialEq)]
pub struct ForallHeader {
    pub triplets: Vec<ForallTriplet>,
    pub mask: Option<ExprKind>,
}

/// Forall 三元#[derive(Debug, Clone, PartialEq)]
pub struct ForallTriplet {
    pub variable: String,
    pub start: ExprKind,
    pub end: ExprKind,
    pub step: Option<ExprKind>,
}

/// Associate 构造节#[derive(Debug, Clone, PartialEq)]
pub struct AssociateConstructNode {
    pub associations: Vec<Association>,
    pub statements: Vec<ExecutableStmt>,
}

/// 关联
#[derive(Debug, Clone, PartialEq)]
pub struct Association {
    pub name: String,
    pub selector: ExprKind,
}

/// Block 构造节#[derive(Debug, Clone, PartialEq)]
pub struct BlockConstructNode {
    pub label: Option<String>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
}

/// Critical 构造节#[derive(Debug, Clone, PartialEq)]
pub struct CriticalConstructNode {
    pub label: Option<String>,
    pub statements: Vec<ExecutableStmt>,
}

/// 分配节点
#[derive(Debug, Clone, PartialEq)]
pub struct AllocateNode {
    pub objects: Vec<ExprKind>,
    pub type_spec: Option<TypeSpec>,
    pub source: Option<ExprKind>,
    pub mold: Option<ExprKind>,
    pub stat: Option<String>,
    pub errmsg: Option<String>,
}

/// 释放节点
#[derive(Debug, Clone, PartialEq)]
pub struct DeallocateNode {
    pub objects: Vec<ExprKind>,
    pub stat: Option<String>,
    pub errmsg: Option<String>,
}

/// 空化节点
#[derive(Debug, Clone, PartialEq)]
pub struct NullifyNode {
    pub objects: Vec<ExprKind>,
}

/// 停止节点
#[derive(Debug, Clone, PartialEq)]
pub struct StopNode {
    pub code: Option<ExprKind>,
    pub quiet: bool,
}

/// 返回节点
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnNode {
    pub expression: Option<ExprKind>,
}

/// 读节#[derive(Debug, Clone, PartialEq)]
pub struct ReadNode {
    pub unit: Option<ExprKind>,
    pub format: Option<ExprKind>,
    pub items: Vec<ExprKind>,
    pub iostat: Option<String>,
    pub err: Option<String>,
    pub end: Option<String>,
}

/// 写节#[derive(Debug, Clone, PartialEq)]
pub struct WriteNode {
    pub unit: Option<ExprKind>,
    pub format: Option<ExprKind>,
    pub items: Vec<ExprKind>,
    pub iostat: Option<String>,
    pub err: Option<String>,
}

/// 打印节点
#[derive(Debug, Clone, PartialEq)]
pub struct PrintNode {
    pub format: Option<ExprKind>,
    pub items: Vec<ExprKind>,
}

/// 参数节点
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterNode {
    pub assignments: Vec<ParameterAssignment>,
}

/// 参数[derive(Debug, Clone, PartialEq)]
pub struct ParameterAssignment {
    pub name: String,
    pub value: ExprKind,
}

/// 隐式节点
#[derive(Debug, Clone, PartialEq)]
pub enum ImplicitNode {
    /// 隐式    None,
    /// 隐式规则
    Rules(Vec<ImplicitRule>),
}

/// 隐式规则
#[derive(Debug, Clone, PartialEq)]
pub struct ImplicitRule {
    pub type_spec: TypeSpec,
    pub letter_specs: Vec<LetterSpec>,
}

/// 字母规范
#[derive(Debug, Clone, PartialEq)]
pub struct LetterSpec {
    pub start: char,
    pub end: Option<char>,
}

/// 使用节点
#[derive(Debug, Clone, PartialEq)]
pub struct UseNode {
    pub module_name: String,
    pub nature: Option<String>,
    pub only_list: Option<Vec<UseItem>>,
    pub rename_list: Option<Vec<UseRename>>,
}

/// 使用#[derive(Debug, Clone, PartialEq)]
pub enum UseItem {
    /// 名称
    Name(String),
    /// 重命    Rename(UseRename),
}

/// 使用重命#[derive(Debug, Clone, PartialEq)]
pub struct UseRename {
    pub local_name: String,
    pub use_name: String,
}

/// 导入节点
#[derive(Debug, Clone, PartialEq)]
pub struct ImportNode {
    pub items: Vec<String>,
}

/// 接口节点
#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceNode {
    pub generic_spec: Option<String>,
    pub interface_body: Vec<InterfaceBody>,
}

/// 接口#[derive(Debug, Clone, PartialEq)]
pub enum InterfaceBody {
    /// 过程声明
    Procedure(ProcedureNode),
    /// 函数声明
    Function(FunctionNode),
    /// 子程序声    Subroutine(SubroutineNode),
}

/// 过程节点
#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureNode {
    pub names: Vec<String>,
    pub interface: Option<String>,
    pub attributes: Vec<AttributeSpec>,
}

/// 通用节点
#[derive(Debug, Clone, PartialEq)]
pub struct GenericNode {
    pub spec: String,
    pub procedures: Vec<String>,
}

/// 最终节#[derive(Debug, Clone, PartialEq)]
pub struct FinalNode {
    pub procedures: Vec<String>,
}
