/// WGSL 根节点
#[derive(Clone, Debug)]
pub struct WgslRoot {
    pub items: Vec<WgslItem>,
}

/// WGSL 项目
#[derive(Clone, Debug)]
pub enum WgslItem {
    Function(WgslFunction),
    Variable(WgslVariable),
    Struct(WgslStruct),
}

/// WGSL 函数
#[derive(Clone, Debug)]
pub struct WgslFunction {
    pub name: String,
    pub params: Vec<WgslParam>,
    pub return_type: Option<WgslType>,
}

/// WGSL 参数
#[derive(Clone, Debug)]
pub struct WgslParam {
    pub name: String,
    pub ty: WgslType,
}

/// WGSL 变量
#[derive(Clone, Debug)]
pub struct WgslVariable {
    pub name: String,
    pub ty: Option<WgslType>,
    pub value: Option<WgslExpression>,
}

/// WGSL 结构体
#[derive(Clone, Debug)]
pub struct WgslStruct {
    pub name: String,
    pub members: Vec<WgslStructMember>,
}

/// WGSL 结构体成员
#[derive(Clone, Debug)]
pub struct WgslStructMember {
    pub name: String,
    pub ty: WgslType,
}

/// WGSL 类型
#[derive(Clone, Debug)]
pub struct WgslType {
    pub name: String,
}

/// WGSL 表达式 (简化)
#[derive(Clone, Debug)]
pub struct WgslExpression {
    pub text: String,
}
