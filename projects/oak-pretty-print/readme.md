# oak-pretty-print

一个基于 `oak-core` 的通用代码格式化库，支持多种编程语言的代码格式化。

## 特性

- 🎯 **语言无关**: 基于 `oak-core` 的抽象语法树，不绑定任何具体语言
- 🔧 **可配置**: 丰富的格式化配置选项
- 📏 **规则系统**: 灵活的格式化规则系统，支持自定义规则
- 🚀 **高性能**: 基于 Rust 实现，无 std 环境支持
- 🔄 **可扩展**: 易于扩展和定制的架构

## 核心组件

### FormatConfig
格式化配置，包含缩进样式、行结束符、最大行长度等选项。

```rust
use oak_pretty_print::{FormatConfig, IndentStyle, LineEnding};

let config = FormatConfig::new()
    .with_indent_style(IndentStyle::Spaces(4))
    .with_line_ending(LineEnding::Unix)
    .with_max_line_length(100);
```

### Formatter
核心格式化器，负责将 AST 转换为格式化后的代码。

```rust
use oak_pretty_print::{Formatter, FormatConfig};

let config = FormatConfig::default();
let formatter = Formatter::new(config);
```

### FormatRule
格式化规则系统，支持自定义格式化逻辑。

```rust
use oak_pretty_print::{BasicFormatRule, FormatRule};

let rule = BasicFormatRule::new(
    "my_rule".to_string(),
    |node, context| {
        // 自定义格式化逻辑
        Ok(())
    },
    |node| true, // 适用条件
).with_priority(10);
```

## 使用示例

### 基础使用

```rust
use oak_pretty_print::{Formatter, FormatConfig};
use oak_core::AstNode;

// 创建格式化器
let config = FormatConfig::default();
let formatter = Formatter::new(config);

// 格式化 AST 节点
let result = formatter.format_ast(&ast_node)?;
println!("格式化后的代码: {}", result.content);
```

### 自定义配置

```rust
use oak_pretty_print::{FormatConfig, IndentStyle, LineEnding};

let config = FormatConfig::new()
    .with_indent_style(IndentStyle::Tabs)
    .with_line_ending(LineEnding::Windows)
    .with_max_line_length(80);
```

### 添加自定义规则

```rust
use oak_pretty_print::{Formatter, BasicFormatRule};

let mut formatter = Formatter::new(FormatConfig::default());

let custom_rule = BasicFormatRule::new(
    "custom_spacing".to_string(),
    |node, context| {
        // 在特定节点后添加空格
        context.write(" ");
        Ok(())
    },
    |node| {
        // 检查节点类型
        true
    },
);

formatter.add_rule(Box::new(custom_rule))?;
```

## 内置规则

库提供了一系列内置的格式化规则：

- **缩进规则**: 自动处理代码缩进
- **空行规则**: 在声明之间添加适当的空行
- **括号规则**: 处理括号的格式化
- **逗号规则**: 在逗号后添加空格
- **分号规则**: 处理语句结束符
- **行长度规则**: 限制行长度并自动换行
- **空白字符规则**: 处理尾随空白

## 架构设计

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   oak-core      │    │ oak-pretty-print│    │   具体语言      │
│                 │    │                 │    │                 │
│ ┌─────────────┐ │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │ AstNode     │ │◄───┤ │ Formatter   │ │◄───┤ │ Language    │ │
│ │ AstVisitor  │ │    │ │ FormatRule  │ │    │ │ Parser      │ │
│ │ Language    │ │    │ │ Config      │ │    │ │ Lexer       │ │
│ └─────────────┘ │    │ └─────────────┘ │    │ └─────────────┘ │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 扩展指南

### 实现自定义规则

```rust
use oak_pretty_print::{FormatRule, FormatContext, FormatResult};
use oak_core::AstNode;

struct MyCustomRule;

impl FormatRule for MyCustomRule {
    fn name(&self) -> &str {
        "my_custom_rule"
    }

    fn priority(&self) -> u8 {
        5
    }

    fn applies_to(&self, node: &dyn AstNode) -> bool {
        // 检查规则是否适用于该节点
        true
    }

    fn apply(&self, node: &dyn AstNode, context: &mut FormatContext) -> FormatResult<()> {
        // 实现格式化逻辑
        Ok(())
    }
}
```

### 集成到具体语言

要将 `oak-pretty-print` 集成到具体的编程语言中，需要：

1. 实现 `oak-core::Language` trait
2. 为语言的 AST 节点实现 `oak-core::AstNode` trait
3. 创建语言特定的格式化规则
4. 配置格式化器

## 依赖

- `oak-core`: 核心 AST 和语言抽象
- `alloc`: 用于动态内存分配（no_std 环境）

## 许可证

本项目采用与工作空间相同的许可证。


