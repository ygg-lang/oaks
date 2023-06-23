#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{boxed::Box, string::String, vec::Vec};

/// 程序节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FortranRoot {
    pub name: Option<String>,
    pub units: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Fortran 程序单元种类
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MainProgramNode {
    pub name: Option<String>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub internal_subprograms: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 子程序节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubroutineNode {
    pub name: String,
    pub parameters: Vec<String>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub internal_subprograms: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 函数节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionNode {
    pub name: String,
    pub parameters: Vec<String>,
    pub result_name: Option<String>,
    pub return_type: Option<TypeSpec>,
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub internal_subprograms: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 模块节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModuleNode {
    pub name: String,
    pub specification_part: Vec<SpecificationStmt>,
    pub module_subprograms: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 子模块节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubmoduleNode {
    pub parent_name: String,
    pub name: String,
    pub specification_part: Vec<SpecificationStmt>,
    pub module_subprograms: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 块数据节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BlockDataNode {
    pub name: Option<String>,
    pub specification_part: Vec<SpecificationStmt>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 规范语句
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KindSelector {
    /// 表达式
    Expression(Box<ExprNode>),
}

/// 字符选择器
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CharacterSelector {
    /// 长度
    Length(Box<ExprNode>),
    /// 长度和种类
    LengthAndKind(Box<ExprNode>, Box<ExprNode>),
}

/// 类型声明节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeDeclarationNode {
    pub type_spec: TypeSpec,
    pub attributes: Vec<Attribute>,
    pub entities: Vec<EntityDecl>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 属性
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Attribute {
    /// 可选
    Allocatable,
    /// 异步
    Asynchronous,
    /// 绑定
    Bind(String),
    /// 维度
    Dimension(Vec<Dimension>),
    /// 外部
    External,
    /// 意图
    Intent(Intent),
    /// 内部
    Intrinsic,
    /// 可选
    Optional,
    /// 参数
    Parameter,
    /// 指针
    Pointer,
    /// 保护
    Protected,
    /// 私有
    Private,
    /// 公有
    Public,
    /// 保存
    Save,
    /// 目标
    Target,
    /// 易变
    Value,
    /// 易变
    Volatile,
}

/// 意图
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Intent {
    /// 输入
    In,
    /// 输出
    Out,
    /// 输入输出
    InOut,
}

/// 维度
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Dimension {
    /// 显式形状
    Explicit(Box<ExprNode>, Box<ExprNode>),
    /// 假定形状
    Assumed(Option<Box<ExprNode>>),
    /// 延迟形状
    Deferred,
    /// 假定大小
    AssumedSize(Option<Box<ExprNode>>),
    /// 假定等级
    AssumedRank,
}

/// 实体声明
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EntityDecl {
    pub name: String,
    pub array_spec: Option<Vec<Dimension>>,
    pub char_length: Option<Box<ExprNode>>,
    pub initialization: Option<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 参数节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ParameterNode {
    pub entities: Vec<EntityDecl>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 隐式节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ImplicitNode {
    /// None
    None,
    /// 规范
    Spec(Vec<ImplicitSpec>),
}

/// 隐式规范
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImplicitSpec {
    pub type_spec: TypeSpec,
    pub letter_ranges: Vec<LetterRange>,
}

/// 字母范围
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LetterRange {
    pub start: char,
    pub end: Option<char>,
}

/// 使用节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UseNode {
    pub module_name: String,
    pub nature: Option<ModuleNature>,
    pub rename_list: Vec<Rename>,
    pub only_list: Vec<Only>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 模块性质
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ModuleNature {
    /// 内部
    Intrinsic,
    /// 非内部
    NonIntrinsic,
}

/// 重命名
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rename {
    pub local_name: String,
    pub use_name: String,
}

/// 仅
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Only {
    /// 泛型
    Generic(String),
    /// 重命名
    Rename(Rename),
}

/// 导入节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImportNode {
    pub import_names: Vec<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 接口节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InterfaceNode {
    pub generic_spec: Option<GenericSpec>,
    pub interface_bodies: Vec<ProgramUnitKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 泛型规范
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GenericSpec {
    /// 泛型名称
    GenericName(String),
    /// 运算符
    Operator(String),
    /// 赋值
    Assignment,
    /// 读取定义
    ReadDefined,
    /// 写入定义
    WriteDefined,
}

/// 过程节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProcedureNode {
    pub interface_name: Option<String>,
    pub attributes: Vec<Attribute>,
    pub entities: Vec<ProcedureEntity>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 过程实体
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProcedureEntity {
    pub name: String,
    pub binding_name: Option<String>,
}

/// 泛型节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GenericNode {
    pub generic_spec: GenericSpec,
    pub access_spec: Option<Attribute>,
    pub procedure_names: Vec<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 赋值节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AssignmentNode {
    pub variable: Box<ExprNode>,
    pub expression: Box<ExprNode>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 调用节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CallNode {
    pub procedure_name: String,
    pub arguments: Vec<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// If 构造节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IfConstructNode {
    pub condition: Box<ExprNode>,
    pub then_part: Vec<ExecutableStmt>,
    pub else_if_parts: Vec<(Box<ExprNode>, Vec<ExecutableStmt>)>,
    pub else_part: Option<Vec<ExecutableStmt>>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Do 构造节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DoConstructNode {
    pub name: Option<String>,
    pub control: Option<DoControl>,
    pub body: Vec<ExecutableStmt>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Do 控制
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DoControl {
    /// 迭代
    Iterative { variable: String, start: Box<ExprNode>, end: Box<ExprNode>, step: Option<Box<ExprNode>> },
    /// While
    While(Box<ExprNode>),
    /// 并发
    Concurrent { header: ConcurrentHeader, locality: Vec<LocalitySpec> },
}

/// 并发头
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConcurrentHeader {
    pub control_list: Vec<ConcurrentControl>,
    pub mask: Option<Box<ExprNode>>,
}

/// 并发控制
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConcurrentControl {
    pub name: String,
    pub start: Box<ExprNode>,
    pub end: Box<ExprNode>,
    pub step: Option<Box<ExprNode>>,
}

/// 局部性规范
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LocalitySpec {
    /// 本地
    Local(Vec<String>),
    /// 本地初始化
    LocalInit(Vec<String>),
    /// 共享
    Shared(Vec<String>),
    /// 默认无
    DefaultNone,
}

/// Select Case 节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SelectCaseNode {
    pub expression: Box<ExprNode>,
    pub cases: Vec<CaseConstruct>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Case 构造
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CaseConstruct {
    pub selector: CaseSelector,
    pub body: Vec<ExecutableStmt>,
}

/// Case 选择器
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CaseSelector {
    /// Case
    Case(Vec<CaseValue>),
    /// Default
    Default,
}

/// Case 值
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CaseValue {
    /// 单个值
    Single(Box<ExprNode>),
    /// 范围
    Range(Option<Box<ExprNode>>, Option<Box<ExprNode>>),
}

/// Where 构造节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WhereConstructNode {
    pub mask: Box<ExprNode>,
    pub where_body: Vec<ExecutableStmt>,
    pub else_where_parts: Vec<(Option<Box<ExprNode>>, Vec<ExecutableStmt>)>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Forall 构造节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForallConstructNode {
    pub header: ConcurrentHeader,
    pub body: Vec<ExecutableStmt>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Associate 构造节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AssociateConstructNode {
    pub associates: Vec<Associate>,
    pub body: Vec<ExecutableStmt>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 关联
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Associate {
    pub name: String,
    pub expression: Box<ExprNode>,
}

/// Block 构造节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BlockConstructNode {
    pub specification_part: Vec<SpecificationStmt>,
    pub execution_part: Vec<ExecutableStmt>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Critical 构造节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CriticalConstructNode {
    pub body: Vec<ExecutableStmt>,
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 分配节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AllocateNode {
    pub objects: Vec<Allocation>,
    pub options: Vec<AllocOpt>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 分配
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Allocation {
    pub variable: Box<ExprNode>,
    pub array_spec: Option<Vec<Dimension>>,
}

/// 分配选项
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AllocOpt {
    /// 统计
    Stat(Box<ExprNode>),
    /// 错误消息
    Errmsg(Box<ExprNode>),
    /// 源
    Source(Box<ExprNode>),
    /// 模子
    Mold(Box<ExprNode>),
}

/// 释放节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeallocateNode {
    pub objects: Vec<Box<ExprNode>>,
    pub options: Vec<DeallocOpt>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 释放选项
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeallocOpt {
    /// 统计
    Stat(Box<ExprNode>),
    /// 错误消息
    Errmsg(Box<ExprNode>),
}

/// 空化节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NullifyNode {
    pub pointers: Vec<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 停止节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StopNode {
    pub stop_code: Option<Box<ExprNode>>,
    pub quiet: Option<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 返回节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReturnNode {
    pub expression: Option<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 读节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReadNode {
    pub control_list: Vec<IoControl>,
    pub input_items: Vec<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 写节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WriteNode {
    pub control_list: Vec<IoControl>,
    pub output_items: Vec<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// 打印节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PrintNode {
    pub format: Box<ExprNode>,
    pub output_items: Vec<Box<ExprNode>>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// IO 控制
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IoControl {
    /// 单元
    Unit(Box<ExprNode>),
    /// 格式
    Fmt(Box<ExprNode>),
    /// 记录
    Rec(Box<ExprNode>),
    /// 统计
    Stat(Box<ExprNode>),
    /// 错误
    Err(String),
    /// 结束
    End(String),
    /// 错误消息
    Errmsg(Box<ExprNode>),
    /// 单元
    Iostat(Box<ExprNode>),
}

/// 表达式节点
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExprNode {
    /// 字面量
    Literal(Literal),
    /// 变量
    Variable(Variable),
    /// 一元运算
    Unary(UnaryOp, Box<ExprNode>),
    /// 二元运算
    Binary(Box<ExprNode>, BinaryOp, Box<ExprNode>),
    /// 函数调用
    Call(String, Vec<Box<ExprNode>>),
    /// 数组构造器
    ArrayConstructor(Vec<Box<ExprNode>>),
    /// 括号
    Paren(Box<ExprNode>),
}

/// 字面量
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Literal {
    /// 整数
    Integer(String),
    /// 实数
    Real(String),
    /// 复数
    Complex(String, String),
    /// 字符
    Character(String),
    /// 逻辑
    Logical(bool),
}

/// 变量
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Variable {
    pub name: String,
    pub selectors: Vec<VariableSelector>,
}

/// 变量选择器
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VariableSelector {
    /// 数组下标
    ArraySubscript(Vec<Box<ExprNode>>),
    /// 组件访问
    Component(String),
    /// 子串
    Substring(Option<Box<ExprNode>>, Option<Box<ExprNode>>),
}

/// 一元运算符
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnaryOp {
    /// 正
    Plus,
    /// 负
    Minus,
    /// 非
    Not,
}

/// 二元运算符
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BinaryOp {
    /// 加
    Add,
    /// 减
    Sub,
    /// 乘
    Mul,
    /// 除
    Div,
    /// 幂
    Pow,
    /// 等于
    Eq,
    /// 不等于
    Ne,
    /// 小于
    Lt,
    /// 小于等于
    Le,
    /// 大于
    Gt,
    /// 大于等于
    Ge,
    /// 且
    And,
    /// 或
    Or,
    /// 等价
    Eqv,
    /// 不等价
    Neqv,
    /// 连接
    Concat,
}
