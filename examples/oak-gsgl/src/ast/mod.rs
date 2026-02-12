#![doc = include_str!("readme.md")]
/// Root node of the GSGL AST.
pub struct GsglRoot {
    /// The functions in the shader.
    pub functions: Vec<GsglFunction>,
    /// The structs in the shader.
    pub structs: Vec<GsglStruct>,
}

/// A function in GSGL.
pub struct GsglFunction {
    /// The name of the function.
    pub name: String,
    /// The return type.
    pub return_type: GsglType,
    /// The parameters.
    pub params: Vec<GsglParam>,
}

/// A parameter in a GSGL function.
pub struct GsglParam {
    /// The name of the parameter.
    pub name: String,
    /// The type of the parameter.
    pub ty: GsglType,
}

/// A struct in GSGL.
pub struct GsglStruct {
    /// The name of the struct.
    pub name: String,
    /// The members of the struct.
    pub members: Vec<GsglStructMember>,
}

/// A member of a GSGL struct.
pub struct GsglStructMember {
    /// The name of the member.
    pub name: String,
    /// The type of the member.
    pub ty: GsglType,
}

/// A type in GSGL.
pub struct GsglType {
    /// The name of the type.
    pub name: String,
}
