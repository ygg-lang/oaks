use crate::ast::{FunctionParam, JsxElement, JsxFragment, JsxSelfClosingElement, Statement, TypeAnnotation, TypeParameter};
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Expression {
    pub kind: Box<ExpressionKind>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Range<usize>) -> Self {
        Self { kind: Box::new(kind), span }
    }

    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExpressionKind {
    Identifier(String),
    NumericLiteral(f64),
    StringLiteral(String),
    BigIntLiteral(String),
    BooleanLiteral(bool),
    NullLiteral,
    RegexLiteral(String),
    TemplateString(String),
    UnaryExpression { operator: String, argument: Box<Expression> },
    UpdateExpression { operator: String, argument: Box<Expression>, prefix: bool },
    BinaryExpression { left: Box<Expression>, operator: String, right: Box<Expression> },
    ConditionalExpression { test: Box<Expression>, consequent: Box<Expression>, alternate: Box<Expression> },
    MemberExpression { object: Box<Expression>, property: Box<Expression>, computed: bool, optional: bool },
    CallExpression { func: Box<Expression>, args: Vec<Expression> },
    NewExpression { func: Box<Expression>, args: Vec<Expression> },
    AssignmentExpression { left: Box<Expression>, operator: String, right: Box<Expression> },
    AsExpression { expression: Box<Expression>, type_annotation: TypeAnnotation },
    ArrowFunction { type_params: Vec<TypeParameter>, params: Vec<FunctionParam>, return_type: Option<TypeAnnotation>, body: Box<Statement>, async_: bool },
    ObjectLiteral { properties: Vec<ObjectProperty> },
    ArrayLiteral { elements: Vec<Expression> },
    SpreadElement(Box<Expression>),
    AwaitExpression(Box<Expression>),
    YieldExpression(Option<Box<Expression>>),
    ImportExpression { module_specifier: Box<Expression> },
    FunctionExpression { name: Option<String>, type_params: Vec<TypeParameter>, params: Vec<FunctionParam>, return_type: Option<TypeAnnotation>, body: Vec<Statement>, async_: bool, generator: bool },
    TaggedTemplateExpression { tag: Box<Expression>, template: Box<Expression> },
    TypeAssertionExpression { expression: Box<Expression>, type_annotation: TypeAnnotation },
    NonNullExpression(Box<Expression>),
    JsxElement(Box<JsxElement>),
    JsxFragment(Box<JsxFragment>),
    JsxSelfClosingElement(Box<JsxSelfClosingElement>),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ObjectProperty {
    Property {
        name: String,
        value: Expression,
        shorthand: bool,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    Spread(Expression),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LiteralType {
    String(String),
    Number(f64),
    Boolean(bool),
    BigInt(String),
}
