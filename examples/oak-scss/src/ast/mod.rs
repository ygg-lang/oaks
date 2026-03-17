#![doc = include_str!("readme.md")]
use core::range::Range;

/// SCSS root node
#[derive(Debug, Clone)]
pub struct ScssRoot {
    /// The source range of the root node
    pub span: Range<usize>,
    /// The children nodes
    pub children: Vec<ScssNode>,
}

impl ScssRoot {
    /// Traverses the AST with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        for child in &self.children {
            child.traverse(visitor);
        }
        visitor.visit_root(self);
    }

    /// Mutably traverses the AST with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        for child in &mut self.children {
            child.traverse_mut(visitor);
        }
        visitor.visit_root_mut(self);
    }
}

/// SCSS node types
#[derive(Debug, Clone)]
pub enum ScssNode {
    /// A rule set (selector + block)
    RuleSet(ScssRuleSet),
    /// A mixin declaration
    MixinDeclaration(ScssMixinDeclaration),
    /// A function declaration
    FunctionDeclaration(ScssFunctionDeclaration),
    /// An include statement
    IncludeStatement(ScssIncludeStatement),
    /// An import statement
    ImportStatement(ScssImportStatement),
    /// A variable declaration
    VariableDeclaration(ScssVariableDeclaration),
    /// An if statement
    IfStatement(ScssIfStatement),
    /// A for statement
    ForStatement(ScssForStatement),
    /// An each statement
    EachStatement(ScssEachStatement),
    /// A while statement
    WhileStatement(ScssWhileStatement),
    /// A return statement
    ReturnStatement(ScssReturnStatement),
    /// A comment
    Comment(ScssComment),
    /// An error node
    Error(ScssError),
}

impl ScssNode {
    /// Traverses the node with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        match self {
            ScssNode::RuleSet(node) => node.traverse(visitor),
            ScssNode::MixinDeclaration(node) => node.traverse(visitor),
            ScssNode::FunctionDeclaration(node) => node.traverse(visitor),
            ScssNode::IncludeStatement(node) => node.traverse(visitor),
            ScssNode::ImportStatement(node) => node.traverse(visitor),
            ScssNode::VariableDeclaration(node) => node.traverse(visitor),
            ScssNode::IfStatement(node) => node.traverse(visitor),
            ScssNode::ForStatement(node) => node.traverse(visitor),
            ScssNode::EachStatement(node) => node.traverse(visitor),
            ScssNode::WhileStatement(node) => node.traverse(visitor),
            ScssNode::ReturnStatement(node) => node.traverse(visitor),
            ScssNode::Comment(node) => node.traverse(visitor),
            ScssNode::Error(node) => node.traverse(visitor),
        }
    }

    /// Mutably traverses the node with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        match self {
            ScssNode::RuleSet(node) => node.traverse_mut(visitor),
            ScssNode::MixinDeclaration(node) => node.traverse_mut(visitor),
            ScssNode::FunctionDeclaration(node) => node.traverse_mut(visitor),
            ScssNode::IncludeStatement(node) => node.traverse_mut(visitor),
            ScssNode::ImportStatement(node) => node.traverse_mut(visitor),
            ScssNode::VariableDeclaration(node) => node.traverse_mut(visitor),
            ScssNode::IfStatement(node) => node.traverse_mut(visitor),
            ScssNode::ForStatement(node) => node.traverse_mut(visitor),
            ScssNode::EachStatement(node) => node.traverse_mut(visitor),
            ScssNode::WhileStatement(node) => node.traverse_mut(visitor),
            ScssNode::ReturnStatement(node) => node.traverse_mut(visitor),
            ScssNode::Comment(node) => node.traverse_mut(visitor),
            ScssNode::Error(node) => node.traverse_mut(visitor),
        }
    }
}

/// A rule set consisting of a selector and a block
#[derive(Debug, Clone)]
pub struct ScssRuleSet {
    /// The source range of the rule set
    pub span: Range<usize>,
    /// The selector
    pub selector: ScssSelector,
    /// The block containing declarations and nested rules
    pub block: ScssBlock,
}

impl ScssRuleSet {
    /// Traverses the rule set with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        self.selector.traverse(visitor);
        self.block.traverse(visitor);
        visitor.visit_rule_set(self);
    }

    /// Mutably traverses the rule set with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        self.selector.traverse_mut(visitor);
        self.block.traverse_mut(visitor);
        visitor.visit_rule_set_mut(self);
    }
}

/// A selector
#[derive(Debug, Clone)]
pub struct ScssSelector {
    /// The source range of the selector
    pub span: Range<usize>,
    /// The selector text
    pub text: String,
}

impl ScssSelector {
    /// Traverses the selector with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        visitor.visit_selector(self);
    }

    /// Mutably traverses the selector with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        visitor.visit_selector_mut(self);
    }
}

/// A block containing declarations and nested rules
#[derive(Debug, Clone)]
pub struct ScssBlock {
    /// The source range of the block
    pub span: Range<usize>,
    /// The children nodes (declarations and nested rules)
    pub children: Vec<ScssNode>,
}

impl ScssBlock {
    /// Traverses the block with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        for child in &self.children {
            child.traverse(visitor);
        }
        visitor.visit_block(self);
    }

    /// Mutably traverses the block with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        for child in &mut self.children {
            child.traverse_mut(visitor);
        }
        visitor.visit_block_mut(self);
    }
}

/// A declaration (property: value;)
#[derive(Debug, Clone)]
pub struct ScssDeclaration {
    /// The source range of the declaration
    pub span: Range<usize>,
    /// The property
    pub property: ScssProperty,
    /// The value
    pub value: ScssValue,
}

impl ScssDeclaration {
    /// Traverses the declaration with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        self.property.traverse(visitor);
        self.value.traverse(visitor);
        visitor.visit_declaration(self);
    }

    /// Mutably traverses the declaration with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        self.property.traverse_mut(visitor);
        self.value.traverse_mut(visitor);
        visitor.visit_declaration_mut(self);
    }
}

/// A property
#[derive(Debug, Clone)]
pub struct ScssProperty {
    /// The source range of the property
    pub span: Range<usize>,
    /// The property name
    pub name: String,
}

impl ScssProperty {
    /// Traverses the property with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        visitor.visit_property(self);
    }

    /// Mutably traverses the property with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        visitor.visit_property_mut(self);
    }
}

/// A value
#[derive(Debug, Clone)]
pub struct ScssValue {
    /// The source range of the value
    pub span: Range<usize>,
    /// The value text
    pub text: String,
}

impl ScssValue {
    /// Traverses the value with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        visitor.visit_value(self);
    }

    /// Mutably traverses the value with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        visitor.visit_value_mut(self);
    }
}

/// A mixin declaration
#[derive(Debug, Clone)]
pub struct ScssMixinDeclaration {
    /// The source range of the mixin declaration
    pub span: Range<usize>,
    /// The mixin name
    pub name: String,
    /// The parameters
    pub parameters: Vec<ScssParameter>,
    /// The block
    pub block: ScssBlock,
}

impl ScssMixinDeclaration {
    /// Traverses the mixin declaration with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        for param in &self.parameters {
            param.traverse(visitor);
        }
        self.block.traverse(visitor);
        visitor.visit_mixin_declaration(self);
    }

    /// Mutably traverses the mixin declaration with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        for param in &mut self.parameters {
            param.traverse_mut(visitor);
        }
        self.block.traverse_mut(visitor);
        visitor.visit_mixin_declaration_mut(self);
    }
}

/// A function declaration
#[derive(Debug, Clone)]
pub struct ScssFunctionDeclaration {
    /// The source range of the function declaration
    pub span: Range<usize>,
    /// The function name
    pub name: String,
    /// The parameters
    pub parameters: Vec<ScssParameter>,
    /// The block
    pub block: ScssBlock,
}

impl ScssFunctionDeclaration {
    /// Traverses the function declaration with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        for param in &self.parameters {
            param.traverse(visitor);
        }
        self.block.traverse(visitor);
        visitor.visit_function_declaration(self);
    }

    /// Mutably traverses the function declaration with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        for param in &mut self.parameters {
            param.traverse_mut(visitor);
        }
        self.block.traverse_mut(visitor);
        visitor.visit_function_declaration_mut(self);
    }
}

/// A parameter
#[derive(Debug, Clone)]
pub struct ScssParameter {
    /// The source range of the parameter
    pub span: Range<usize>,
    /// The parameter name
    pub name: String,
    /// The default value (optional)
    pub default_value: Option<ScssValue>,
}

impl ScssParameter {
    /// Traverses the parameter with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        if let Some(value) = &self.default_value {
            value.traverse(visitor);
        }
        visitor.visit_parameter(self);
    }

    /// Mutably traverses the parameter with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        if let Some(value) = &mut self.default_value {
            value.traverse_mut(visitor);
        }
        visitor.visit_parameter_mut(self);
    }
}

/// An include statement
#[derive(Debug, Clone)]
pub struct ScssIncludeStatement {
    /// The source range of the include statement
    pub span: Range<usize>,
    /// The mixin name
    pub name: String,
    /// The arguments
    pub arguments: Vec<ScssArgument>,
    /// The block (optional)
    pub block: Option<ScssBlock>,
}

impl ScssIncludeStatement {
    /// Traverses the include statement with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        for arg in &self.arguments {
            arg.traverse(visitor);
        }
        if let Some(block) = &self.block {
            block.traverse(visitor);
        }
        visitor.visit_include_statement(self);
    }

    /// Mutably traverses the include statement with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        for arg in &mut self.arguments {
            arg.traverse_mut(visitor);
        }
        if let Some(block) = &mut self.block {
            block.traverse_mut(visitor);
        }
        visitor.visit_include_statement_mut(self);
    }
}

/// An argument
#[derive(Debug, Clone)]
pub struct ScssArgument {
    /// The source range of the argument
    pub span: Range<usize>,
    /// The argument value
    pub value: ScssValue,
}

impl ScssArgument {
    /// Traverses the argument with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        self.value.traverse(visitor);
        visitor.visit_argument(self);
    }

    /// Mutably traverses the argument with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        self.value.traverse_mut(visitor);
        visitor.visit_argument_mut(self);
    }
}

/// An import statement
#[derive(Debug, Clone)]
pub struct ScssImportStatement {
    /// The source range of the import statement
    pub span: Range<usize>,
    /// The import path
    pub path: String,
}

impl ScssImportStatement {
    /// Traverses the import statement with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        visitor.visit_import_statement(self);
    }

    /// Mutably traverses the import statement with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        visitor.visit_import_statement_mut(self);
    }
}

/// A variable declaration
#[derive(Debug, Clone)]
pub struct ScssVariableDeclaration {
    /// The source range of the variable declaration
    pub span: Range<usize>,
    /// The variable name
    pub name: String,
    /// The variable value
    pub value: ScssValue,
}

impl ScssVariableDeclaration {
    /// Traverses the variable declaration with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        self.value.traverse(visitor);
        visitor.visit_variable_declaration(self);
    }

    /// Mutably traverses the variable declaration with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        self.value.traverse_mut(visitor);
        visitor.visit_variable_declaration_mut(self);
    }
}

/// An if statement
#[derive(Debug, Clone)]
pub struct ScssIfStatement {
    /// The source range of the if statement
    pub span: Range<usize>,
    /// The condition
    pub condition: String,
    /// The then block
    pub then_block: ScssBlock,
    /// The else block (optional)
    pub else_block: Option<ScssBlock>,
}

impl ScssIfStatement {
    /// Traverses the if statement with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        self.then_block.traverse(visitor);
        if let Some(block) = &self.else_block {
            block.traverse(visitor);
        }
        visitor.visit_if_statement(self);
    }

    /// Mutably traverses the if statement with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        self.then_block.traverse_mut(visitor);
        if let Some(block) = &mut self.else_block {
            block.traverse_mut(visitor);
        }
        visitor.visit_if_statement_mut(self);
    }
}

/// A for statement
#[derive(Debug, Clone)]
pub struct ScssForStatement {
    /// The source range of the for statement
    pub span: Range<usize>,
    /// The variable name
    pub variable: String,
    /// The start value
    pub start: String,
    /// The end value
    pub end: String,
    /// Whether it's an inclusive range (through) or exclusive (to)
    pub inclusive: bool,
    /// The block
    pub block: ScssBlock,
}

impl ScssForStatement {
    /// Traverses the for statement with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        self.block.traverse(visitor);
        visitor.visit_for_statement(self);
    }

    /// Mutably traverses the for statement with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        self.block.traverse_mut(visitor);
        visitor.visit_for_statement_mut(self);
    }
}

/// An each statement
#[derive(Debug, Clone)]
pub struct ScssEachStatement {
    /// The source range of the each statement
    pub span: Range<usize>,
    /// The variable names
    pub variables: Vec<String>,
    /// The list expression
    pub list: String,
    /// The block
    pub block: ScssBlock,
}

impl ScssEachStatement {
    /// Traverses the each statement with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        self.block.traverse(visitor);
        visitor.visit_each_statement(self);
    }

    /// Mutably traverses the each statement with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        self.block.traverse_mut(visitor);
        visitor.visit_each_statement_mut(self);
    }
}

/// A while statement
#[derive(Debug, Clone)]
pub struct ScssWhileStatement {
    /// The source range of the while statement
    pub span: Range<usize>,
    /// The condition
    pub condition: String,
    /// The block
    pub block: ScssBlock,
}

impl ScssWhileStatement {
    /// Traverses the while statement with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        self.block.traverse(visitor);
        visitor.visit_while_statement(self);
    }

    /// Mutably traverses the while statement with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        self.block.traverse_mut(visitor);
        visitor.visit_while_statement_mut(self);
    }
}

/// A return statement
#[derive(Debug, Clone)]
pub struct ScssReturnStatement {
    /// The source range of the return statement
    pub span: Range<usize>,
    /// The return value
    pub value: Option<ScssValue>,
}

impl ScssReturnStatement {
    /// Traverses the return statement with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        if let Some(value) = &self.value {
            value.traverse(visitor);
        }
        visitor.visit_return_statement(self);
    }

    /// Mutably traverses the return statement with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        if let Some(value) = &mut self.value {
            value.traverse_mut(visitor);
        }
        visitor.visit_return_statement_mut(self);
    }
}

/// A comment
#[derive(Debug, Clone)]
pub struct ScssComment {
    /// The source range of the comment
    pub span: Range<usize>,
    /// The comment text
    pub text: String,
}

impl ScssComment {
    /// Traverses the comment with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        visitor.visit_comment(self);
    }

    /// Mutably traverses the comment with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        visitor.visit_comment_mut(self);
    }
}

/// An error node
#[derive(Debug, Clone)]
pub struct ScssError {
    /// The source range of the error
    pub span: Range<usize>,
    /// The error message
    pub message: String,
}

impl ScssError {
    /// Traverses the error with the given visitor.
    pub fn traverse<V: ScssVisitor>(&self, visitor: &mut V) {
        visitor.visit_error(self);
    }

    /// Mutably traverses the error with the given visitor.
    pub fn traverse_mut<V: ScssVisitorMut>(&mut self, visitor: &mut V) {
        visitor.visit_error_mut(self);
    }
}

/// Visitor trait for SCSS AST nodes.
pub trait ScssVisitor {
    /// Visits the root node.
    fn visit_root(&mut self, _node: &ScssRoot) {}
    /// Visits a rule set node.
    fn visit_rule_set(&mut self, _node: &ScssRuleSet) {}
    /// Visits a selector node.
    fn visit_selector(&mut self, _node: &ScssSelector) {}
    /// Visits a block node.
    fn visit_block(&mut self, _node: &ScssBlock) {}
    /// Visits a declaration node.
    fn visit_declaration(&mut self, _node: &ScssDeclaration) {}
    /// Visits a property node.
    fn visit_property(&mut self, _node: &ScssProperty) {}
    /// Visits a value node.
    fn visit_value(&mut self, _node: &ScssValue) {}
    /// Visits a mixin declaration node.
    fn visit_mixin_declaration(&mut self, _node: &ScssMixinDeclaration) {}
    /// Visits a function declaration node.
    fn visit_function_declaration(&mut self, _node: &ScssFunctionDeclaration) {}
    /// Visits a parameter node.
    fn visit_parameter(&mut self, _node: &ScssParameter) {}
    /// Visits an include statement node.
    fn visit_include_statement(&mut self, _node: &ScssIncludeStatement) {}
    /// Visits an argument node.
    fn visit_argument(&mut self, _node: &ScssArgument) {}
    /// Visits an import statement node.
    fn visit_import_statement(&mut self, _node: &ScssImportStatement) {}
    /// Visits a variable declaration node.
    fn visit_variable_declaration(&mut self, _node: &ScssVariableDeclaration) {}
    /// Visits an if statement node.
    fn visit_if_statement(&mut self, _node: &ScssIfStatement) {}
    /// Visits a for statement node.
    fn visit_for_statement(&mut self, _node: &ScssForStatement) {}
    /// Visits an each statement node.
    fn visit_each_statement(&mut self, _node: &ScssEachStatement) {}
    /// Visits a while statement node.
    fn visit_while_statement(&mut self, _node: &ScssWhileStatement) {}
    /// Visits a return statement node.
    fn visit_return_statement(&mut self, _node: &ScssReturnStatement) {}
    /// Visits a comment node.
    fn visit_comment(&mut self, _node: &ScssComment) {}
    /// Visits an error node.
    fn visit_error(&mut self, _node: &ScssError) {}
}

/// Mutable visitor trait for SCSS AST nodes.
pub trait ScssVisitorMut {
    /// Visits the root node mutably.
    fn visit_root_mut(&mut self, _node: &mut ScssRoot) {}
    /// Visits a rule set node mutably.
    fn visit_rule_set_mut(&mut self, _node: &mut ScssRuleSet) {}
    /// Visits a selector node mutably.
    fn visit_selector_mut(&mut self, _node: &mut ScssSelector) {}
    /// Visits a block node mutably.
    fn visit_block_mut(&mut self, _node: &mut ScssBlock) {}
    /// Visits a declaration node mutably.
    fn visit_declaration_mut(&mut self, _node: &mut ScssDeclaration) {}
    /// Visits a property node mutably.
    fn visit_property_mut(&mut self, _node: &mut ScssProperty) {}
    /// Visits a value node mutably.
    fn visit_value_mut(&mut self, _node: &mut ScssValue) {}
    /// Visits a mixin declaration node mutably.
    fn visit_mixin_declaration_mut(&mut self, _node: &mut ScssMixinDeclaration) {}
    /// Visits a function declaration node mutably.
    fn visit_function_declaration_mut(&mut self, _node: &mut ScssFunctionDeclaration) {}
    /// Visits a parameter node mutably.
    fn visit_parameter_mut(&mut self, _node: &mut ScssParameter) {}
    /// Visits an include statement node mutably.
    fn visit_include_statement_mut(&mut self, _node: &mut ScssIncludeStatement) {}
    /// Visits an argument node mutably.
    fn visit_argument_mut(&mut self, _node: &mut ScssArgument) {}
    /// Visits an import statement node mutably.
    fn visit_import_statement_mut(&mut self, _node: &mut ScssImportStatement) {}
    /// Visits a variable declaration node mutably.
    fn visit_variable_declaration_mut(&mut self, _node: &mut ScssVariableDeclaration) {}
    /// Visits an if statement node mutably.
    fn visit_if_statement_mut(&mut self, _node: &mut ScssIfStatement) {}
    /// Visits a for statement node mutably.
    fn visit_for_statement_mut(&mut self, _node: &mut ScssForStatement) {}
    /// Visits an each statement node mutably.
    fn visit_each_statement_mut(&mut self, _node: &mut ScssEachStatement) {}
    /// Visits a while statement node mutably.
    fn visit_while_statement_mut(&mut self, _node: &mut ScssWhileStatement) {}
    /// Visits a return statement node mutably.
    fn visit_return_statement_mut(&mut self, _node: &mut ScssReturnStatement) {}
    /// Visits a comment node mutably.
    fn visit_comment_mut(&mut self, _node: &mut ScssComment) {}
    /// Visits an error node mutably.
    fn visit_error_mut(&mut self, _node: &mut ScssError) {}
}
