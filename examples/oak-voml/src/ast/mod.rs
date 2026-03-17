#![doc = include_str!("readme.md")]

/// Root node of the Voml AST.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VRoot {
    /// The name of the module.
    pub module_name: String,
    /// List of imported module names.
    pub imports: Vec<String>,
    /// List of top-level items in the module.
    pub items: Vec<VItem>,
}

/// A top-level item in a Voml module.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VItem {
    /// A struct definition.
    Struct(VStruct),
    /// A function definition.
    Function(VFunction),
    /// An enum definition.
    Enum(VEnum),
    /// A constant definition.
    Const(VConst),
}

/// A struct definition in Voml.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VStruct {
    /// The name of the struct.
    pub name: String,
    /// Whether the struct is public.
    pub is_pub: bool,
    /// List of fields in the struct.
    pub fields: Vec<VField>,
}

/// A field in a Voml struct.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VField {
    /// The name of the field.
    pub name: String,
    /// The type of the field.
    pub field_type: String,
    /// Whether the field is public.
    pub is_pub: bool,
    /// Whether the field is mutable.
    pub is_mut: bool,
}

/// A function definition in Voml.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VFunction {
    /// The name of the function.
    pub name: String,
    /// Whether the function is public.
    pub is_pub: bool,
    /// Optional receiver (for methods).
    pub receiver: Option<VReceiver>,
    /// List of function parameters.
    pub params: Vec<VParam>,
    /// Optional return type.
    pub return_type: Option<String>,
    /// Function body (currently represented as strings).
    pub body: Vec<String>,
}

/// A method receiver in Voml.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VReceiver {
    /// The name of the receiver (usually `self`).
    pub name: String,
    /// The type of the receiver.
    pub receiver_type: String,
    /// Whether the receiver is mutable.
    pub is_mut: bool,
}

/// A parameter in a Voml function.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VParam {
    /// The name of the parameter.
    pub name: String,
    /// The type of the parameter.
    pub param_type: String,
    /// Whether the parameter is mutable.
    pub is_mut: bool,
}

/// An enum definition in Voml.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VEnum {
    /// The name of the enum.
    pub name: String,
    /// Whether the enum is public.
    pub is_pub: bool,
    /// List of enum variant names.
    pub variants: Vec<String>,
}

/// A constant definition in Voml.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VConst {
    /// The name of the constant.
    pub name: String,
    /// Whether the constant is public.
    pub is_pub: bool,
    /// The value of the constant.
    pub value: String,
}
