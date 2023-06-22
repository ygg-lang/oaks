# Rust 语法高亮器

这个模块提供了 Rust 源代码的语法高亮功能，支持 Rust 语言的各种语法元素的高亮显示。

## 概述

`RustHighlighter` 是一个专门为 Rust 语言设计的语法高亮器，它实现了 `Highlighter` trait，能够识别和高亮 Rust 代码中的各种语法元素，包括关键字、字符串字面量、数字字面量、注释和宏调用等。

## 核心组件

### RustHighlighter

主要的语法高亮器结构体，提供以下功能：

- **基于词法分析器的高亮**：快速的基本语法高亮
- **基于解析器的高亮**：更准确的语义分析高亮
- **多种语法元素支持**：关键字、字符串、数字、注释、宏等

## 使用方法

### 基本用法

```rust
use oak_rust::highlighter::RustHighlighter;
use oak_highlight::highlighter::Highlighter;

// 创建高亮器实例
let highlighter = RustHighlighter::new();

// 对 Rust 代码进行语法高亮
let code = r#"
fn main() {
    let x = 42;
    println!("Hello, world! x = {}", x);
}
"#;

let highlights = highlighter.highlight(code);
```

### 使用解析器模式

```rust
// 创建启用解析器模式的高亮器
let highlighter = RustHighlighter::with_parser();
let highlights = highlighter.highlight(code);
```

## 支持的 Rust 语言特性

### 关键字高亮

支持所有 Rust 关键字的高亮，包括：

- **严格关键字**：`as`, `break`, `const`, `continue`, `crate`, `else`, `enum`, `extern`, `false`, `fn`, `for`, `if`, `impl`, `in`, `let`, `loop`, `match`, `mod`, `move`, `mut`, `pub`, `ref`, `return`, `self`, `Self`, `static`, `struct`, `super`, `trait`, `true`, `type`, `unsafe`, `use`, `where`, `while`
- **保留关键字**：`abstract`, `become`, `box`, `do`, `final`, `macro`, `override`, `priv`, `typeof`, `unsized`, `virtual`, `yield`
- **弱关键字**：`union`, `'static`
- **版本特定关键字**：`async`, `await`, `dyn`, `try`

### 字符串字面量高亮

支持多种字符串格式：

- **普通字符串**：`"hello"`
- **原始字符串**：`r"hello"`, `r#"hello"#`
- **字节字符串**：`b"hello"`
- **原始字节字符串**：`br"hello"`, `br#"hello"#`
- **字符字面量**：`'a'`, `'\n'`
- **字节字符**：`b'a'`

### 数字字面量高亮

支持各种数字格式：

- **整数**：`42`, `0x2A`, `0o52`, `0b101010`
- **浮点数**：`3.14`, `1e10`, `1.0f32`
- **带类型后缀**：`42u32`, `3.14f64`
- **带分隔符**：`1_000_000`

### 注释高亮

支持 Rust 的注释格式：

- **行注释**：`// 这是行注释`
- **块注释**：`/* 这是块注释 */`
- **嵌套块注释**：`/* 外层 /* 内层 */ 注释 */`
- **文档注释**：`/// 文档注释`, `//! 模块文档`

### 宏调用高亮

识别和高亮宏调用：

- **函数式宏**：`println!()`, `vec![]`
- **属性宏**：`#[derive(Debug)]`
- **自定义宏**：用户定义的宏调用

## 配置选项

### 高亮模式

- **词法分析器模式**（默认）：快速但基础的语法高亮
- **解析器模式**：更准确的语义分析，但性能稍慢

```rust
// 词法分析器模式
let highlighter = RustHighlighter::new();

// 解析器模式
let highlighter = RustHighlighter::with_parser();
```

## 性能特性

- **增量高亮**：支持对代码变更进行增量高亮
- **内存效率**：优化的内存使用，适合大型文件
- **并发安全**：线程安全的设计，支持并发使用

## 错误处理

高亮器采用容错设计：

- **语法错误容忍**：即使代码有语法错误也能提供基本高亮
- **部分高亮**：对于无法完全解析的代码提供部分高亮
- **优雅降级**：解析器模式失败时自动回退到词法分析器模式

## 设计原则

1. **准确性**：尽可能准确地识别 Rust 语法元素
2. **性能**：优化的算法确保快速的高亮处理
3. **可扩展性**：模块化设计便于添加新的高亮特性
4. **兼容性**：与 Rust 语言规范保持同步

## 扩展性

高亮器设计为可扩展的：

- **自定义高亮规则**：可以添加特定项目的高亮规则
- **插件支持**：支持通过插件扩展高亮功能
- **主题集成**：与各种编辑器主题系统集成