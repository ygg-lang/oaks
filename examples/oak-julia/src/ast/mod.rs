use alloc::{boxed::Box, string::String, vec::Vec};

/// Python AST 根节
pub type JuliaAst = ModuleNode;

/// 模块节点
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleNode {
    pub statements: Vec<StmtKind>,
}

/// Python 语句种类
#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    /// 函数定义
    FunctionDef(FunctionDefNode),
    /// 异步函数定义
    AsyncFunctionDef(AsyncFunctionDefNode),
    /// 类定
    ClassDef(ClassDefNode),
    /// 返回语句
    Return(Option<Box<ExprKind>>),
    /// 删除语句
    Delete(Vec<ExprKind>),
    /// 赋值语
    Assign(AssignNode),
    /// 增强赋值语
    AugAssign(AugAssignNode),
    /// 注解赋值语
    AnnAssign(AnnAssignNode),
    /// For 循环
    For(ForNode),
    /// 异步 For 循环
    AsyncFor(AsyncForNode),
    /// While 循环
    While(WhileNode),
    /// If 语句
    If(IfNode),
    /// With 语句
    With(WithNode),
    /// 异步 With 语句
    AsyncWith(AsyncWithNode),
    /// Raise 语句
    Raise(RaiseNode),
    /// Try 语句
    Try(TryNode),
    /// Assert 语句
    Assert(AssertNode),
    /// Import 语句
    Import(ImportNode),
    /// ImportFrom 语句
    ImportFrom(ImportFromNode),
    /// Global 语句
    Global(Vec<String>),
    /// Nonlocal 语句
    Nonlocal(Vec<String>),
    /// 表达式语
    Expr(ExprKind),
    /// Pass 语句
    Pass,
    /// Break 语句
    Break,
    /// Continue 语句
    Continue,
}

/// Python 表达式种
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    /// 布尔操作
    BoolOp(BoolOpNode),
    /// 命名表达(walrus operator)
    NamedExpr(NamedExprNode),
    /// 二元操作
    BinOp(BinOpNode),
    /// 一元操
    UnaryOp(UnaryOpNode),
    /// Lambda 表达
    Lambda(LambdaNode),
    /// 条件表达
    IfExp(IfExpNode),
    /// 字典表达
    Dict(DictNode),
    /// 集合表达
    Set(SetNode),
    /// 列表推导
    ListComp(ListCompNode),
    /// 集合推导
    SetComp(SetCompNode),
    /// 字典推导
    DictComp(DictCompNode),
    /// 生成器表达式
    GeneratorExp(GeneratorExpNode),
    /// Await 表达
    Await(Box<ExprKind>),
    /// Yield 表达
    Yield(Option<Box<ExprKind>>),
    /// YieldFrom 表达
    YieldFrom(Box<ExprKind>),
    /// 比较表达
    Compare(CompareNode),
    /// 函数调用
    Call(CallNode),
    /// 格式化字符串
    FormattedValue(FormattedValueNode),
    /// 连接字符
    JoinedStr(Vec<ExprKind>),
    /// 常量
    Constant(ConstantNode),
    /// 属性访
    Attribute(AttributeNode),
    /// 下标访问
    Subscript(SubscriptNode),
    /// 星号表达
    Starred(Box<ExprKind>),
    /// 名称
    Name(NameNode),
    /// 列表
    List(ListNode),
    /// 元组
    Tuple(TupleNode),
    /// 切片
    Slice(SliceNode),
}

/// 函数定义节点
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefNode {
    pub name: String,
    pub args: ArgumentsNode,
    pub body: Vec<StmtKind>,
    pub decorator_list: Vec<ExprKind>,
    pub returns: Option<Box<ExprKind>>,
    pub type_comment: Option<String>,
}

/// 异步函数定义节点
#[derive(Debug, Clone, PartialEq)]
pub struct AsyncFunctionDefNode {
    pub name: String,
    pub args: ArgumentsNode,
    pub body: Vec<StmtKind>,
    pub decorator_list: Vec<ExprKind>,
    pub returns: Option<Box<ExprKind>>,
    pub type_comment: Option<String>,
}

/// 类定义节
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDefNode {
    pub name: String,
    pub bases: Vec<ExprKind>,
    pub keywords: Vec<KeywordNode>,
    pub body: Vec<StmtKind>,
    pub decorator_list: Vec<ExprKind>,
}

/// 参数节点
#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentsNode {
    pub posonlyargs: Vec<ArgNode>,
    pub args: Vec<ArgNode>,
    pub vararg: Option<ArgNode>,
    pub kwonlyargs: Vec<ArgNode>,
    pub kw_defaults: Vec<Option<ExprKind>>,
    pub kwarg: Option<ArgNode>,
    pub defaults: Vec<ExprKind>,
}

/// 参数节点
#[derive(Debug, Clone, PartialEq)]
pub struct ArgNode {
    pub arg: String,
    pub annotation: Option<Box<ExprKind>>,
    pub type_comment: Option<String>,
}

/// 关键字参数节
#[derive(Debug, Clone, PartialEq)]
pub struct KeywordNode {
    pub arg: Option<String>,
    pub value: ExprKind,
}

/// 赋值节
#[derive(Debug, Clone, PartialEq)]
pub struct AssignNode {
    pub targets: Vec<ExprKind>,
    pub value: ExprKind,
    pub type_comment: Option<String>,
}

/// 增强赋值节
#[derive(Debug, Clone, PartialEq)]
pub struct AugAssignNode {
    pub target: ExprKind,
    pub op: BinaryOperator,
    pub value: ExprKind,
}

/// 注解赋值节
#[derive(Debug, Clone, PartialEq)]
pub struct AnnAssignNode {
    pub target: ExprKind,
    pub annotation: ExprKind,
    pub value: Option<Box<ExprKind>>,
    pub simple: bool,
}

/// For 循环节点
#[derive(Debug, Clone, PartialEq)]
pub struct ForNode {
    pub target: ExprKind,
    pub iter: ExprKind,
    pub body: Vec<StmtKind>,
    pub orelse: Vec<StmtKind>,
    pub type_comment: Option<String>,
}

/// 异步 For 循环节点
#[derive(Debug, Clone, PartialEq)]
pub struct AsyncForNode {
    pub target: ExprKind,
    pub iter: ExprKind,
    pub body: Vec<StmtKind>,
    pub orelse: Vec<StmtKind>,
    pub type_comment: Option<String>,
}

/// While 循环节点
#[derive(Debug, Clone, PartialEq)]
pub struct WhileNode {
    pub test: ExprKind,
    pub body: Vec<StmtKind>,
    pub orelse: Vec<StmtKind>,
}

/// If 语句节点
#[derive(Debug, Clone, PartialEq)]
pub struct IfNode {
    pub test: ExprKind,
    pub body: Vec<StmtKind>,
    pub orelse: Vec<StmtKind>,
}

/// With 语句节点
#[derive(Debug, Clone, PartialEq)]
pub struct WithNode {
    pub items: Vec<WithItemNode>,
    pub body: Vec<StmtKind>,
    pub type_comment: Option<String>,
}

/// 异步 With 语句节点
#[derive(Debug, Clone, PartialEq)]
pub struct AsyncWithNode {
    pub items: Vec<WithItemNode>,
    pub body: Vec<StmtKind>,
    pub type_comment: Option<String>,
}

/// With 项节
#[derive(Debug, Clone, PartialEq)]
pub struct WithItemNode {
    pub context_expr: ExprKind,
    pub optional_vars: Option<Box<ExprKind>>,
}

/// Raise 语句节点
#[derive(Debug, Clone, PartialEq)]
pub struct RaiseNode {
    pub exc: Option<Box<ExprKind>>,
    pub cause: Option<Box<ExprKind>>,
}

/// Try 语句节点
#[derive(Debug, Clone, PartialEq)]
pub struct TryNode {
    pub body: Vec<StmtKind>,
    pub handlers: Vec<ExceptHandlerNode>,
    pub orelse: Vec<StmtKind>,
    pub finalbody: Vec<StmtKind>,
}

/// 异常处理器节
#[derive(Debug, Clone, PartialEq)]
pub struct ExceptHandlerNode {
    pub type_: Option<Box<ExprKind>>,
    pub name: Option<String>,
    pub body: Vec<StmtKind>,
}

/// Assert 语句节点
#[derive(Debug, Clone, PartialEq)]
pub struct AssertNode {
    pub test: ExprKind,
    pub msg: Option<Box<ExprKind>>,
}

/// Import 语句节点
#[derive(Debug, Clone, PartialEq)]
pub struct ImportNode {
    pub names: Vec<AliasNode>,
}

/// ImportFrom 语句节点
#[derive(Debug, Clone, PartialEq)]
pub struct ImportFromNode {
    pub module: Option<String>,
    pub names: Vec<AliasNode>,
    pub level: Option<i32>,
}

/// 别名节点
#[derive(Debug, Clone, PartialEq)]
pub struct AliasNode {
    pub name: String,
    pub asname: Option<String>,
}

/// 布尔操作节点
#[derive(Debug, Clone, PartialEq)]
pub struct BoolOpNode {
    pub op: BooleanOperator,
    pub values: Vec<ExprKind>,
}

/// 命名表达式节
#[derive(Debug, Clone, PartialEq)]
pub struct NamedExprNode {
    pub target: Box<ExprKind>,
    pub value: Box<ExprKind>,
}

/// 二元操作节点
#[derive(Debug, Clone, PartialEq)]
pub struct BinOpNode {
    pub left: Box<ExprKind>,
    pub op: BinaryOperator,
    pub right: Box<ExprKind>,
}

/// 一元操作节
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOpNode {
    pub op: UnaryOperator,
    pub operand: Box<ExprKind>,
}

/// Lambda 节点
#[derive(Debug, Clone, PartialEq)]
pub struct LambdaNode {
    pub args: ArgumentsNode,
    pub body: Box<ExprKind>,
}

/// 条件表达式节
#[derive(Debug, Clone, PartialEq)]
pub struct IfExpNode {
    pub test: Box<ExprKind>,
    pub body: Box<ExprKind>,
    pub orelse: Box<ExprKind>,
}

/// 字典节点
#[derive(Debug, Clone, PartialEq)]
pub struct DictNode {
    pub keys: Vec<Option<ExprKind>>,
    pub values: Vec<ExprKind>,
}

/// 集合节点
#[derive(Debug, Clone, PartialEq)]
pub struct SetNode {
    pub elts: Vec<ExprKind>,
}

/// 列表推导式节
#[derive(Debug, Clone, PartialEq)]
pub struct ListCompNode {
    pub elt: Box<ExprKind>,
    pub generators: Vec<ComprehensionNode>,
}

/// 集合推导式节
#[derive(Debug, Clone, PartialEq)]
pub struct SetCompNode {
    pub elt: Box<ExprKind>,
    pub generators: Vec<ComprehensionNode>,
}

/// 字典推导式节
#[derive(Debug, Clone, PartialEq)]
pub struct DictCompNode {
    pub key: Box<ExprKind>,
    pub value: Box<ExprKind>,
    pub generators: Vec<ComprehensionNode>,
}

/// 生成器表达式节点
#[derive(Debug, Clone, PartialEq)]
pub struct GeneratorExpNode {
    pub elt: Box<ExprKind>,
    pub generators: Vec<ComprehensionNode>,
}

/// 推导式节
#[derive(Debug, Clone, PartialEq)]
pub struct ComprehensionNode {
    pub target: ExprKind,
    pub iter: ExprKind,
    pub ifs: Vec<ExprKind>,
    pub is_async: bool,
}

/// 比较节点
#[derive(Debug, Clone, PartialEq)]
pub struct CompareNode {
    pub left: Box<ExprKind>,
    pub ops: Vec<ComparisonOperator>,
    pub comparators: Vec<ExprKind>,
}

/// 函数调用节点
#[derive(Debug, Clone, PartialEq)]
pub struct CallNode {
    pub func: Box<ExprKind>,
    pub args: Vec<ExprKind>,
    pub keywords: Vec<KeywordNode>,
}

/// 格式化值节
#[derive(Debug, Clone, PartialEq)]
pub struct FormattedValueNode {
    pub value: Box<ExprKind>,
    pub conversion: Option<i32>,
    pub format_spec: Option<Box<ExprKind>>,
}

/// 常量节点
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantNode {
    pub value: ConstantValue,
    pub kind: Option<String>,
}

/// 常量
#[derive(Debug, Clone, PartialEq)]
pub enum ConstantValue {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    Complex { real: f64, imag: f64 },
    Str(String),
    Bytes(Vec<u8>),
    Ellipsis,
}

/// 属性访问节
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeNode {
    pub value: Box<ExprKind>,
    pub attr: String,
}

/// 下标访问节点
#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptNode {
    pub value: Box<ExprKind>,
    pub slice: Box<ExprKind>,
}

/// 名称节点
#[derive(Debug, Clone, PartialEq)]
pub struct NameNode {
    pub id: String,
}

/// 列表节点
#[derive(Debug, Clone, PartialEq)]
pub struct ListNode {
    pub elts: Vec<ExprKind>,
}

/// 元组节点
#[derive(Debug, Clone, PartialEq)]
pub struct TupleNode {
    pub elts: Vec<ExprKind>,
}

/// 切片节点
#[derive(Debug, Clone, PartialEq)]
pub struct SliceNode {
    pub lower: Option<Box<ExprKind>>,
    pub upper: Option<Box<ExprKind>>,
    pub step: Option<Box<ExprKind>>,
}

/// 布尔操作
#[derive(Debug, Clone, PartialEq)]
pub enum BooleanOperator {
    And,
    Or,
}

/// 二元操作
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

/// 一元操作符
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Invert,
    Not,
    UAdd,
    USub,
}

/// 比较操作
#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOperator {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
}

// 实现 AstNode trait
impl ModuleNode {
    pub fn kind(&self) -> &'static str {
        "Module"
    }
}

/// Python AST 访问trait
pub trait JuliaAstVisitor {
    fn visit_module(&mut self, node: &ModuleNode);
    fn visit_stmt(&mut self, stmt: &StmtKind);
    fn visit_expr(&mut self, expr: &ExprKind);
    fn visit_function_def(&mut self, node: &FunctionDefNode);
    fn visit_class_def(&mut self, node: &ClassDefNode);
    fn visit_call(&mut self, node: &CallNode);
    fn visit_name(&mut self, node: &NameNode);
    fn visit_constant(&mut self, node: &ConstantNode);
}
