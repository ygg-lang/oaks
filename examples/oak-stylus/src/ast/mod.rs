#![doc = include_str!("readme.md")]
use core::range::Range;
use oak_core::tree::{GreenNode, RedNode, TypedNode};

/// AST节点访问者trait
///
/// 提供了访问各种Stylus AST节点的方法，用于实现访问者模式
pub trait StylusVisitor {
    /// 访问根节点
    fn visit_root(&mut self, node: &StylusRoot) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问规则节点
    fn visit_rule(&mut self, node: &StylusRule) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问注释节点
    fn visit_comment(&mut self, node: &StylusComment) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问属性节点
    fn visit_property(&mut self, node: &StylusProperty) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问混合定义节点
    fn visit_mixin(&mut self, node: &StylusMixin) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问变量定义节点
    fn visit_variable(&mut self, node: &StylusVariable) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问导入语句节点
    fn visit_import(&mut self, node: &StylusImport) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问函数定义节点
    fn visit_function(&mut self, node: &StylusFunction) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问条件语句节点
    fn visit_if(&mut self, node: &StylusIf) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问for循环节点
    fn visit_for(&mut self, node: &StylusFor) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问while循环节点
    fn visit_while(&mut self, node: &StylusWhile) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问参数节点
    fn visit_param(&mut self, node: &StylusParam) -> VisitResult {
        VisitResult::Continue
    }
}

/// 访问结果枚举
///
/// 用于控制访问过程的流程
pub enum VisitResult {
    /// 继续访问
    Continue,
    /// 停止访问
    Stop,
    /// 跳过当前节点的子节点
    SkipChildren,
}

/// AST节点可访问性trait
///
/// 为AST节点提供接受访问者的方法
pub trait AcceptVisitor {
    /// 接受访问者访问
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult;
}

/// AST处理器trait
///
/// 定义处理AST的方法，用于实现AST的转换和操作
pub trait AstProcessor {
    /// 遍历AST并应用访问者
    fn process<T: AcceptVisitor>(&self, node: &T, visitor: &mut impl StylusVisitor);

    /// 深度遍历AST并应用访问者
    fn process_recursive<T: AcceptVisitor>(&self, node: &T, visitor: &mut impl StylusVisitor);

    /// 转换AST节点
    fn transform<T: AcceptVisitor>(&self, node: &T) -> T;
}

/// 默认AST处理器
///
/// 提供AstProcessor trait的默认实现
pub struct DefaultAstProcessor;

impl AstProcessor for DefaultAstProcessor {
    /// 遍历AST并应用访问者
    fn process<T: AcceptVisitor>(&self, node: &T, visitor: &mut impl StylusVisitor) {
        node.accept(visitor);
    }

    /// 深度遍历AST并应用访问者
    fn process_recursive<T: AcceptVisitor>(&self, node: &T, visitor: &mut impl StylusVisitor) {
        self.process(node, visitor);
    }

    /// 转换AST节点（默认实现：返回原节点）
    fn transform<T: AcceptVisitor>(&self, node: &T) -> T {
        // 注意：这里的实现需要根据具体类型进行调整
        // 暂时返回原节点的克隆
        unimplemented!("DefaultAstProcessor::transform() not implemented")
    }
}

impl DefaultAstProcessor {
    /// 创建一个新的默认AST处理器
    pub fn new() -> Self {
        Self
    }
}

/// 映射AST处理器
///
/// 用于将AST节点映射为新的节点
pub struct MapAstProcessor<F: MapFn> {
    /// 映射函数
    pub map_fn: F,
}

impl<F: MapFn> MapAstProcessor<F> {
    /// 创建一个新的映射AST处理器
    pub fn new(map_fn: F) -> Self {
        Self { map_fn }
    }
}

impl<F: MapFn> AstProcessor for MapAstProcessor<F> {
    /// 遍历AST并应用访问者
    fn process<T: AcceptVisitor>(&self, node: &T, visitor: &mut impl StylusVisitor) {
        node.accept(visitor);
    }

    /// 深度遍历AST并应用访问者
    fn process_recursive<T: AcceptVisitor>(&self, node: &T, visitor: &mut impl StylusVisitor) {
        self.process(node, visitor);
    }

    /// 转换AST节点
    fn transform<T: AcceptVisitor>(&self, node: &T) -> T {
        // 注意：这里的实现需要根据具体类型进行调整
        unimplemented!("MapAstProcessor::transform() not implemented")
    }
}

/// 调试AST处理器
///
/// 用于打印AST节点信息，方便调试
pub struct DebugAstProcessor {
    /// 缩进级别
    pub indent: usize,
}

impl DebugAstProcessor {
    /// 创建一个新的调试AST处理器
    pub fn new() -> Self {
        Self { indent: 0 }
    }
}

impl AstProcessor for DebugAstProcessor {
    /// 遍历AST并应用访问者
    fn process<T: AcceptVisitor>(&self, node: &T, visitor: &mut impl StylusVisitor) {
        node.accept(visitor);
    }

    /// 深度遍历AST并应用访问者
    fn process_recursive<T: AcceptVisitor>(&self, node: &T, visitor: &mut impl StylusVisitor) {
        self.process(node, visitor);
    }

    /// 转换AST节点（默认实现：返回原节点）
    fn transform<T: AcceptVisitor>(&self, node: &T) -> T {
        // 注意：这里的实现需要根据具体类型进行调整
        unimplemented!("DebugAstProcessor::transform() not implemented")
    }
}

/// 默认访问者
///
/// 提供所有访问方法的默认实现，方便用户只重写需要的方法
pub struct DefaultVisitor;

impl StylusVisitor for DefaultVisitor {
    // 所有方法都使用默认实现
}

/// 映射访问者
///
/// 用于将AST节点映射为新的节点，实现AST转换
pub struct MapVisitor<F> {
    /// 映射函数
    pub map_fn: F,
}

impl<F> MapVisitor<F> {
    /// 创建一个新的映射访问者
    pub fn new(map_fn: F) -> Self {
        Self { map_fn }
    }
}

/// 映射函数trait
///
/// 定义了映射AST节点的方法
pub trait MapFn {
    /// 映射根节点
    fn map_root(&mut self, node: &StylusRoot) -> StylusRoot {
        let mut new_root = StylusRoot::new(node.span.clone());
        for item in &node.items {
            new_root.add_item(self.map_item(item));
        }
        new_root
    }

    /// 映射项节点
    fn map_item(&mut self, node: &StylusItem) -> StylusItem {
        match node {
            StylusItem::Rule(rule) => StylusItem::Rule(self.map_rule(rule)),
            StylusItem::Comment(comment) => StylusItem::Comment(self.map_comment(comment)),
            StylusItem::Mixin(mixin) => StylusItem::Mixin(self.map_mixin(mixin)),
            StylusItem::Variable(variable) => StylusItem::Variable(self.map_variable(variable)),
            StylusItem::Import(import) => StylusItem::Import(self.map_import(import)),
            StylusItem::Function(function) => StylusItem::Function(self.map_function(function)),
            StylusItem::If(if_stmt) => StylusItem::If(self.map_if(if_stmt)),
            StylusItem::For(for_stmt) => StylusItem::For(self.map_for(for_stmt)),
            StylusItem::While(while_stmt) => StylusItem::While(self.map_while(while_stmt)),
        }
    }

    /// 映射规则节点
    fn map_rule(&mut self, node: &StylusRule) -> StylusRule {
        let mut new_rule = StylusRule { span: node.span.clone(), selector: node.selector.clone(), properties: Vec::new() };
        for property in &node.properties {
            new_rule.add_property(self.map_property(property));
        }
        new_rule
    }

    /// 映射注释节点
    fn map_comment(&mut self, node: &StylusComment) -> StylusComment {
        StylusComment { span: node.span.clone(), text: node.text.clone() }
    }

    /// 映射属性节点
    fn map_property(&mut self, node: &StylusProperty) -> StylusProperty {
        StylusProperty { span: node.span.clone(), name: node.name.clone(), value: node.value.clone() }
    }

    /// 映射混合定义节点
    fn map_mixin(&mut self, node: &StylusMixin) -> StylusMixin {
        let mut new_mixin = StylusMixin { span: node.span.clone(), name: node.name.clone(), params: Vec::new(), body: Vec::new() };
        for param in &node.params {
            new_mixin.add_param(self.map_param(param));
        }
        for item in &node.body {
            new_mixin.add_body_item(self.map_item(item));
        }
        new_mixin
    }

    /// 映射变量定义节点
    fn map_variable(&mut self, node: &StylusVariable) -> StylusVariable {
        StylusVariable { span: node.span.clone(), name: node.name.clone(), value: node.value.clone() }
    }

    /// 映射导入语句节点
    fn map_import(&mut self, node: &StylusImport) -> StylusImport {
        StylusImport { span: node.span.clone(), path: node.path.clone() }
    }

    /// 映射函数定义节点
    fn map_function(&mut self, node: &StylusFunction) -> StylusFunction {
        let mut new_function = StylusFunction { span: node.span.clone(), name: node.name.clone(), params: Vec::new(), body: Vec::new(), return_value: node.return_value.clone() };
        for param in &node.params {
            new_function.add_param(self.map_param(param));
        }
        for item in &node.body {
            new_function.add_body_item(self.map_item(item));
        }
        new_function
    }

    /// 映射条件语句节点
    fn map_if(&mut self, node: &StylusIf) -> StylusIf {
        let mut new_if = StylusIf { span: node.span.clone(), condition: node.condition.clone(), body: Vec::new(), else_clause: None };
        for item in &node.body {
            new_if.add_body_item(self.map_item(item));
        }
        if let Some(else_items) = &node.else_clause {
            let mut new_else_items = Vec::new();
            for item in else_items {
                new_else_items.push(self.map_item(item));
            }
            new_if.else_clause = Some(new_else_items);
        }
        new_if
    }

    /// 映射for循环节点
    fn map_for(&mut self, node: &StylusFor) -> StylusFor {
        let mut new_for = StylusFor { span: node.span.clone(), variable: node.variable.clone(), range: node.range.clone(), body: Vec::new() };
        for item in &node.body {
            new_for.add_body_item(self.map_item(item));
        }
        new_for
    }

    /// 映射while循环节点
    fn map_while(&mut self, node: &StylusWhile) -> StylusWhile {
        let mut new_while = StylusWhile { span: node.span.clone(), condition: node.condition.clone(), body: Vec::new() };
        for item in &node.body {
            new_while.add_body_item(self.map_item(item));
        }
        new_while
    }

    /// 映射参数节点
    fn map_param(&mut self, node: &StylusParam) -> StylusParam {
        StylusParam { span: node.span.clone(), name: node.name.clone(), default: node.default.clone() }
    }
}

/// 调试访问者
///
/// 用于打印AST节点信息，方便调试
pub struct DebugVisitor {
    /// 缩进级别
    pub indent: usize,
}

impl DebugVisitor {
    /// 创建一个新的调试访问者
    pub fn new() -> Self {
        Self { indent: 0 }
    }

    /// 打印缩进
    fn print_indent(&self) {
        for _ in 0..self.indent {
            print!("  ");
        }
    }
}

impl StylusVisitor for DebugVisitor {
    fn visit_root(&mut self, node: &StylusRoot) -> VisitResult {
        self.print_indent();
        println!(
            "StylusRoot (span: {:?})
",
            node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_rule(&mut self, node: &StylusRule) -> VisitResult {
        self.print_indent();
        println!(
            "StylusRule (selector: {}, span: {:?})
",
            node.selector, node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_comment(&mut self, node: &StylusComment) -> VisitResult {
        self.print_indent();
        println!(
            "StylusComment (text: {}, span: {:?})
",
            node.text, node.span
        );
        VisitResult::Continue
    }

    fn visit_property(&mut self, node: &StylusProperty) -> VisitResult {
        self.print_indent();
        println!(
            "StylusProperty (name: {}, value: {}, span: {:?})
",
            node.name, node.value, node.span
        );
        VisitResult::Continue
    }

    fn visit_mixin(&mut self, node: &StylusMixin) -> VisitResult {
        self.print_indent();
        println!(
            "StylusMixin (name: {}, span: {:?})
",
            node.name, node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_variable(&mut self, node: &StylusVariable) -> VisitResult {
        self.print_indent();
        println!(
            "StylusVariable (name: {}, value: {}, span: {:?})
",
            node.name, node.value, node.span
        );
        VisitResult::Continue
    }

    fn visit_import(&mut self, node: &StylusImport) -> VisitResult {
        self.print_indent();
        println!(
            "StylusImport (path: {}, span: {:?})
",
            node.path, node.span
        );
        VisitResult::Continue
    }

    fn visit_function(&mut self, node: &StylusFunction) -> VisitResult {
        self.print_indent();
        println!(
            "StylusFunction (name: {}, span: {:?})
",
            node.name, node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_if(&mut self, node: &StylusIf) -> VisitResult {
        self.print_indent();
        println!(
            "StylusIf (condition: {}, span: {:?})
",
            node.condition, node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_for(&mut self, node: &StylusFor) -> VisitResult {
        self.print_indent();
        println!(
            "StylusFor (variable: {}, range: {}, span: {:?})
",
            node.variable, node.range, node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_while(&mut self, node: &StylusWhile) -> VisitResult {
        self.print_indent();
        println!(
            "StylusWhile (condition: {}, span: {:?})
",
            node.condition, node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_param(&mut self, node: &StylusParam) -> VisitResult {
        self.print_indent();
        println!(
            "StylusParam (name: {}, default: {:?}, span: {:?})
",
            node.name, node.default, node.span
        );
        VisitResult::Continue
    }
}

/// Stylus document root node
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusRoot {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the root node.
    pub span: Range<usize>,
    /// The list of top-level items in the document.
    pub items: Vec<StylusItem>,
}

/// Stylus top-level item
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StylusItem {
    /// CSS rule item.
    Rule(StylusRule),
    /// Comment item.
    Comment(StylusComment),
    /// Mixin definition item.
    Mixin(StylusMixin),
    /// Variable definition item.
    Variable(StylusVariable),
    /// Import statement item.
    Import(StylusImport),
    /// Function definition item.
    Function(StylusFunction),
    /// If statement item.
    If(StylusIf),
    /// For loop item.
    For(StylusFor),
    /// While loop item.
    While(StylusWhile),
}

/// Stylus rule
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusRule {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the rule.
    pub span: Range<usize>,
    /// Selector of the rule.
    pub selector: String,
    /// Properties of the rule.
    pub properties: Vec<StylusProperty>,
}

/// Stylus comment
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusComment {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the comment.
    pub span: Range<usize>,
    /// Text of the comment.
    pub text: String,
}

/// Stylus property
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusProperty {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the property.
    pub span: Range<usize>,
    /// Name of the property.
    pub name: String,
    /// Value of the property.
    pub value: String,
}

/// Stylus mixin definition
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusMixin {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the mixin.
    pub span: Range<usize>,
    /// Name of the mixin.
    pub name: String,
    /// Parameters of the mixin.
    pub params: Vec<StylusParam>,
    /// Body of the mixin.
    pub body: Vec<StylusItem>,
}

/// Stylus variable definition
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusVariable {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the variable.
    pub span: Range<usize>,
    /// Name of the variable.
    pub name: String,
    /// Value of the variable.
    pub value: String,
}

/// Stylus import statement
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusImport {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the import.
    pub span: Range<usize>,
    /// Path to the imported file.
    pub path: String,
}

/// Stylus function definition
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusFunction {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the function.
    pub span: Range<usize>,
    /// Name of the function.
    pub name: String,
    /// Parameters of the function.
    pub params: Vec<StylusParam>,
    /// Body of the function.
    pub body: Vec<StylusItem>,
    /// Return value of the function.
    pub return_value: Option<String>,
}

/// Stylus if statement
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusIf {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the if statement.
    pub span: Range<usize>,
    /// Condition of the if statement.
    pub condition: String,
    /// Body of the if statement.
    pub body: Vec<StylusItem>,
    /// Else clause of the if statement.
    pub else_clause: Option<Vec<StylusItem>>,
}

/// Stylus for loop
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusFor {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the for loop.
    pub span: Range<usize>,
    /// Variable name of the for loop.
    pub variable: String,
    /// Range expression of the for loop.
    pub range: String,
    /// Body of the for loop.
    pub body: Vec<StylusItem>,
}

/// Stylus while loop
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusWhile {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the while loop.
    pub span: Range<usize>,
    /// Condition of the while loop.
    pub condition: String,
    /// Body of the while loop.
    pub body: Vec<StylusItem>,
}

/// Stylus parameter
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StylusParam {
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    /// The source span of the parameter.
    pub span: Range<usize>,
    /// Name of the parameter.
    pub name: String,
    /// Default value of the parameter.
    pub default: Option<String>,
}

impl StylusRoot {
    /// Creates a new StylusRoot with the given span.
    pub fn new(span: Range<usize>) -> Self {
        Self { span, items: Vec::new() }
    }

    /// Adds an item to the root.
    pub fn add_item(&mut self, item: StylusItem) {
        self.items.push(item);
    }

    /// Removes an item from the root at the given index.
    pub fn remove_item(&mut self, index: usize) -> Option<StylusItem> {
        if index < self.items.len() { Some(self.items.remove(index)) } else { None }
    }

    /// Inserts an item at the given index.
    pub fn insert_item(&mut self, index: usize, item: StylusItem) {
        if index <= self.items.len() {
            self.items.insert(index, item);
        }
    }
}

impl AcceptVisitor for StylusRoot {
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult {
        match visitor.visit_root(self) {
            VisitResult::Stop => VisitResult::Stop,
            VisitResult::SkipChildren => VisitResult::Continue,
            VisitResult::Continue => {
                for item in &self.items {
                    if let VisitResult::Stop = item.accept(visitor) {
                        return VisitResult::Stop;
                    }
                }
                VisitResult::Continue
            }
        }
    }
}

impl AcceptVisitor for StylusItem {
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult {
        match self {
            StylusItem::Rule(rule) => rule.accept(visitor),
            StylusItem::Comment(comment) => comment.accept(visitor),
            StylusItem::Mixin(mixin) => mixin.accept(visitor),
            StylusItem::Variable(variable) => variable.accept(visitor),
            StylusItem::Import(import) => import.accept(visitor),
            StylusItem::Function(function) => function.accept(visitor),
            StylusItem::If(if_stmt) => if_stmt.accept(visitor),
            StylusItem::For(for_stmt) => for_stmt.accept(visitor),
            StylusItem::While(while_stmt) => while_stmt.accept(visitor),
        }
    }
}

impl AcceptVisitor for StylusRule {
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult {
        match visitor.visit_rule(self) {
            VisitResult::Stop => VisitResult::Stop,
            VisitResult::SkipChildren => VisitResult::Continue,
            VisitResult::Continue => {
                for property in &self.properties {
                    if let VisitResult::Stop = property.accept(visitor) {
                        return VisitResult::Stop;
                    }
                }
                VisitResult::Continue
            }
        }
    }
}

impl StylusRule {
    /// Adds a property to the rule.
    pub fn add_property(&mut self, property: StylusProperty) {
        self.properties.push(property);
    }

    /// Removes a property from the rule at the given index.
    pub fn remove_property(&mut self, index: usize) -> Option<StylusProperty> {
        if index < self.properties.len() { Some(self.properties.remove(index)) } else { None }
    }

    /// Inserts a property at the given index.
    pub fn insert_property(&mut self, index: usize, property: StylusProperty) {
        if index <= self.properties.len() {
            self.properties.insert(index, property);
        }
    }
}

impl AcceptVisitor for StylusComment {
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult {
        visitor.visit_comment(self)
    }
}

impl AcceptVisitor for StylusProperty {
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult {
        visitor.visit_property(self)
    }
}

impl AcceptVisitor for StylusMixin {
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult {
        match visitor.visit_mixin(self) {
            VisitResult::Stop => VisitResult::Stop,
            VisitResult::SkipChildren => VisitResult::Continue,
            VisitResult::Continue => {
                for param in &self.params {
                    if let VisitResult::Stop = param.accept(visitor) {
                        return VisitResult::Stop;
                    }
                }
                for item in &self.body {
                    if let VisitResult::Stop = item.accept(visitor) {
                        return VisitResult::Stop;
                    }
                }
                VisitResult::Continue
            }
        }
    }
}

impl StylusMixin {
    /// Adds a parameter to the mixin.
    pub fn add_param(&mut self, param: StylusParam) {
        self.params.push(param);
    }

    /// Adds an item to the mixin body.
    pub fn add_body_item(&mut self, item: StylusItem) {
        self.body.push(item);
    }
}

impl AcceptVisitor for StylusVariable {
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult {
        visitor.visit_variable(self)
    }
}

impl AcceptVisitor for StylusImport {
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult {
        visitor.visit_import(self)
    }
}

impl AcceptVisitor for StylusFunction {
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult {
        match visitor.visit_function(self) {
            VisitResult::Stop => VisitResult::Stop,
            VisitResult::SkipChildren => VisitResult::Continue,
            VisitResult::Continue => {
                for param in &self.params {
                    if let VisitResult::Stop = param.accept(visitor) {
                        return VisitResult::Stop;
                    }
                }
                for item in &self.body {
                    if let VisitResult::Stop = item.accept(visitor) {
                        return VisitResult::Stop;
                    }
                }
                VisitResult::Continue
            }
        }
    }
}

impl StylusFunction {
    /// Adds a parameter to the function.
    pub fn add_param(&mut self, param: StylusParam) {
        self.params.push(param);
    }

    /// Adds an item to the function body.
    pub fn add_body_item(&mut self, item: StylusItem) {
        self.body.push(item);
    }

    /// Sets the return value of the function.
    pub fn set_return_value(&mut self, return_value: Option<String>) {
        self.return_value = return_value;
    }
}

impl AcceptVisitor for StylusIf {
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult {
        match visitor.visit_if(self) {
            VisitResult::Stop => VisitResult::Stop,
            VisitResult::SkipChildren => VisitResult::Continue,
            VisitResult::Continue => {
                for item in &self.body {
                    if let VisitResult::Stop = item.accept(visitor) {
                        return VisitResult::Stop;
                    }
                }
                if let Some(else_items) = &self.else_clause {
                    for item in else_items {
                        if let VisitResult::Stop = item.accept(visitor) {
                            return VisitResult::Stop;
                        }
                    }
                }
                VisitResult::Continue
            }
        }
    }
}

impl StylusIf {
    /// Adds an item to the if body.
    pub fn add_body_item(&mut self, item: StylusItem) {
        self.body.push(item);
    }

    /// Adds an item to the else body.
    pub fn add_else_item(&mut self, item: StylusItem) {
        if self.else_clause.is_none() {
            self.else_clause = Some(Vec::new());
        }
        if let Some(else_items) = &mut self.else_clause {
            else_items.push(item);
        }
    }
}

impl AcceptVisitor for StylusFor {
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult {
        match visitor.visit_for(self) {
            VisitResult::Stop => VisitResult::Stop,
            VisitResult::SkipChildren => VisitResult::Continue,
            VisitResult::Continue => {
                for item in &self.body {
                    if let VisitResult::Stop = item.accept(visitor) {
                        return VisitResult::Stop;
                    }
                }
                VisitResult::Continue
            }
        }
    }
}

impl StylusFor {
    /// Adds an item to the for loop body.
    pub fn add_body_item(&mut self, item: StylusItem) {
        self.body.push(item);
    }
}

impl AcceptVisitor for StylusWhile {
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult {
        match visitor.visit_while(self) {
            VisitResult::Stop => VisitResult::Stop,
            VisitResult::SkipChildren => VisitResult::Continue,
            VisitResult::Continue => {
                for item in &self.body {
                    if let VisitResult::Stop = item.accept(visitor) {
                        return VisitResult::Stop;
                    }
                }
                VisitResult::Continue
            }
        }
    }
}

impl StylusWhile {
    /// Adds an item to the while loop body.
    pub fn add_body_item(&mut self, item: StylusItem) {
        self.body.push(item);
    }
}

impl AcceptVisitor for StylusParam {
    fn accept(&self, visitor: &mut impl StylusVisitor) -> VisitResult {
        visitor.visit_param(self)
    }
}

impl<'a> TypedNode<'a> for StylusRoot {
    type Language = crate::StylusLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        let children: Vec<_> = node.children().collect();
        Some(Self { span: node.span().into(), items: Vec::new() })
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("StylusRoot::green() not implemented")
    }
}

/// Implements TypedNode for StylusItem
impl<'a> TypedNode<'a> for StylusItem {
    type Language = crate::StylusLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        if let Some(rule) = StylusRule::cast(node.clone()) {
            Some(Self::Rule(rule))
        }
        else if let Some(comment) = StylusComment::cast(node.clone()) {
            Some(Self::Comment(comment))
        }
        else if let Some(mixin) = StylusMixin::cast(node.clone()) {
            Some(Self::Mixin(mixin))
        }
        else if let Some(variable) = StylusVariable::cast(node.clone()) {
            Some(Self::Variable(variable))
        }
        else if let Some(import) = StylusImport::cast(node.clone()) {
            Some(Self::Import(import))
        }
        else if let Some(function) = StylusFunction::cast(node.clone()) {
            Some(Self::Function(function))
        }
        else if let Some(if_stmt) = StylusIf::cast(node.clone()) {
            Some(Self::If(if_stmt))
        }
        else if let Some(for_stmt) = StylusFor::cast(node.clone()) {
            Some(Self::For(for_stmt))
        }
        else if let Some(while_stmt) = StylusWhile::cast(node.clone()) {
            Some(Self::While(while_stmt))
        }
        else {
            None
        }
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("StylusItem::green() not implemented")
    }
}

/// Implements TypedNode for StylusRule
impl<'a> TypedNode<'a> for StylusRule {
    type Language = crate::StylusLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        let children: Vec<_> = node.children().collect();
        Some(Self { span: node.span().into(), selector: String::new(), properties: Vec::new() })
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("StylusRule::green() not implemented")
    }
}

/// Implements TypedNode for StylusComment
impl<'a> TypedNode<'a> for StylusComment {
    type Language = crate::StylusLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        Some(Self { span: node.span().into(), text: String::new() })
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("StylusComment::green() not implemented")
    }
}

/// Implements TypedNode for StylusProperty
impl<'a> TypedNode<'a> for StylusProperty {
    type Language = crate::StylusLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        let children: Vec<_> = node.children().collect();
        Some(Self { span: node.span().into(), name: String::new(), value: String::new() })
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("StylusProperty::green() not implemented")
    }
}

/// Implements TypedNode for StylusMixin
impl<'a> TypedNode<'a> for StylusMixin {
    type Language = crate::StylusLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        let children: Vec<_> = node.children().collect();
        Some(Self { span: node.span().into(), name: String::new(), params: Vec::new(), body: Vec::new() })
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("StylusMixin::green() not implemented")
    }
}

/// Implements TypedNode for StylusVariable
impl<'a> TypedNode<'a> for StylusVariable {
    type Language = crate::StylusLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        let children: Vec<_> = node.children().collect();
        Some(Self { span: node.span().into(), name: String::new(), value: String::new() })
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("StylusVariable::green() not implemented")
    }
}

/// Implements TypedNode for StylusImport
impl<'a> TypedNode<'a> for StylusImport {
    type Language = crate::StylusLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        Some(Self { span: node.span().into(), path: String::new() })
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("StylusImport::green() not implemented")
    }
}

/// Implements TypedNode for StylusFunction
impl<'a> TypedNode<'a> for StylusFunction {
    type Language = crate::StylusLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        let children: Vec<_> = node.children().collect();
        Some(Self { span: node.span().into(), name: String::new(), params: Vec::new(), body: Vec::new(), return_value: None })
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("StylusFunction::green() not implemented")
    }
}

/// Implements TypedNode for StylusIf
impl<'a> TypedNode<'a> for StylusIf {
    type Language = crate::StylusLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        let children: Vec<_> = node.children().collect();
        Some(Self { span: node.span().into(), condition: String::new(), body: Vec::new(), else_clause: None })
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("StylusIf::green() not implemented")
    }
}

/// Implements TypedNode for StylusFor
impl<'a> TypedNode<'a> for StylusFor {
    type Language = crate::StylusLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        let children: Vec<_> = node.children().collect();
        Some(Self { span: node.span().into(), variable: String::new(), range: String::new(), body: Vec::new() })
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("StylusFor::green() not implemented")
    }
}

/// Implements TypedNode for StylusWhile
impl<'a> TypedNode<'a> for StylusWhile {
    type Language = crate::StylusLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        let children: Vec<_> = node.children().collect();
        Some(Self { span: node.span().into(), condition: String::new(), body: Vec::new() })
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("StylusWhile::green() not implemented")
    }
}

/// Implements TypedNode for StylusParam
impl<'a> TypedNode<'a> for StylusParam {
    type Language = crate::StylusLanguage;

    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self> {
        let children: Vec<_> = node.children().collect();
        Some(Self { span: node.span().into(), name: String::new(), default: None })
    }

    fn green(&self) -> &'a GreenNode<'a, Self::Language> {
        // 注意：这是一个临时实现，需要根据实际情况修复
        panic!("StylusParam::green() not implemented")
    }
}
