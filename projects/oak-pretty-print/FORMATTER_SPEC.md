# Oak 格式化系统设计规范

## 1. 简介

### 1.1 文档目的
本文档定义了 Oak 格式化系统的设计规范，包括架构设计、核心组件、接口定义和实现要求。该系统由两个主要部分组成：`oak-formatter`（高层格式化库）和 `oak-pretty-print`（底层格式化库）。

### 1.2 术语定义
- **AST (Abstract Syntax Tree)**：抽象语法树，是源代码的抽象表示，不包含注释和空白等信息。
- **CST (Concrete Syntax Tree)**：具体语法树，也称为红绿树（Red-Green Tree），包含源代码的完整结构，包括注释和空白。
- **注解 (Annotation)**：代码中的特殊标记，用于控制格式化行为，如 Rust 中的 `#[rustfmt]`。
- **FormatState**：格式化状态，用于在格式化过程中传递动态配置和状态。

## 2. 系统架构

### 2.1 整体架构

```
oak-formatter (高层格式化库)
├── 语言特定的格式化器 (AST 层面)
├── 配置管理 (每个语言定义自己的 Config)
├── 注解处理系统
└── 依赖 oak-pretty-print

oak-pretty-print (底层格式化库)
├── 文档构建和渲染
├── 红绿树层面的格式化
├── 注释和空白处理
└── 依赖 oak-formatter 的配置
```

### 2.2 核心组件

#### 2.2.1 oak-formatter
- **语言特定格式化器**：为每种语言实现的格式化逻辑，工作在 AST 层面。
- **配置管理**：每种语言定义自己的配置结构，管理语言特定的格式化选项。
- **注解处理系统**：解析和处理代码中的格式化注解，如 `#[rustfmt]`。

#### 2.2.2 oak-pretty-print
- **文档构建**：构建格式化文档的工具。
- **红绿树处理**：在红绿树层面进行格式化，保留原始的空格和注释。
- **注释处理**：智能处理和重定位注释。
- **空白处理**：处理空格和空白行的规范化。

## 3. 接口定义

### 3.1 oak-formatter 接口

#### 3.1.1 配置接口
```rust
// 语言特定的配置结构示例
pub struct RustFormatterConfig {
    pub indent_style: IndentStyle,
    pub max_width: u32,
    pub newline_style: NewlineStyle,
    // 其他语言特定选项
}

// 所有配置结构应实现的 trait
pub trait FormatterConfig {
    type State: Default + Clone;
    
    fn default() -> Self;
    fn state(&self) -> Self::State;
}
```

#### 3.1.2 格式化器接口
```rust
pub trait LanguageFormatter {
    type Config: FormatterConfig;
    type Error: std::error::Error;
    
    fn new(config: Self::Config) -> Self;
    fn format(&mut self, ast: &AstNode, source: &str) -> Result<FormatOutput, Self::Error>;
}
```

#### 3.1.3 注解接口
```rust
pub trait AnnotationParser {
    type Annotation;
    
    fn parse(&self, node: &AstNode) -> Vec<Self::Annotation>;
}

pub trait AnnotationProcessor {
    type Annotation;
    type State;
    
    fn process(&self, annotation: &Self::Annotation, state: &mut Self::State);
}
```

### 3.2 oak-pretty-print 接口

#### 3.2.1 文档构建接口
```rust
pub trait DocumentBuilder {
    fn text(&mut self, text: &str);
    fn line_break(&mut self);
    fn indent(&mut self);
    fn dedent(&mut self);
    fn group(&mut self, content: impl FnOnce(&mut Self));
}
```

#### 3.2.2 格式化接口
```rust
pub trait TreeFormatter {
    type State;
    
    fn format_node(&mut self, node: &GreenNode, state: &mut Self::State) -> Result<(), Self::Error>;
    fn format_comment(&mut self, comment: &Comment, state: &mut Self::State) -> Result<(), Self::Error>;
}
```

#### 3.2.3 空白处理接口
```rust
pub trait WhitespaceProcessor {
    fn process(&self, whitespace: &str, state: &mut impl FormatState) -> String;
    fn process_blank_lines(&self, lines: usize, state: &mut impl FormatState) -> String;
}
```

## 4. 实现要求

### 4.1 配置系统
- 每种语言必须定义自己的配置结构，不使用通用的 BaseFormatConfig。
- 配置结构应使用组合而非继承的方式组织。
- 配置应支持序列化和反序列化（可选）。

### 4.2 状态管理
- 使用 FormatState 管理格式化过程中的动态状态。
- FormatState 应支持状态的继承和覆盖。
- 从注解中解析的局部配置应存储在 FormatState 中。

### 4.3 注解系统
- 支持类似 `#[rustfmt]` 的注解语法。
- 支持在注解中指定局部格式化规则。
- 支持注解的作用域管理。

### 4.4 红绿树处理
- 在红绿树层面进行格式化，保留原始的空格和注释。
- 支持注释的智能处理和重定位。
- 支持空白行的保留和规范化。

### 4.5 性能优化
- 缓存格式化结果，避免重复计算。
- 使用高效的文档构建算法。
- 优化注解解析和处理的性能。

### 4.6 扩展性
- 支持自定义格式化规则。
- 支持自定义注解类型。
- 支持插件系统，允许扩展格式化功能。

## 5. 依赖关系

### 5.1 项目依赖
- `oak-formatter` 依赖 `oak-pretty-print` 的底层格式化功能，用于在红绿树层面进行格式化。
- `oak-pretty-print` 不依赖 `oak-formatter`，而是提供独立的格式化基础功能。

### 5.2 外部依赖
- `oak-core`：提供核心数据结构和工具。
- `serde`（可选）：用于配置的序列化和反序列化。
- `regex`（可选）：用于注解解析。

## 6. 示例用法

### 6.1 基本用法

```rust
// 使用语言特定的格式化器
use oak_formatter::rust::RustFormatter;

let config = RustFormatterConfig::new()
    .with_indent_style(IndentStyle::Spaces(4))
    .with_max_width(100);

let mut formatter = RustFormatter::new(config);
let output = formatter.format(&ast, source)?;
println!("{}", output.content);
```

### 6.2 使用注解

```rust
// 代码中的注解
#[rustfmt(indent_style = "tabs")]
fn foo() {
    let x = 123;
}

// 格式化时会自动应用注解中的配置
let output = formatter.format(&ast, source)?;
```

## 7. 测试计划

### 7.1 单元测试
- 测试配置系统的功能。
- 测试注解解析和处理。
- 测试红绿树层面的格式化。

### 7.2 集成测试
- 测试整个格式化流程。
- 测试不同语言的格式化结果。

### 7.3 性能测试
- 测试格式化的性能。
- 测试大型文件的处理能力。

### 7.4 回归测试
- 确保修改不会破坏现有功能。
- 确保格式化结果的一致性。

## 8. 风险评估

### 8.1 复杂性风险
- **风险**：系统变得过于复杂，难以理解和维护。
- **缓解措施**：模块化设计，清晰的职责划分，详细的文档。

### 8.2 性能风险
- **风险**：格式化过程变得缓慢。
- **缓解措施**：性能优化，缓存机制，高效的算法。

### 8.3 兼容性风险
- **风险**：修改现有 API，破坏向后兼容性。
- **缓解措施**：保持 API 兼容性，提供迁移指南。

### 8.4 正确性风险
- **风险**：格式化结果不正确。
- **缓解措施**：全面的测试，代码审查，用户反馈。

## 9. 结论

通过本设计规范，我们将创建一个功能强大、灵活且高效的格式化系统，满足现代编程语言对格式化工具的需求。该系统将支持参数化格式化、inline 配置，并在红绿树层面进行处理，确保格式化结果的正确性和一致性。同时，清晰的职责划分将使系统易于维护和扩展，为未来的功能增强做好准备。