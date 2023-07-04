use core::range::Range;
use oak_core::tree::{GreenNode, RedNode, TypedNode};

use super::{
    nodes::*,
    visitor::{AcceptVisitor, StylusVisitor, VisitResult},
};

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

impl StylusFor {
    /// Adds an item to the for loop body.
    pub fn add_body_item(&mut self, item: StylusItem) {
        self.body.push(item);
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

impl StylusWhile {
    /// Adds an item to the while loop body.
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
