# Oak Rust - Rust 语言解析器

Oak Rust 是一个基于 Oak 框架构建的高性能 Rust 语言解析器，提供完整的语法分析、AST 生成、语法高亮和代码格式化功能。

## 📋 概述

Oak Rust 解析器专注于以下核心特性：
- **高性能**: 增量解析，内存开销最小
- **准确性**: 完全支持 Rust 复杂的语法规则
- **可扩展性**: 模块化设计，易于定制
- **错误恢复**: 优雅处理语法错误
- **语法高亮**: 内置高亮器，支持关键字、字符串、注释等
- **代码格式化**: 遵循 Rust 官方代码风格指南

## 🏗️ 架构

### 核心组件

- **[`RustParser`](parser/struct.RustParser.html)**: 主解析器实现，使用 Pratt 解析器处理操作符优先级
- **[`RustLexer`](lexer/struct.RustLexer.html)**: 词法分析引擎，提供精确的位置跟踪
- **[`RustLanguage`](language/struct.RustLanguage.html)**: 语言配置和语法规则
- **[`RustBuilder`](builder/struct.RustBuilder.html)**: AST 构建器，将解析树转换为强类型 AST
- **[`RustRoot`](ast/struct.RustRoot.html)**: AST 根节点，包含所有解析的项目

### 可选组件

- **[`RustFormatter`](formatter/struct.RustFormatter.html)**: Rust 代码格式化器（需要 `oak-pretty-print` 特性）
- **[`RustHighlighter`](highlighter/struct.RustHighlighter.html)**: Rust 语法高亮器（需要 `oak-highlight` 特性）

### AST 结构

解析器生成强类型 AST，包含以下主要结构：

- **[`Item`](ast/enum.Item.html)**: 顶级项目（函数、结构体、枚举、trait、impl 等）
- **[`Function`](ast/struct.Function.html)**: 函数定义，包含参数和函数体
- **[`Statement`](ast/enum.Statement.html)**: 各种语句类型（let 绑定、表达式语句等）
- **[`Expr`](ast/enum.Expr.html)**: 表达式类型，涵盖所有 Rust 表达式
- **[`Type`](ast/enum.Type.html)**: 类型表示（基本类型、引用、泛型等）
- **[`Pattern`](ast/enum.Pattern.html)**: 模式匹配（标识符、结构体、元组等）
- **[`Identifier`](ast/struct.Identifier.html)**: 命名标识符，包含位置信息

## 🔧 使用示例

### 基本解析
```rust,ignore
use oak_rust::{RustLanguage, RustParser};
use oak_core::language::Language;

let language = RustLanguage::new();
let parser = RustParser::new();

let source = r#"
fn main() {
    let x = 42;
    println!("Hello, world! x = {}", x);
}
"#;

let result = language.parse(source);
match result {
    Ok(ast) => println!("解析成功: {:?}", ast),
    Err(errors) => println!("解析错误: {:?}", errors),
}
```

### 处理 AST
```rust,ignore
use oak_rust::{RustLanguage, RustParser, ast::*};

let language = RustLanguage::new();
let parser = RustParser::new();
let source = "fn add(a: i32, b: i32) -> i32 { a + b }";

if let Ok(result) = language.parse(source) {
    if let Some(root) = result.root {
        println!("解析了 {} 个项目", root.items.len());
        
        // 遍历所有项目
        for item in &root.items {
            match item {
                Item::Function(func) => {
                    println!("函数: {}", func.name.name);
                    println!("参数数量: {}", func.params.len());
                }
                Item::Struct(s) => {
                    println!("结构体: {}", s.name.name);
                }
                _ => println!("其他项目类型"),
            }
        }
    }
}
```

### 语法高亮
```rust,ignore
use oak_rust::RustHighlighter;
use oak_highlight::highlighter::Highlighter;

let highlighter = RustHighlighter::new();
let source = r#"
fn main() {
    let x = 42; // 这是一个注释
    println!("Hello, {}", x);
}
"#;

let highlights = highlighter.highlight(source);
for (start, end, kind) in highlights {
    println!("高亮范围: {}..{}, 类型: {:?}", start, end, kind);
}
```

### 代码格式化
```rust
use oak_rust::RustFormatter;

let formatter = RustFormatter::new();
let unformatted = "fn main(){let x=42;println!(\"x={}\",x);}";
let formatted = formatter.format(unformatted);

println!("格式化前:\n{}", unformatted);
println!("格式化后:\n{}", formatted);
```

### 自定义配置
```rust
use oak_rust::{RustFormatter, RustHighlighter};

// 自定义格式化器配置
let formatter = RustFormatter::with_config("  ".to_string(), 80); // 2空格缩进，80字符行宽

// 启用基于解析器的高亮
let highlighter = RustHighlighter::with_parser();
```

## 📊 性能特征

- **时间复杂度**: O(n)，其中 n 是 token 数量
- **空间复杂度**: O(n)，用于 AST 存储
- **增量解析**: O(delta)，仅重新解析变更部分
- **内存使用**: 针对大文件处理进行优化

## 🔗 集成点

此解析器可与以下工具集成：
- **IDE 支持**: 语言服务器实现
- **静态分析**: 代码质量和安全工具
- **代码生成**: 模板和宏系统
- **重构工具**: 自动化代码转换
- **语法高亮**: 编辑器和 IDE 的语法着色
- **代码格式化**: 自动代码风格统一

## 📝 实现细节

### 操作符优先级

解析器使用 Pratt 解析器，具有精心定义的操作符优先级：

1. **赋值操作符** (最低优先级，右结合)
2. **逻辑操作符** (`||`, `&&`)
3. **位操作符** (`|`, `^`, `&`)
4. **比较操作符** (`==`, `!=`, `<`, `>` 等)
5. **移位操作符** (`<<`, `>>`)
6. **算术操作符** (`+`, `-`, `*`, `/`, `%`)
7. **一元操作符** (最高优先级)

### 错误恢复

解析器实现了复杂的错误恢复策略：
- **同步**: 在语句和表达式边界恢复
- **Token 跳过**: 智能跳过意外的 token
- **上下文感知**: 使用语法上下文进行更好的恢复

### 语法高亮特性

- **关键字高亮**: 支持所有 Rust 关键字
- **字符串处理**: 支持普通字符串、字符字面量和原始字符串
- **数字格式**: 支持十进制、十六进制、八进制、二进制数字
- **注释支持**: 单行注释 (`//`) 和多行注释 (`/* */`)
- **宏识别**: 识别宏调用 (`macro_name!`)

### 代码格式化特性

- **缩进管理**: 可配置的缩进字符串和级别
- **行长度控制**: 可配置的最大行长度
- **语法结构格式化**: 函数、结构体、表达式等的格式化
- **空格和换行**: 遵循 Rust 官方风格指南

## 🧪 测试

全面的测试套件包括：
- **单元测试**: 各个组件的独立测试
- **集成测试**: 完整解析工作流程测试
- **回归测试**: 防止已知问题重现
- **性能测试**: 确保解析速度要求
- **语法高亮测试**: 验证高亮准确性
- **格式化测试**: 验证代码格式化正确性

## 📚 相关文档

- **[Parser 模块](parser/index.html)**: 主解析器实现
- **[Lexer 模块](lexer/index.html)**: 词法分析引擎
- **[AST 模块](ast/index.html)**: 抽象语法树定义
- **[Language 模块](language/index.html)**: 语言配置
- **[Builder 模块](builder/index.html)**: AST 构建器
- **[Formatter 模块](formatter/index.html)**: 代码格式化器
- **[Highlighter 模块](highlighter/index.html)**: 语法高亮器

## 🚀 特性标志

- `oak-pretty-print`: 启用代码格式化功能
- `oak-highlight`: 启用语法高亮功能

## 📄 许可证

本项目遵循与 Oak 框架相同的许可证。