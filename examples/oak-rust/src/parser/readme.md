# Rust 语法分析器

Rust 语法分析器模块提供了完整的 Rust 语言语法分析功能，能够将词法分析器产生的 token 流转换为抽象语法树（AST）。

## 概述

该模块实现了基于 Oak 框架的增量解析器，支持 Rust 语言的完整语法结构。解析器采用红绿树架构，提供高效的增量解析和错误恢复能力。

## 核心组件

### RustElementType
定义了 Rust 语言的所有 AST 节点类型：
- **顶级项目**: `SourceFile`, `Module`, `Function`, `Struct`, `Enum`, `Trait`, `Impl`
- **声明**: `Use`, `Static`, `Const`, `TypeAlias`, `Macro`, `ExternCrate`, `ExternBlock`
- **语法结构**: `Block`, `ParameterList`, `StructBody`, `EnumBody`, `TraitBody`, `ImplBody`
- **表达式**: `Expression`, `Statement`, `Pattern`, `Type`

### RustParser
实现了 `Parser<RustLanguage>` trait，提供：
- **增量解析**: 支持基于缓存的增量解析
- **错误恢复**: 遇到语法错误时能够继续解析
- **完整语法支持**: 覆盖 Rust 语言的所有语法结构

## 使用方法

```rust,ignore
use oak_rust::{RustParser, RustLanguage};
use oak_core::{IncrementalCache, source::StringSource};

let parser = RustParser::new();
let source = StringSource::new("fn main() { println!(\"Hello, world!\"); }");
let cache = IncrementalCache::<RustLanguage>::new();

let result = parser.parse_incremental(source, cache);
match result {
    Ok(ast) => {
        // 处理解析成功的 AST
        println!("解析成功: {:?}", ast);
    }
    Err(diagnostics) => {
        // 处理解析错误
        for error in diagnostics.errors {
            println!("语法错误: {}", error);
        }
    }
}
```

## 支持的 Rust 语言特性

### 顶级项目
- **函数定义**: `fn` 关键字定义的函数
- **结构体**: `struct` 定义的结构体类型
- **枚举**: `enum` 定义的枚举类型
- **特征**: `trait` 定义的特征
- **实现块**: `impl` 实现块
- **模块**: `mod` 模块定义
- **使用声明**: `use` 导入声明
- **常量**: `const` 常量定义
- **静态变量**: `static` 静态变量定义
- **类型别名**: `type` 类型别名定义

### 可见性修饰符
- **公共项目**: `pub` 修饰的公共项目
- **受限可见性**: `pub(crate)`, `pub(super)` 等

### 语法结构
- **参数列表**: 函数参数的解析
- **代码块**: `{}` 包围的代码块
- **结构体体**: 结构体字段定义
- **枚举体**: 枚举变体定义
- **特征体**: 特征方法定义
- **实现体**: 实现方法定义

## 错误处理

解析器提供了完善的错误处理机制：

### 语法错误检测
- **缺失标识符**: 检测函数名、结构体名等缺失
- **不匹配的括号**: 检测括号、大括号的匹配
- **意外的 token**: 检测不符合语法规则的 token

### 错误恢复
- **跳过错误 token**: 遇到错误时跳过并继续解析
- **同步点恢复**: 在特定的语法点进行错误恢复
- **部分 AST 构建**: 即使有错误也能构建部分 AST

## 性能特性

### 增量解析
- **缓存机制**: 利用之前的解析结果
- **最小重解析**: 只重新解析修改的部分
- **内存效率**: 共享不变的 AST 节点

### 红绿树架构
- **不可变节点**: 绿色节点提供结构共享
- **可变视图**: 红色节点提供可变的 AST 视图
- **高效更新**: 支持高效的 AST 修改操作

## 设计原则

1. **完整性**: 支持 Rust 语言的完整语法
2. **容错性**: 能够处理不完整或有错误的代码
3. **性能**: 提供高效的解析和增量更新
4. **可扩展性**: 易于添加新的语法特性
5. **一致性**: 与 Oak 框架的设计保持一致