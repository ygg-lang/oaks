use crate::lexer::token_type::DejavuTokenType;
use oak_core::Range;

/// Root node of a Dejavu AST.
#[derive(Debug, Clone, PartialEq)]
pub struct DejavuRoot {
    /// Top-level items in the source file.
    pub items: Vec<ItemNode>,
}

/// Identifier node representing a name.
#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierNode {
    /// The identifier name.
    pub name: String,
    /// Source span of the identifier.
    pub span: Range<usize>,
}

/// Name path node representing a qualified path like `foo::bar::baz`.
#[derive(Debug, Clone, PartialEq)]
pub struct NamePathNode {
    /// Parts of the path.
    pub parts: Vec<IdentifierNode>,
    /// Source span of the path.
    pub span: Range<usize>,
}

/// Attribute node representing an annotation like `#[derive(Clone)]`.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeNode {
    /// Attribute name.
    pub name: IdentifierNode,
    /// Attribute arguments.
    pub args: Vec<ExpressionNode>,
    /// Source span of the attribute.
    pub span: Range<usize>,
}

/// Block node containing a sequence of statements.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockNode {
    /// Statements in the block.
    pub statements: Vec<StatementNode>,
    /// Source span of the block.
    pub span: Range<usize>,
}

/// Parameter node for function definitions.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterNode {
    /// Parameter annotations.
    pub annotations: Vec<AttributeNode>,
    /// Parameter name.
    pub name: IdentifierNode,
    /// Optional type annotation.
    pub ty: Option<String>,
    /// Source span of the parameter.
    pub span: Range<usize>,
}

/// Raw block node for raw text blocks.
#[derive(Debug, Clone, PartialEq)]
pub struct RawBlockNode {
    /// Raw text content.
    pub content: String,
    /// Source span of the block.
    pub span: Range<usize>,
}

/// Template text node for plain template text.
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateTextNode {
    /// Template text content.
    pub content: String,
    /// Source span of the text.
    pub span: Range<usize>,
}

/// Include directive node for including other files.
#[derive(Debug, Clone, PartialEq)]
pub struct IncludeDirectiveNode {
    /// Path to the included file.
    pub path: ExpressionNode,
    /// Source span of the directive.
    pub span: Range<usize>,
}

/// Block node for block declarations.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockDeclaration {
    /// Block name.
    pub name: IdentifierNode,
    /// Block annotations.
    pub annotations: Vec<AttributeNode>,
    /// Block items.
    pub items: Vec<ItemNode>,
    /// Source span of the block.
    pub span: Range<usize>,
}

/// Item node representing top-level declarations.
#[derive(Debug, Clone, PartialEq)]
pub enum ItemNode {
    /// Namespace declaration.
    Namespace(NamespaceDeclaration),
    /// Class declaration.
    Class(ClassDeclaration),
    /// Flags declaration.
    Flags(FlagsDeclaration),
    /// Enum declaration.
    Enum(EnumDeclaration),
    /// Trait declaration.
    Trait(TraitDeclaration),
    /// Widget declaration.
    Widget(WidgetDeclaration),
    /// Using statement.
    Using(UsingStatement),
    /// Micro definition.
    Micro(MicroDefinition),
    /// Type function definition.
    TypeFunction(TypeFunctionDefinition),
    /// Statement item.
    Statement(StatementNode),
    /// Variant definition.
    Variant(VariantDefinition),
    /// Template control node.
    TemplateControl(TemplateControlNode),
    /// Template interpolation node.
    TemplateInterpolation(TemplateInterpolationNode),
    /// Raw block node.
    RawBlock(RawBlockNode),
    /// Include directive node.
    IncludeDirective(IncludeDirectiveNode),
    /// Block declaration.
    Block(BlockDeclaration),
    /// Template text node.
    TemplateText(TemplateTextNode),
    /// For loop control node.
    ForControl(ForControlNode),
    /// If conditional control node.
    IfControl(IfControlNode),
    /// While loop control node.
    WhileControl(WhileControlNode),
    /// Loop control node.
    LoopControl(LoopControlNode),
}

/// Namespace declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceDeclaration {
    /// Namespace name.
    pub name: NamePathNode,
    /// Namespace annotations.
    pub annotations: Vec<AttributeNode>,
    /// Items in the namespace.
    pub items: Vec<ItemNode>,
    /// Source span of the declaration.
    pub span: Range<usize>,
}

/// Class declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDeclaration {
    /// Class name.
    pub name: IdentifierNode,
    /// Class annotations.
    pub annotations: Vec<AttributeNode>,
    /// Parent classes/traits.
    pub parents: Vec<NamePathNode>,
    /// Items in the class.
    pub items: Vec<ItemNode>,
    /// Source span of the declaration.
    pub span: Range<usize>,
}

/// Flags declaration for bitflag enums.
#[derive(Debug, Clone, PartialEq)]
pub struct FlagsDeclaration {
    /// Flags name.
    pub name: IdentifierNode,
    /// Flags annotations.
    pub annotations: Vec<AttributeNode>,
    /// Flag variants.
    pub items: Vec<ItemNode>,
    /// Source span of the declaration.
    pub span: Range<usize>,
}

/// Enum declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDeclaration {
    /// Enum name.
    pub name: IdentifierNode,
    /// Enum annotations.
    pub annotations: Vec<AttributeNode>,
    /// Enum variants.
    pub items: Vec<ItemNode>,
    /// Source span of the declaration.
    pub span: Range<usize>,
}

/// Trait declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct TraitDeclaration {
    /// Trait name.
    pub name: IdentifierNode,
    /// Trait annotations.
    pub annotations: Vec<AttributeNode>,
    /// Parent traits.
    pub parents: Vec<NamePathNode>,
    /// Items in the trait.
    pub items: Vec<ItemNode>,
    /// Source span of the declaration.
    pub span: Range<usize>,
}

/// Widget declaration for UI components.
#[derive(Debug, Clone, PartialEq)]
pub struct WidgetDeclaration {
    /// Widget name.
    pub name: IdentifierNode,
    /// Widget annotations.
    pub annotations: Vec<AttributeNode>,
    /// Items in the widget.
    pub items: Vec<ItemNode>,
    /// Source span of the declaration.
    pub span: Range<usize>,
}

/// Using statement for imports.
#[derive(Debug, Clone, PartialEq)]
pub struct UsingStatement {
    /// Import path.
    pub path: NamePathNode,
    /// Source span of the statement.
    pub span: Range<usize>,
}

/// Micro definition for small functions.
#[derive(Debug, Clone, PartialEq)]
pub struct MicroDefinition {
    /// Function name.
    pub name: IdentifierNode,
    /// Function annotations.
    pub annotations: Vec<AttributeNode>,
    /// Function parameters.
    pub params: Vec<ParameterNode>,
    /// Return type annotation.
    pub return_type: Option<String>,
    /// Function body.
    pub body: BlockNode,
    /// Source span of the definition.
    pub span: Range<usize>,
}

/// Type function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeFunctionDefinition {
    /// Function name.
    pub name: IdentifierNode,
    /// Function annotations.
    pub annotations: Vec<AttributeNode>,
    /// Function parameters.
    pub params: Vec<ParameterNode>,
    /// Return type annotation.
    pub return_type: Option<String>,
    /// Function body.
    pub body: BlockNode,
    /// Source span of the definition.
    pub span: Range<usize>,
}

/// Variant definition for enum variants.
#[derive(Debug, Clone, PartialEq)]
pub struct VariantDefinition {
    /// Variant name.
    pub name: IdentifierNode,
    /// Variant annotations.
    pub annotations: Vec<AttributeNode>,
    /// Optional variant value.
    pub value: Option<ExpressionNode>,
    /// Source span of the definition.
    pub span: Range<usize>,
}

/// Template control node for template directives.
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateControlNode {
    /// Template items.
    pub items: Vec<ItemNode>,
    /// Source span of the node.
    pub span: Range<usize>,
}

/// Else branch node for if control structures.
#[derive(Debug, Clone, PartialEq)]
pub enum ElseBranchNode {
    /// Else if branch.
    Elif {
        /// Condition expression.
        condition: ExpressionNode,
        /// Branch body.
        body: Vec<ItemNode>,
        /// Nested else branch.
        else_branch: Option<Box<ElseBranchNode>>,
    },
    /// Else branch.
    Else {
        /// Branch body.
        body: Vec<ItemNode>,
    },
}

/// For loop control node.
#[derive(Debug, Clone, PartialEq)]
pub struct ForControlNode {
    /// Loop variable pattern (e.g., item or (index, item)).
    pub pattern: PatternNode,
    /// Iterable expression.
    pub iterable: ExpressionNode,
    /// Loop body.
    pub body: Vec<ItemNode>,
    /// Optional else branch (executed when iterator is empty).
    pub else_body: Option<Vec<ItemNode>>,
    /// Source span.
    pub span: Range<usize>,
}

/// If conditional control node.
#[derive(Debug, Clone, PartialEq)]
pub struct IfControlNode {
    /// Condition expression.
    pub condition: ExpressionNode,
    /// Then branch.
    pub then_body: Vec<ItemNode>,
    /// Else branch (can be else or else if).
    pub else_branch: Option<ElseBranchNode>,
    /// Source span.
    pub span: Range<usize>,
}

/// While loop control node.
#[derive(Debug, Clone, PartialEq)]
pub struct WhileControlNode {
    /// Condition expression.
    pub condition: ExpressionNode,
    /// Loop body.
    pub body: Vec<ItemNode>,
    /// Source span.
    pub span: Range<usize>,
}

/// Template interpolation node for embedded expressions.
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateInterpolationNode {
    /// Interpolated expression.
    pub expr: ExpressionNode,
    /// Source span of the node.
    pub span: Range<usize>,
}

/// Statement node.
#[derive(Debug, Clone, PartialEq)]
pub enum StatementNode {
    /// Let binding statement.
    Let(LetStatement),
    /// Expression statement.
    Expr(ExpressionStatement),
}

/// Let binding statement.
#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    /// Statement annotations.
    pub annotations: Vec<AttributeNode>,
    /// Whether the binding is mutable.
    pub is_mutable: bool,
    /// Binding pattern.
    pub pattern: PatternNode,
    /// Bound expression.
    pub expr: ExpressionNode,
    /// Source span of the statement.
    pub span: Range<usize>,
}

/// Expression statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    /// Statement annotations.
    pub annotations: Vec<AttributeNode>,
    /// The expression.
    pub expr: ExpressionNode,
    /// Whether a semicolon is present.
    pub semi: bool,
    /// Source span of the statement.
    pub span: Range<usize>,
}

/// Expression node.
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    /// Identifier expression.
    Ident(IdentifierNode),
    /// Path expression.
    Path(NamePathNode),
    /// Literal expression.
    Literal(LiteralExpressionNode),
    /// Boolean literal.
    Bool(BooleanLiteralNode),
    /// Parenthesized expression.
    Paren(ParenthesizedExpressionNode),
    /// Unary expression.
    Unary(UnaryExpressionNode),
    /// Binary expression.
    Binary(BinaryExpressionNode),
    /// Call expression.
    Call(CallExpressionNode),
    /// Field access expression.
    Field(FieldExpressionNode),
    /// Index expression.
    Index(IndexExpressionNode),
    /// If expression.
    If(IfExpressionNode),
    /// Match expression.
    Match(MatchExpressionNode),
    /// Lambda expression.
    Lambda(LambdaExpressionNode),
    /// Object expression.
    Object(ObjectExpressionNode),
    /// Block expression.
    Block(BlockNode),
    /// Loop expression.
    Loop(LoopExpressionNode),
    /// Return expression.
    Return(ReturnExpressionNode),
    /// Break expression.
    Break(BreakExpressionNode),
    /// Continue expression.
    Continue(ContinueExpressionNode),
    /// Yield expression.
    Yield(YieldExpressionNode),
    /// Raise expression.
    Raise(RaiseExpressionNode),
    /// Catch expression.
    Catch(CatchExpressionNode),
    /// Resume expression.
    Resume(ResumeExpressionNode),
    /// Filter expression.
    Filter(FilterExpressionNode),
    /// Translate expression for internationalization.
    Translate(TranslateExpressionNode),
}

/// Translate expression node for internationalization.
#[derive(Debug, Clone, PartialEq)]
pub struct TranslateExpressionNode {
    /// Translation key.
    pub key: String,
    /// Translation arguments.
    pub args: Vec<(String, ExpressionNode)>,
    /// Plural count for plural forms.
    pub plural: Option<Box<ExpressionNode>>,
    /// Context for context-dependent translations.
    pub context: Option<String>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Literal expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpressionNode {
    /// Literal value.
    pub value: String,
    /// Source span of the literal.
    pub span: Range<usize>,
}

/// Boolean literal node.
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteralNode {
    /// Boolean value.
    pub value: bool,
    /// Source span of the literal.
    pub span: Range<usize>,
}

/// Parenthesized expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct ParenthesizedExpressionNode {
    /// Inner expression.
    pub expr: Box<ExpressionNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Unary expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpressionNode {
    /// Unary operator.
    pub op: DejavuTokenType,
    /// Operand expression.
    pub expr: Box<ExpressionNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Binary expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpressionNode {
    /// Left operand.
    pub left: Box<ExpressionNode>,
    /// Binary operator.
    pub op: DejavuTokenType,
    /// Right operand.
    pub right: Box<ExpressionNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Call expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct CallExpressionNode {
    /// Callee expression.
    pub callee: Box<ExpressionNode>,
    /// Call arguments.
    pub args: Vec<ExpressionNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Field access expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldExpressionNode {
    /// Receiver expression.
    pub receiver: Box<ExpressionNode>,
    /// Field name.
    pub field: IdentifierNode,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Index expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpressionNode {
    /// Receiver expression.
    pub receiver: Box<ExpressionNode>,
    /// Index expression.
    pub index: Box<ExpressionNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// If expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct IfExpressionNode {
    /// Optional pattern for condition binding.
    pub pattern: Option<PatternNode>,
    /// Condition expression.
    pub condition: Box<ExpressionNode>,
    /// Then branch.
    pub then_branch: BlockNode,
    /// Optional else branch.
    pub else_branch: Option<BlockNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Match expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpressionNode {
    /// Scrutinee expression.
    pub scrutinee: Box<ExpressionNode>,
    /// Match arms.
    pub arms: Vec<MatchArmNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Lambda expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct LambdaExpressionNode {
    /// Lambda parameters.
    pub params: Vec<ParameterNode>,
    /// Return type annotation.
    pub return_type: Option<String>,
    /// Lambda body.
    pub body: BlockNode,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Object expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectExpressionNode {
    /// Callee expression.
    pub callee: Box<ExpressionNode>,
    /// Object block.
    pub block: BlockNode,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Loop expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct LoopExpressionNode {
    /// Optional loop label.
    pub label: Option<String>,
    /// Optional pattern for condition binding.
    pub pattern: Option<PatternNode>,
    /// Optional condition expression.
    pub condition: Option<Box<ExpressionNode>>,
    /// Loop body.
    pub body: BlockNode,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Return expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnExpressionNode {
    /// Optional return value.
    pub expr: Option<Box<ExpressionNode>>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Break expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct BreakExpressionNode {
    /// Optional target label.
    pub label: Option<String>,
    /// Optional break value.
    pub expr: Option<Box<ExpressionNode>>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Continue expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct ContinueExpressionNode {
    /// Optional target label.
    pub label: Option<String>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Yield expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct YieldExpressionNode {
    /// Optional yielded value.
    pub expr: Option<Box<ExpressionNode>>,
    /// Whether this is a yield from.
    pub yield_from: bool,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Raise expression node for throwing errors.
#[derive(Debug, Clone, PartialEq)]
pub struct RaiseExpressionNode {
    /// Error expression.
    pub expr: Box<ExpressionNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Catch expression node for error handling.
#[derive(Debug, Clone, PartialEq)]
pub struct CatchExpressionNode {
    /// Optional return type.
    pub return_type: Option<NamePathNode>,
    /// Expression to catch errors from.
    pub expr: Box<ExpressionNode>,
    /// Catch arms.
    pub arms: Vec<MatchArmNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Resume expression node for error recovery.
#[derive(Debug, Clone, PartialEq)]
pub struct ResumeExpressionNode {
    /// Optional resume value.
    pub expr: Option<Box<ExpressionNode>>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Filter expression node for applying filters to values.
#[derive(Debug, Clone, PartialEq)]
pub struct FilterExpressionNode {
    /// The expression to apply the filter to.
    pub expr: Box<ExpressionNode>,
    /// The filter name.
    pub name: IdentifierNode,
    /// Filter arguments.
    pub args: Vec<ExpressionNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Pattern node for pattern matching.
#[derive(Debug, Clone, PartialEq)]
pub enum PatternNode {
    /// Variable pattern.
    Variable(VariablePatternNode),
    /// Wildcard pattern.
    Wildcard(WildcardPatternNode),
    /// Literal pattern.
    Literal(LiteralPatternNode),
    /// Tuple pattern.
    Tuple(TuplePatternNode),
    /// Array pattern.
    Array(ArrayPatternNode),
    /// Object pattern.
    Object(ObjectPatternNode),
    /// Or pattern.
    Or(OrPatternNode),
    /// Type pattern.
    Type(TypePatternNode),
    /// Class pattern.
    Class(ClassPatternNode),
}

/// Variable pattern node.
#[derive(Debug, Clone, PartialEq)]
pub struct VariablePatternNode {
    /// Variable name.
    pub name: IdentifierNode,
    /// Source span of the pattern.
    pub span: Range<usize>,
}

/// Wildcard pattern node.
#[derive(Debug, Clone, PartialEq)]
pub struct WildcardPatternNode {
    /// Source span of the pattern.
    pub span: Range<usize>,
}

/// Literal pattern node.
#[derive(Debug, Clone, PartialEq)]
pub struct LiteralPatternNode {
    /// Literal value.
    pub value: String,
    /// Source span of the pattern.
    pub span: Range<usize>,
}

/// Tuple pattern node.
#[derive(Debug, Clone, PartialEq)]
pub struct TuplePatternNode {
    /// Tuple element patterns.
    pub items: Vec<PatternNode>,
    /// Source span of the pattern.
    pub span: Range<usize>,
}

/// Array pattern node.
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayPatternNode {
    /// Array element patterns.
    pub items: Vec<PatternNode>,
    /// Source span of the pattern.
    pub span: Range<usize>,
}

/// Object pattern node.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectPatternNode {
    /// Object property patterns.
    pub props: Vec<(IdentifierNode, PatternNode)>,
    /// Source span of the pattern.
    pub span: Range<usize>,
}

/// Or pattern node for alternatives.
#[derive(Debug, Clone, PartialEq)]
pub struct OrPatternNode {
    /// Alternative patterns.
    pub patterns: Vec<PatternNode>,
    /// Source span of the pattern.
    pub span: Range<usize>,
}

/// Type pattern node.
#[derive(Debug, Clone, PartialEq)]
pub struct TypePatternNode {
    /// Type name.
    pub name: NamePathNode,
    /// Source span of the pattern.
    pub span: Range<usize>,
}

/// Class pattern node.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassPatternNode {
    /// Class name.
    pub name: NamePathNode,
    /// Field patterns.
    pub fields: Vec<(IdentifierNode, PatternNode)>,
    /// Source span of the pattern.
    pub span: Range<usize>,
}

/// Match arm node.
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArmNode {
    /// Match pattern.
    pub pattern: PatternNode,
    /// Optional guard expression.
    pub guard: Option<Box<ExpressionNode>>,
    /// Arm body.
    pub body: Box<ExpressionNode>,
    /// Source span of the arm.
    pub span: Range<usize>,
}

/// Effect definition node.
#[derive(Debug, Clone, PartialEq)]
pub struct EffectDefinition {
    /// Effect name.
    pub name: IdentifierNode,
    /// Effect annotations.
    pub annotations: Vec<AttributeNode>,
    /// Effect items.
    pub items: Vec<ItemNode>,
    /// Source span of the definition.
    pub span: Range<usize>,
}
