#![doc = include_str!("readme.md")]

/// V 根节点
#[derive(Clone, Debug)]
pub struct VRoot {
    pub module_name: String,
    pub imports: Vec<String>,
    pub items: Vec<VItem>,
}

/// V 项目
#[derive(Clone, Debug)]
pub enum VItem {
    Struct(VStruct),
    Function(VFunction),
    Enum(VEnum),
    Const(VConst),
}

/// V 结构体
#[derive(Clone, Debug)]
pub struct VStruct {
    pub name: String,
    pub is_pub: bool,
    pub fields: Vec<VField>,
}

/// V 字段
#[derive(Clone, Debug)]
pub struct VField {
    pub name: String,
    pub field_type: String,
    pub is_pub: bool,
    pub is_mut: bool,
}

/// V 函数
#[derive(Clone, Debug)]
pub struct VFunction {
    pub name: String,
    pub is_pub: bool,
    pub receiver: Option<VReceiver>,
    pub params: Vec<VParam>,
    pub return_type: Option<String>,
    pub body: Vec<String>, // 暂时用字符串表示
}

/// V 接收者 (Method)
#[derive(Clone, Debug)]
pub struct VReceiver {
    pub name: String,
    pub receiver_type: String,
    pub is_mut: bool,
}

/// V 参数
#[derive(Clone, Debug)]
pub struct VParam {
    pub name: String,
    pub param_type: String,
    pub is_mut: bool,
}

/// V 枚举
#[derive(Clone, Debug)]
pub struct VEnum {
    pub name: String,
    pub is_pub: bool,
    pub variants: Vec<String>,
}

/// V 常量
#[derive(Clone, Debug)]
pub struct VConst {
    pub name: String,
    pub is_pub: bool,
    pub value: String,
}
