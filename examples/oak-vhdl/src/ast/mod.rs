#![doc = include_str!("readme.md")]

/// VHDL root node
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VhdlRoot {
    /// The design units in the file
    pub units: Vec<DesignUnit>,
}

/// A design unit
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DesignUnit {
    /// Entity declaration
    Entity(EntityDeclaration),
    /// Architecture body
    Architecture(ArchitectureBody),
    /// Package declaration
    Package(PackageDeclaration),
}

/// An entity declaration
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntityDeclaration {
    /// The name of the entity
    pub name: String,
    /// The ports of the entity
    pub ports: Vec<PortDeclaration>,
}

/// A port declaration
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PortDeclaration {
    /// The name of the port
    pub name: String,
    /// The direction of the port
    pub direction: PortDirection,
    /// The data type of the port
    pub data_type: String,
}

/// Port direction
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PortDirection {
    /// Input port
    #[default]
    In,
    /// Output port
    Out,
    /// Bidirectional port
    Inout,
    /// Buffer port
    Buffer,
    /// Linkage port
    Linkage,
}

/// Architecture body
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArchitectureBody {
    /// The name of the architecture
    pub name: String,
    /// The name of the entity this architecture belongs to
    pub entity_name: String,
    /// The items in the architecture
    pub items: Vec<ArchitectureItem>,
}

/// An item in an architecture body
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ArchitectureItem {
    /// Signal declaration
    Signal(SignalDeclaration),
    /// Process statement
    Process(ProcessStatement),
    /// Component declaration
    Component(ComponentDeclaration),
}

/// A signal declaration
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignalDeclaration {
    /// The name of the signal
    pub name: String,
    /// The data type of the signal
    pub data_type: String,
}

/// A process statement
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProcessStatement {
    /// The label of the process
    pub label: Option<String>,
    /// The sensitivity list of the process
    pub sensitivity_list: Vec<String>,
    /// The body of the process
    pub body: String,
}

/// A component declaration
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComponentDeclaration {
    /// The name of the component
    pub name: String,
    /// The ports of the component
    pub ports: Vec<PortDeclaration>,
}

/// A package declaration
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PackageDeclaration {
    /// The name of the package
    pub name: String,
    /// The items in the package
    pub items: Vec<PackageItem>,
}

/// An item in a package declaration
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PackageItem {
    /// A function declaration
    Function(String),
    /// A type declaration
    Type(String),
}
