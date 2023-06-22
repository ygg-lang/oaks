#![doc = include_str!("readme.md")]

use core::range::Range;
use serde::{Deserialize, Serialize};

/// Vala 根节点
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValaRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub items: Vec<ValaItem>,
}

/// Vala 顶层项目
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ValaItem {
    Namespace(ValaNamespace),
    Class(ValaClass),
    Interface(ValaInterface),
    Method(ValaMethod),
}

/// Vala 命名空间
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValaNamespace {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub name: String,
    pub items: Vec<ValaItem>,
}

/// Vala 类
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValaClass {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub name: String,
    pub parent: Option<String>,
    pub interfaces: Vec<String>,
    pub members: Vec<ValaMember>,
}

/// Vala 接口
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValaInterface {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub name: String,
    pub members: Vec<ValaMember>,
}

/// Vala 成员
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ValaMember {
    Field(String),
    Property(String),
    Method(ValaMethod),
}

/// Vala 方法
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValaMethod {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub name: String,
    pub return_type: String,
    pub params: Vec<ValaParam>,
}

/// Vala 参数
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValaParam {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub name: String,
    pub param_type: String,
}
