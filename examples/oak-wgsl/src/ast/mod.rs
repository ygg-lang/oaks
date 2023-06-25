#![doc = include_str!("readme.md")]
/// WGSL root node.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct WgslRoot {
    /// List of items.
    pub items: Vec<WgslItem>,
}

/// A WGSL item.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum WgslItem {
    /// Function definition.
    Function(WgslFunction),
    /// Variable definition.
    Variable(WgslVariable),
    /// Struct definition.
    Struct(WgslStruct),
}

/// A WGSL function.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct WgslFunction {
    /// Function name.
    pub name: String,
    /// Parameter list.
    pub params: Vec<WgslParam>,
    /// Return type.
    pub return_type: Option<WgslType>,
}

/// A WGSL parameter.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct WgslParam {
    /// Parameter name.
    pub name: String,
    /// Parameter type.
    pub ty: WgslType,
}

/// A WGSL variable.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct WgslVariable {
    /// Variable name.
    pub name: String,
    /// Variable type.
    pub ty: Option<WgslType>,
    /// Variable value.
    pub value: Option<WgslExpression>,
}

/// A WGSL struct.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct WgslStruct {
    /// Struct name.
    pub name: String,
    /// Member list.
    pub members: Vec<WgslStructMember>,
}

/// A WGSL struct member.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct WgslStructMember {
    /// Member name.
    pub name: String,
    /// Member type.
    pub ty: WgslType,
}

/// A WGSL type.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct WgslType {
    /// Type name.
    pub name: String,
}

/// A WGSL expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct WgslExpression {
    /// Expression text.
    pub text: String,
}
