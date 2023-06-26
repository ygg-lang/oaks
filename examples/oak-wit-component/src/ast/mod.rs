#![doc = include_str!("readme.md")]
/// WIT root node
#[derive(Clone, Debug)]
pub struct WitRoot {
    /// The list of top-level items in the WIT document.
    pub items: Vec<WitItem>,
}

/// WIT item
#[derive(Clone, Debug)]
pub enum WitItem {
    /// A package declaration.
    Package(WitPackage),
    /// A world definition.
    World(WitWorld),
    /// An interface definition.
    Interface(WitInterface),
}

/// WIT package
#[derive(Clone, Debug)]
pub struct WitPackage {
    /// The name of the package.
    pub name: String,
}

/// WIT world
#[derive(Clone, Debug)]
pub struct WitWorld {
    /// The name of the world.
    pub name: String,
    /// The items defined in the world.
    pub items: Vec<WitWorldItem>,
}

/// WIT world item
#[derive(Clone, Debug)]
pub enum WitWorldItem {
    /// An import declaration.
    Import(WitImport),
    /// An export declaration.
    Export(WitExport),
    /// An include declaration.
    Include(WitInclude),
}

/// WIT interface
#[derive(Clone, Debug)]
pub struct WitInterface {
    /// The name of the interface.
    pub name: String,
    /// The items defined in the interface.
    pub items: Vec<WitInterfaceItem>,
}

/// WIT interface item
#[derive(Clone, Debug)]
pub enum WitInterfaceItem {
    Type(WitType),
    Func(WitFunc),
}

/// WIT function
#[derive(Clone, Debug)]
pub struct WitFunc {
    /// The name of the function.
    pub name: String,
    /// The parameters of the function.
    pub params: Vec<WitParam>,
    /// The optional return type of the function.
    pub result: Option<WitTypeKind>,
}

/// WIT parameter
#[derive(Clone, Debug)]
pub struct WitParam {
    /// The name of the parameter.
    pub name: String,
    /// The type of the parameter.
    pub ty: WitTypeKind,
}

/// WIT type
#[derive(Clone, Debug)]
pub struct WitType {
    /// The name of the type.
    pub name: String,
    /// The kind of the type.
    pub kind: WitTypeKind,
}

/// WIT type kind
#[derive(Clone, Debug)]
pub enum WitTypeKind {
    /// Boolean type.
    Bool,
    /// 32-bit unsigned integer type.
    U32,
    /// String type.
    String,
}

/// WIT import
#[derive(Clone, Debug)]
pub struct WitImport {
    /// The name of the imported item.
    pub name: String,
}

/// WIT export
#[derive(Clone, Debug)]
pub struct WitExport {
    /// The name of the exported item.
    pub name: String,
}

/// WIT include
#[derive(Clone, Debug)]
pub struct WitInclude {
    /// The name of the included item.
    pub name: String,
}
