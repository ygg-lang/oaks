#![doc = include_str!("readme.md")]
/// Root node of the V AST.
#[derive(Clone, Debug)]
pub struct VRoot {
    /// The name of the module.
    pub module_name: String,
    /// The list of imported modules.
    pub imports: Vec<String>,
    /// The list of items in the module.
    pub items: Vec<VItem>,
}

/// A top-level item in a V module.
#[derive(Clone, Debug)]
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

/// A struct definition in V.
#[derive(Clone, Debug)]
pub struct VStruct {
    /// The name of the struct.
    pub name: String,
    /// Whether the struct is public.
    pub is_pub: bool,
    /// The list of fields in the struct.
    pub fields: Vec<VField>,
}

/// A field in a V struct.
#[derive(Clone, Debug)]
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

/// A function definition in V.
#[derive(Clone, Debug)]
pub struct VFunction {
    /// The name of the function.
    pub name: String,
    /// Whether the function is public.
    pub is_pub: bool,
    /// The receiver of the function (if it's a method).
    pub receiver: Option<VReceiver>,
    /// The list of parameters for the function.
    pub params: Vec<VParam>,
    /// The return type of the function.
    pub return_type: Option<String>,
    /// The body of the function (currently represented as a list of strings).
    pub body: Vec<String>,
}

/// A receiver for a V method.
#[derive(Clone, Debug)]
pub struct VReceiver {
    /// The name of the receiver variable.
    pub name: String,
    /// The type of the receiver.
    pub receiver_type: String,
    /// Whether the receiver is mutable.
    pub is_mut: bool,
}

/// A parameter in a V function.
#[derive(Clone, Debug)]
pub struct VParam {
    /// The name of the parameter.
    pub name: String,
    /// The type of the parameter.
    pub param_type: String,
    /// Whether the parameter is mutable.
    pub is_mut: bool,
}

/// An enum definition in V.
#[derive(Clone, Debug)]
pub struct VEnum {
    /// The name of the enum.
    pub name: String,
    /// Whether the enum is public.
    pub is_pub: bool,
    /// The list of variants in the enum.
    pub variants: Vec<String>,
}

/// A constant definition in V.
#[derive(Clone, Debug)]
pub struct VConst {
    /// The name of the constant.
    pub name: String,
    /// Whether the constant is public.
    pub is_pub: bool,
    /// The value of the constant.
    pub value: String,
}
