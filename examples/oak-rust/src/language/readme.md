# Rust 语言定义模块

这个模块定义了 Rust 语言的核心配置和类型绑定，是整个解析器框架的中心组件。

## 主要功能

### 语言配置

`RustLanguage` 结构体包含了 Rust 语言的配置选项：

- **`allow_unsafe`**: 是否允许使用 `unsafe` 代码块和函数
- **`allow_async`**: 是否允许使用 `async`/`await` 语法
- **`experimental_features`**: 是否启用实验性功能

### 类型绑定

通过实现 `Language` trait，该模块将以下类型绑定在一起：

- **`TokenType`**: `RustTokenType` - 词法分析器生成的 token 类型
- **`ElementType`**: `RustElementType` - 解析器生成的无类型红绿树元素类型
- **`TypedRoot`**: `RustRoot` - 构建器生成的强类型 AST 根节点

## 使用示例

```rust,ignore
use oak_rust::language::RustLanguage;
use oak_core::Language;

// 创建默认的 Rust 语言配置
let language = RustLanguage::default();

// 创建自定义配置
let custom_language = RustLanguage {
    allow_unsafe: false,
    allow_async: true,
    experimental_features: true,
};

// 语言配置可以用于控制解析器的行为
println!("Unsafe allowed: {}", language.allow_unsafe);
```

## 设计原则

1. **类型安全**: 通过 trait 绑定确保各组件之间的类型一致性
2. **可配置性**: 提供灵活的语言特性开关
3. **扩展性**: 易于添加新的语言配置选项
4. **兼容性**: 支持不同版本的 Rust 语言特性