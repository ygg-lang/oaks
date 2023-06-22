# Oak Rust Parser

[![Crates.io](https://img.shields.io/crates/v/oak-rust.svg)](https://crates.io/crates/oak-rust)
[![Documentation](https://docs.rs/oak-rust/badge.svg)](https://docs.rs/oak-rust)

基于 Oak 框架构建的高性能增量 Rust 解析器，提供完整的 Rust 语法分析、代码格式化和语法高亮功能。

## 🎯 概述

Oak Rust 是一个专为 Rust 语言设计的强大解析器，支持完整的 Rust 语法，包括现代语言特性。基于 oak-core 的坚实基础，它提供了高级便利性和详细的 AST 生成功能，适用于静态分析、代码生成、格式化和语法高亮。

## ✨ 主要特性

- **完整的 Rust 语法支持**: 支持所有 Rust 语言特性，包括现代规范
- **完整的 AST 生成**: 生成全面的抽象语法树
- **词法分析器**: 内置标记化功能，提供准确的位置信息
- **语法高亮**: 支持关键字、字符串、数字、注释、宏等的高亮显示
- **代码格式化**: 提供符合官方风格指南的代码格式化功能
- **错误恢复**: 优雅处理语法错误，提供详细的诊断信息
- **增量解析**: 基于 Oak 框架的增量解析能力，提供高效的代码分析

## 🚀 快速开始

### 基本解析示例

```rust
use oak_rust::{RustLanguage, RustParser};
use oak_core::language::Language;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let language = RustLanguage::new();
    let parser = RustParser::new();
    
    let source = r#"
        fn main() {
            let x = 42;
            println!("Hello, Rust! x = {}", x);
        }
    "#;
    
    let result = language.parse(source);
    match result {
        Ok(ast) => println!("解析成功: {:?}", ast),
        Err(errors) => println!("解析错误: {:?}", errors),
    }
    Ok(())
}
```

### 语法高亮示例

```rust
use oak_rust::RustHighlighter;
use oak_highlight::highlighter::Highlighter;

fn main() {
    let highlighter = RustHighlighter::new();
    let code = r#"
        fn fibonacci(n: u32) -> u32 {
            match n {
                0 => 0,
                1 => 1,
                _ => fibonacci(n - 1) + fibonacci(n - 2),
            }
        }
    "#;
    
    let highlights = highlighter.highlight(code);
    for (start, end, kind) in highlights {
        println!("高亮范围: {}..{}, 类型: {:?}", start, end, kind);
    }
}
```

### 代码格式化示例

```rust
use oak_rust::RustFormatter;

fn main() {
    let formatter = RustFormatter::new();
    let unformatted_code = "fn main(){let x=42;println!(\"x={}\",x);}";
    
    let formatted = formatter.format(unformatted_code);
    println!("格式化后的代码:\n{}", formatted);
}
```

## 📋 解析示例

### 函数解析
```rust
use oak_rust::{RustLanguage, RustParser};

let language = RustLanguage::new();
let source = r#"
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
"#;

let result = language.parse(source);
println!("Rust 函数解析成功");
```

### 结构体解析
```rust
use oak_rust::{RustLanguage, RustParser};

let language = RustLanguage::new();
let source = r#"
    #[derive(Debug, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }
    
    impl Point {
        fn new(x: f64, y: f64) -> Self {
            Point { x, y }
        }
        
        fn distance(&self, other: &Point) -> f64 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }
"#;

let result = language.parse(source);
println!("Rust 结构体解析成功");
```

### 枚举和模式匹配解析
```rust
use oak_rust::{RustLanguage, RustParser};

let language = RustLanguage::new();
let source = r#"
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    fn process_message(msg: Message) {
        match msg {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("写入: {}", text),
            Message::ChangeColor(r, g, b) => println!("颜色: ({}, {}, {})", r, g, b),
        }
    }
"#;

let result = language.parse(source);
println!("Rust 枚举和模式匹配解析成功");
```

## 🔧 高级特性

### 词法分析
```rust
use oak_rust::RustLexer;

let lexer = RustLexer::new();
let source = "let x = 42; // 这是一个注释";
let tokens = lexer.tokenize(source);

for token in tokens {
    println!("Token: {:?}", token);
}
```

### 错误处理
```rust
use oak_rust::{RustLanguage, RustParser};

let language = RustLanguage::new();
let source = r#"
    fn main() {
        println!("Hello, Rust!")
    // 缺少闭合大括号
"#;

let result = language.parse(source);
if let Err(errors) = result {
    for error in errors {
        println!("解析错误: {:?}", error);
    }
}
```

### 自定义格式化配置
```rust
use oak_rust::RustFormatter;

let formatter = RustFormatter::new()
    .with_indent_size(2)
    .with_max_line_length(100)
    .with_tabs(false);

let code = "fn main(){let x=42;}";
let formatted = formatter.format(code);
println!("自定义格式化结果:\n{}", formatted);
```

## 🏗️ AST 结构

解析器生成全面的 AST，包含以下主要结构：

- **SourceFile**: Rust 源文件的根容器
- **Function**: Rust 函数和方法定义
- **Struct**: Rust 结构体定义
- **Enum**: Rust 枚举定义
- **Trait**: Rust trait 定义
- **Impl**: 实现块（impl 块）
- **Module**: 模块定义
- **Use**: 使用声明
- **Static/Const**: 静态变量和常量
- **TypeAlias**: 类型别名
- **Macro**: 宏定义
- **Statement**: 各种语句类型
- **Expression**: 各种表达式类型
- **Pattern**: 模式匹配结构
- **Type**: 类型表示

## 🎨 语法高亮特性

### 支持的高亮元素

- **关键字**: 所有 Rust 关键字（严格、保留、弱关键字等）
- **字符串字面量**: 普通字符串、原始字符串、字节字符串等
- **数字字面量**: 整数、浮点数、各种进制表示
- **注释**: 行注释、块注释、文档注释
- **宏调用**: 函数式宏、属性宏等
- **标识符**: 变量名、函数名、类型名等

### 高亮模式

- **词法分析器模式**: 快速基础高亮
- **解析器模式**: 更准确的语义高亮

## 🎯 代码格式化特性

### 格式化功能

- **缩进管理**: 支持空格和制表符缩进
- **行长度控制**: 可配置最大行长度
- **代码结构**: 自动格式化函数、结构体、枚举等
- **表达式格式化**: 优化表达式和语句的布局
- **注释保持**: 保留原有注释的位置和格式

### 配置选项

- `indent_size`: 缩进大小
- `use_tabs`: 是否使用制表符
- `max_line_length`: 最大行长度

## 📊 性能特性

- **流式解析**: 无需将大型 Rust 文件完全加载到内存
- **增量解析**: 仅重新解析更改的部分
- **内存效率**: 智能的 AST 节点分配
- **快速恢复**: 快速错误恢复，更好的 IDE 集成
- **并发安全**: 线程安全的设计，支持并发使用

## 🔗 集成应用

Oak Rust 可以无缝集成到以下场景：

- **静态分析**: 代码质量和安全性分析
- **代码生成**: 从 Rust AST 生成代码
- **IDE 支持**: 语言服务器协议兼容性
- **重构工具**: 自动化代码重构
- **文档生成**: 从 Rust 代码生成文档
- **语法高亮**: 编辑器和 IDE 的语法高亮
- **代码格式化**: 自动代码格式化工具
- **Linting**: 代码风格检查和建议

## 📚 模块结构

- [`ast`] - 抽象语法树定义，包含所有 Rust 语法结构
- [`RustLanguage`] - Rust 语言定义和配置
- [`RustLexer`] - Rust 词法分析器
- [`RustParser`] - Rust 语法分析器
- [`RustBuilder`] - AST 构建器，将解析树转换为 AST
- [`RustFormatter`] - Rust 代码格式化器
- [`RustHighlighter`] - Rust 语法高亮器

## 📚 示例

查看 [examples](examples/) 目录获取全面的示例：

- 完整的 Rust 程序解析
- 函数和结构体分析
- 代码转换和格式化
- 语法高亮集成
- 开发工作流集成

## 🤝 贡献

欢迎贡献！

请随时在 [项目仓库](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-rust) 提交 pull request 或提出 [issues](https://github.com/ygg-lang/oaks/issues)。

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](../../License.md) 文件了解详情。