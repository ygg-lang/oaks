# Rust AST 构建器

这个模块提供了将解析得到的语法树转换为强类型抽象语法树 (AST) 的功能，专门用于 Rust 编程语言。

## 概述

构建器模块将解析器产生的通用红绿树结构转换为表示 Rust 源代码的强类型 AST。这种转换提供了：

- **类型安全**: 每个 AST 节点都有对应 Rust 语言构造的特定类型
- **错误处理**: AST 构建过程中的全面错误报告
- **增量构建**: 支持增量解析和 AST 构建

## 核心组件

### 构建器实现

为 `RustBuilder` 实现了 `Builder<RustLanguage>` trait，提供了 AST 构建的主要入口点：

```rust,ignore
impl<'config> Builder<RustLanguage> for RustBuilder<'config> {
    fn build_incremental(
        &self,
        text: impl Source,
        changed: usize,
        cache: IncrementalCache<RustLanguage>,
    ) -> OakDiagnostics<RustRoot>;
}
```

### AST 构建方法

构建器提供了用于构建不同类型 AST 节点的专门方法：

- `build_root()` - 构建包含所有顶级项目的根 AST 节点
- `build_function()` - 构建函数定义
- `build_struct()` - 构建结构体定义
- `build_enum()` - 构建枚举定义
- `build_trait()` - 构建 trait 定义
- `build_impl()` - 构建 impl 块
- `build_module()` - 构建模块定义
- `build_expr()` - 构建各种表达式类型
- `build_stmt()` - 构建语句节点

### 错误处理

构建器提供了带有详细源位置信息的全面错误处理：

- 带有精确位置报告的语法错误
- 缺失必需元素检测
- 意外 token/节点验证

## 使用方法

构建器通常通过 `RustBuilder` 在解析 Rust 源代码时使用：

```rust,ignore
use oak_rust::{RustBuilder, RustLanguage};
use oak_core::Builder;

let language = RustLanguage::default();
let builder = RustBuilder::new(&language);
let source = "fn main() { let x = 42; println!(\"Hello, world!\"); }";
let result = builder.build_incremental(source, 0, Default::default());

match result.result {
    Ok(ast) => {
        // 使用强类型 AST
        println!("解析了 {} 个项目", ast.items.len());
    }
    Err(error) => {
        eprintln!("解析错误: {}", error);
    }
}
```

## Rust 语言特性

构建器支持所有主要的 Rust 语言构造：

- **函数**: 支持泛型、参数、返回类型和函数体
- **结构体**: 包括字段定义和泛型参数
- **枚举**: 支持变体和关联数据
- **Trait**: 接口定义和方法签名
- **Impl 块**: 类型实现和 trait 实现
- **模块**: 代码组织单元
- **表达式**: 标识符、字面量、二元/一元操作、调用、字段访问、索引等
- **语句**: 变量绑定 (`let`)、表达式语句等
- **类型系统**: 基本类型、引用、泛型、生命周期等

## 架构

构建器遵循红绿树架构：

1. **绿树**: 来自解析器的不可变结构表示
2. **红树**: 提供导航和跨度信息
3. **AST**: 强类型、特定于语言的表示

这种架构支持高效的增量解析，并为 IDE 场景提供了出色的性能特征。

## 设计原则

1. **完整性**: 支持完整的 Rust 语法
2. **准确性**: 精确的错误位置和诊断信息
3. **性能**: 高效的增量构建和内存使用
4. **可扩展性**: 易于添加新的语言特性支持