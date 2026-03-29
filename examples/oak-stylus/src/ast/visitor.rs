use core::range::Range;

/// AST节点访问者trait
///
/// 提供了访问各种Stylus AST节点的方法，用于实现访问者模式
pub trait StylusVisitor {
    /// 访问根节点
    fn visit_root(&mut self, node: &crate::ast::StylusRoot) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问规则节点
    fn visit_rule(&mut self, node: &crate::ast::StylusRule) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问注释节点
    fn visit_comment(&mut self, node: &crate::ast::StylusComment) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问属性节点
    fn visit_property(&mut self, node: &crate::ast::StylusProperty) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问混合定义节点
    fn visit_mixin(&mut self, node: &crate::ast::StylusMixin) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问变量定义节点
    fn visit_variable(&mut self, node: &crate::ast::StylusVariable) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问导入语句节点
    fn visit_import(&mut self, node: &crate::ast::StylusImport) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问函数定义节点
    fn visit_function(&mut self, node: &crate::ast::StylusFunction) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问条件语句节点
    fn visit_if(&mut self, node: &crate::ast::StylusIf) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问for循环节点
    fn visit_for(&mut self, node: &crate::ast::StylusFor) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问while循环节点
    fn visit_while(&mut self, node: &crate::ast::StylusWhile) -> VisitResult {
        VisitResult::Continue
    }

    /// 访问参数节点
    fn visit_param(&mut self, node: &crate::ast::StylusParam) -> VisitResult {
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
    fn map_root(&mut self, node: &crate::ast::StylusRoot) -> crate::ast::StylusRoot {
        let mut new_root = crate::ast::StylusRoot::new(node.span.clone());
        for item in &node.items {
            new_root.add_item(self.map_item(item));
        }
        new_root
    }

    /// 映射项节点
    fn map_item(&mut self, node: &crate::ast::StylusItem) -> crate::ast::StylusItem {
        match node {
            crate::ast::StylusItem::Rule(rule) => crate::ast::StylusItem::Rule(self.map_rule(rule)),
            crate::ast::StylusItem::Comment(comment) => crate::ast::StylusItem::Comment(self.map_comment(comment)),
            crate::ast::StylusItem::Mixin(mixin) => crate::ast::StylusItem::Mixin(self.map_mixin(mixin)),
            crate::ast::StylusItem::Variable(variable) => crate::ast::StylusItem::Variable(self.map_variable(variable)),
            crate::ast::StylusItem::Import(import) => crate::ast::StylusItem::Import(self.map_import(import)),
            crate::ast::StylusItem::Function(function) => crate::ast::StylusItem::Function(self.map_function(function)),
            crate::ast::StylusItem::If(if_stmt) => crate::ast::StylusItem::If(self.map_if(if_stmt)),
            crate::ast::StylusItem::For(for_stmt) => crate::ast::StylusItem::For(self.map_for(for_stmt)),
            crate::ast::StylusItem::While(while_stmt) => crate::ast::StylusItem::While(self.map_while(while_stmt)),
        }
    }

    /// 映射规则节点
    fn map_rule(&mut self, node: &crate::ast::StylusRule) -> crate::ast::StylusRule {
        let mut new_rule = crate::ast::StylusRule { span: node.span.clone(), selector: node.selector.clone(), properties: Vec::new() };
        for property in &node.properties {
            new_rule.add_property(self.map_property(property));
        }
        new_rule
    }

    /// 映射注释节点
    fn map_comment(&mut self, node: &crate::ast::StylusComment) -> crate::ast::StylusComment {
        crate::ast::StylusComment { span: node.span.clone(), text: node.text.clone() }
    }

    /// 映射属性节点
    fn map_property(&mut self, node: &crate::ast::StylusProperty) -> crate::ast::StylusProperty {
        crate::ast::StylusProperty { span: node.span.clone(), name: node.name.clone(), value: node.value.clone() }
    }

    /// 映射混合定义节点
    fn map_mixin(&mut self, node: &crate::ast::StylusMixin) -> crate::ast::StylusMixin {
        let mut new_mixin = crate::ast::StylusMixin { span: node.span.clone(), name: node.name.clone(), params: Vec::new(), body: Vec::new() };
        for param in &node.params {
            new_mixin.add_param(self.map_param(param));
        }
        for item in &node.body {
            new_mixin.add_body_item(self.map_item(item));
        }
        new_mixin
    }

    /// 映射变量定义节点
    fn map_variable(&mut self, node: &crate::ast::StylusVariable) -> crate::ast::StylusVariable {
        crate::ast::StylusVariable { span: node.span.clone(), name: node.name.clone(), value: node.value.clone() }
    }

    /// 映射导入语句节点
    fn map_import(&mut self, node: &crate::ast::StylusImport) -> crate::ast::StylusImport {
        crate::ast::StylusImport { span: node.span.clone(), path: node.path.clone() }
    }

    /// 映射函数定义节点
    fn map_function(&mut self, node: &crate::ast::StylusFunction) -> crate::ast::StylusFunction {
        let mut new_function = crate::ast::StylusFunction { span: node.span.clone(), name: node.name.clone(), params: Vec::new(), body: Vec::new(), return_value: node.return_value.clone() };
        for param in &node.params {
            new_function.add_param(self.map_param(param));
        }
        for item in &node.body {
            new_function.add_body_item(self.map_item(item));
        }
        new_function
    }

    /// 映射条件语句节点
    fn map_if(&mut self, node: &crate::ast::StylusIf) -> crate::ast::StylusIf {
        let mut new_if = crate::ast::StylusIf { span: node.span.clone(), condition: node.condition.clone(), body: Vec::new(), else_clause: None };
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
    fn map_for(&mut self, node: &crate::ast::StylusFor) -> crate::ast::StylusFor {
        let mut new_for = crate::ast::StylusFor { span: node.span.clone(), variable: node.variable.clone(), range: node.range.clone(), body: Vec::new() };
        for item in &node.body {
            new_for.add_body_item(self.map_item(item));
        }
        new_for
    }

    /// 映射while循环节点
    fn map_while(&mut self, node: &crate::ast::StylusWhile) -> crate::ast::StylusWhile {
        let mut new_while = crate::ast::StylusWhile { span: node.span.clone(), condition: node.condition.clone(), body: Vec::new() };
        for item in &node.body {
            new_while.add_body_item(self.map_item(item));
        }
        new_while
    }

    /// 映射参数节点
    fn map_param(&mut self, node: &crate::ast::StylusParam) -> crate::ast::StylusParam {
        crate::ast::StylusParam { span: node.span.clone(), name: node.name.clone(), default: node.default.clone() }
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
    fn visit_root(&mut self, node: &crate::ast::StylusRoot) -> VisitResult {
        self.print_indent();
        println!(
            "StylusRoot (span: {:?})\n",
            node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_rule(&mut self, node: &crate::ast::StylusRule) -> VisitResult {
        self.print_indent();
        println!(
            "StylusRule (selector: {}, span: {:?})\n",
            node.selector, node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_comment(&mut self, node: &crate::ast::StylusComment) -> VisitResult {
        self.print_indent();
        println!(
            "StylusComment (text: {}, span: {:?})\n",
            node.text, node.span
        );
        VisitResult::Continue
    }

    fn visit_property(&mut self, node: &crate::ast::StylusProperty) -> VisitResult {
        self.print_indent();
        println!(
            "StylusProperty (name: {}, value: {}, span: {:?})\n",
            node.name, node.value, node.span
        );
        VisitResult::Continue
    }

    fn visit_mixin(&mut self, node: &crate::ast::StylusMixin) -> VisitResult {
        self.print_indent();
        println!(
            "StylusMixin (name: {}, span: {:?})\n",
            node.name, node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_variable(&mut self, node: &crate::ast::StylusVariable) -> VisitResult {
        self.print_indent();
        println!(
            "StylusVariable (name: {}, value: {}, span: {:?})\n",
            node.name, node.value, node.span
        );
        VisitResult::Continue
    }

    fn visit_import(&mut self, node: &crate::ast::StylusImport) -> VisitResult {
        self.print_indent();
        println!(
            "StylusImport (path: {}, span: {:?})\n",
            node.path, node.span
        );
        VisitResult::Continue
    }

    fn visit_function(&mut self, node: &crate::ast::StylusFunction) -> VisitResult {
        self.print_indent();
        println!(
            "StylusFunction (name: {}, span: {:?})\n",
            node.name, node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_if(&mut self, node: &crate::ast::StylusIf) -> VisitResult {
        self.print_indent();
        println!(
            "StylusIf (condition: {}, span: {:?})\n",
            node.condition, node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_for(&mut self, node: &crate::ast::StylusFor) -> VisitResult {
        self.print_indent();
        println!(
            "StylusFor (variable: {}, range: {}, span: {:?})\n",
            node.variable, node.range, node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_while(&mut self, node: &crate::ast::StylusWhile) -> VisitResult {
        self.print_indent();
        println!(
            "StylusWhile (condition: {}, span: {:?})\n",
            node.condition, node.span
        );
        self.indent += 1;
        let result = VisitResult::Continue;
        self.indent -= 1;
        result
    }

    fn visit_param(&mut self, node: &crate::ast::StylusParam) -> VisitResult {
        self.print_indent();
        println!(
            "StylusParam (name: {}, default: {:?}, span: {:?})\n",
            node.name, node.default, node.span
        );
        VisitResult::Continue
    }
}
