/// The root node of an Ada kind tree
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaRoot {
    /// The items in this Ada file
    pub items: Vec<AdaItem>,
}

impl AdaRoot {
    /// Create a new Ada root
    pub fn new(items: Vec<AdaItem>) -> Self {
        Self { items }
    }

    /// Get all top-level items in this Ada file
    pub fn items(&self) -> &[AdaItem] {
        &self.items
    }
}

/// An item in an Ada file (package, procedure, function, etc.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdaItem {
    Package(AdaPackage),
    Procedure(AdaProcedure),
    Function(AdaFunction),
    Type(AdaTypeDeclaration),
    Variable(AdaVariableDeclaration),
}

/// An Ada package declaration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaPackage {
    pub name: String,
}

impl AdaPackage {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// An Ada procedure declaration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaProcedure {
    pub name: String,
    pub parameters: Vec<AdaParameter>,
}

impl AdaProcedure {
    pub fn new(name: String, parameters: Vec<AdaParameter>) -> Self {
        Self { name, parameters }
    }
}

/// An Ada function declaration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaFunction {
    pub name: String,
    pub parameters: Vec<AdaParameter>,
    pub return_type: String,
}

impl AdaFunction {
    pub fn new(name: String, parameters: Vec<AdaParameter>, return_type: String) -> Self {
        Self { name, parameters, return_type }
    }
}

/// An Ada parameter
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaParameter {
    pub name: String,
    pub param_type: String,
}

impl AdaParameter {
    pub fn new(name: String, param_type: String) -> Self {
        Self { name, param_type }
    }
}

/// An Ada type declaration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaTypeDeclaration {
    pub name: String,
    pub type_def: String,
}

impl AdaTypeDeclaration {
    pub fn new(name: String, type_def: String) -> Self {
        Self { name, type_def }
    }
}

/// An Ada variable declaration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaVariableDeclaration {
    pub name: String,
    pub var_type: String,
}

impl AdaVariableDeclaration {
    pub fn new(name: String, var_type: String) -> Self {
        Self { name, var_type }
    }
}

impl Default for AdaRoot {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
