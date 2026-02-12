#![doc = include_str!("readme.md")]
/// WIT root node
#[derive(Clone, Debug)]
pub struct WitRoot {
    pub items: Vec<WitItem>,
}

/// WIT item
#[derive(Clone, Debug)]
pub enum WitItem {
    Package(WitPackage),
    World(WitWorld),
    Interface(WitInterface),
}

/// WIT package
#[derive(Clone, Debug)]
pub struct WitPackage {
    pub name: String,
}

/// WIT world
#[derive(Clone, Debug)]
pub struct WitWorld {
    pub name: String,
    pub items: Vec<WitWorldItem>,
}

/// WIT world item
#[derive(Clone, Debug)]
pub enum WitWorldItem {
    Import(WitImport),
    Export(WitExport),
    Include(WitInclude),
}

/// WIT interface
#[derive(Clone, Debug)]
pub struct WitInterface {
    pub name: String,
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
    pub name: String,
    pub params: Vec<WitParam>,
    pub result: Option<WitTypeKind>,
}

/// WIT parameter
#[derive(Clone, Debug)]
pub struct WitParam {
    pub name: String,
    pub ty: WitTypeKind,
}

/// WIT type
#[derive(Clone, Debug)]
pub struct WitType {
    pub name: String,
    pub kind: WitTypeKind,
}

/// WIT type kind
#[derive(Clone, Debug)]
pub enum WitTypeKind {
    Bool,
    U32,
    String,
    // ...
}

/// WIT import
#[derive(Clone, Debug)]
pub struct WitImport {
    pub name: String,
}

/// WIT export
#[derive(Clone, Debug)]
pub struct WitExport {
    pub name: String,
}

/// WIT include
#[derive(Clone, Debug)]
pub struct WitInclude {
    pub name: String,
}
