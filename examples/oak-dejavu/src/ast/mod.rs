use oak_core::Range;

/// Dejavu root node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DejavuRoot {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Namespace(Namespace),
    Class(Class),
    Flags(Flags),
    Enums(Enums),
    Trait(Trait),
    Widget(Widget),
    Using(Using),
    Micro(Micro),
    TypeFunction(Mezzo),
    Statement(Statement),
    Variant(Variant),
    Effect(Effect),
    TemplateText { content: String, span: Range<usize> },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Namespace {
    pub name: NamePath,
    pub items: Vec<Item>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Class {
    // TODO: fields
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Flags {
    // TODO: fields
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Enums {
    // TODO: fields
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Trait {
    // TODO: fields
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Widget {
    // TODO: fields
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Using {
    // TODO: fields
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Micro {
    // TODO: fields
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mezzo {
    // TODO: fields
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    ExprStmt { annotations: Vec<Attribute>, expr: Expr, semi: bool, span: Range<usize> },
    // TODO: other variants
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    // TODO: fields
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Effect {
    // TODO: fields
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamePath {
    pub parts: Vec<String>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    // TODO: fields
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Ident(Identifier),
    // TODO: other variants
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
    pub span: Range<usize>,
}
