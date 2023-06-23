#![doc = include_str!("readme.md")]
/// Root node of the Ada syntax tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaRoot {
    /// Items in this Ada file.
    pub items: Vec<AdaItem>,
}

impl AdaRoot {
    /// Creates a new Ada root node.
    pub fn new(items: Vec<AdaItem>) -> Self {
        Self { items }
    }

    /// Gets all top-level items in this Ada file.
    pub fn items(&self) -> &[AdaItem] {
        &self.items
    }
}

/// Item in an Ada file (package, procedure, function, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdaItem {
    /// Package declaration.
    Package(AdaPackage),
    /// Procedure declaration.
    Procedure(AdaProcedure),
    /// Function declaration.
    Function(AdaFunction),
    /// Type declaration.
    Type(AdaTypeDeclaration),
    /// Variable declaration.
    Variable(AdaVariableDeclaration),
}

/// Ada package declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaPackage {
    /// Package name.
    pub name: String,
}

impl AdaPackage {
    /// Creates a new package declaration.
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// Ada procedure declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaProcedure {
    /// Procedure name.
    pub name: String,
    /// Parameter list.
    pub parameters: Vec<AdaParameter>,
}

impl AdaProcedure {
    /// Creates a new procedure declaration.
    pub fn new(name: String, parameters: Vec<AdaParameter>) -> Self {
        Self { name, parameters }
    }
}

/// Ada function declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaFunction {
    /// Function name.
    pub name: String,
    /// Parameter list.
    pub parameters: Vec<AdaParameter>,
    /// Return type.
    pub return_type: String,
}

impl AdaFunction {
    /// Creates a new function declaration.
    pub fn new(name: String, parameters: Vec<AdaParameter>, return_type: String) -> Self {
        Self { name, parameters, return_type }
    }
}

/// Ada parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaParameter {
    /// Parameter name.
    pub name: String,
    /// Parameter type.
    pub param_type: String,
}

impl AdaParameter {
    /// Creates a new parameter.
    pub fn new(name: String, param_type: String) -> Self {
        Self { name, param_type }
    }
}

/// Ada type declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaTypeDeclaration {
    /// Type name.
    pub name: String,
    /// Type definition.
    pub type_def: String,
}

impl AdaTypeDeclaration {
    /// Creates a new type declaration.
    pub fn new(name: String, type_def: String) -> Self {
        Self { name, type_def }
    }
}

/// Ada variable declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaVariableDeclaration {
    /// Variable name.
    pub name: String,
    /// Variable type.
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
