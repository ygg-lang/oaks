#![doc = include_str!("readme.md")]

use serde::{Deserialize, Serialize};

/// Jasmin 根节点
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JasminRoot {
    pub class: JasminClass,
}

/// Jasmin 类声明的 AST 节点
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JasminClass {
    /// 访问修饰符（public, private 等）
    pub modifiers: Vec<String>,
    /// 类名
    pub name: String,
    /// 版本信息（如 65:0）
    pub version: Option<String>,
    /// 方法列表
    pub methods: Vec<JasminMethod>,
    /// 字段列表
    pub fields: Vec<JasminField>,
    /// 源文件信息
    pub source_file: Option<String>,
}

/// Jasmin 方法声明AST 节点
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JasminMethod {
    /// 访问修饰符（public, static 等）
    pub modifiers: Vec<String>,
    /// 方法名和类型描述符（"main":"([Ljava/lang/String;)V"）
    pub name_and_descriptor: String,
    /// 栈大小
    pub stack_size: Option<u32>,
    /// 局部变量数量
    pub locals_count: Option<u32>,
    /// 指令列表
    pub instructions: Vec<JasminInstruction>,
}

/// Jasmin 字段声明AST 节点
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JasminField {
    /// 访问修饰符（public, static 等）
    pub modifiers: Vec<String>,
    /// 字段名和类型描述符（"value":"I"）
    pub name_and_descriptor: String,
}

/// Jasmin 指令AST 节点
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum JasminInstruction {
    /// 简单指令（aload_0, return）
    Simple(String),
    /// 带参数的指令（如 ldc "Hello"）
    WithArgument { instruction: String, argument: String },
    /// 方法调用指令（如 invokespecial Method java/lang/Object."<init>":"()V"）
    MethodCall { instruction: String, method_ref: String },
    /// 字段访问指令（如 getstatic Field java/lang/System.out:"Ljava/io/PrintStream;"）
    FieldAccess { instruction: String, field_ref: String },
}
