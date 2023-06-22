# Rust 词法分析器

这个模块提供了 Rust 编程语言的词法分析功能，将源代码文本转换为 token 流。

## 概述

词法分析器（Lexer）是编译器前端的第一个阶段，负责将原始的源代码文本分解为有意义的 token 序列。这个模块专门为 Rust 语言设计，支持 Rust 的所有语法元素。

## 核心组件

### Token 类型 (`RustTokenType`)

定义了 Rust 语言中所有可能的 token 类型：

- **空白字符**: `Space`, `Newline`
- **分隔符**: `LeftParen`, `RightParen`, `LeftBracket`, `RightBracket`, `LeftBrace`, `RightBrace`
- **标点符号**: `Semicolon`, `Comma`, `Dot`, `Colon`, `DoubleColon`, `Question`, `At`, `Hash`, `Dollar`
- **操作符**: `Plus`, `Minus`, `Star`, `Slash`, `Percent`, `Caret`, `Ampersand`, `Pipe`, `Tilde`, `Bang`, `Eq`, `Lt`, `Gt`
- **复合操作符**: `PlusEq`, `MinusEq`, `StarEq`, `SlashEq`, `PercentEq`, `EqEq`, `Ne`, `Le`, `Ge`, `AndAnd`, `OrOr`, `Shl`, `Shr`
- **Rust 特定操作符**: `DotDot`, `DotDotEq`, `Arrow`, `FatArrow`
- **关键字**: `Keyword(RustKeywords)`
- **标识符和字面量**: `Identifier`, `IntegerLiteral`, `FloatLiteral`, `StringLiteral`, `CharLiteral`, `BoolLiteral`
- **注释**: `LineComment`, `BlockComment`, `DocComment`
- **特殊 token**: `Error`, `Eof`, `Lifetime`

### 关键字 (`RustKeywords`)

包含 Rust 语言的所有关键字类型：

- **严格关键字**: `as`, `break`, `const`, `continue`, `crate`, `else`, `enum`, `extern`, `false`, `fn`, `for`, `if`, `impl`, `in`, `let`, `loop`, `match`, `mod`, `move`, `mut`, `pub`, `ref`, `return`, `self`, `Self`, `static`, `struct`, `super`, `trait`, `true`, `type`, `unsafe`, `use`, `where`, `while`
- **保留关键字**: `abstract`, `become`, `box`, `do`, `final`, `macro`, `override`, `priv`, `typeof`, `unsized`, `virtual`, `yield`
- **弱关键字**: `async`, `await`, `dyn`, `try`, `union`
- **版本特定关键字**: `raw`

### 词法分析器 (`RustLexer`)

主要的词法分析器实现，提供以下功能：

- **空白字符处理**: 识别和跳过空格、制表符、换行符等
- **注释处理**: 支持行注释 (`//`) 和块注释 (`/* */`)，包括嵌套块注释
- **字符串字面量**: 支持普通字符串、原始字符串、字节字符串
- **字符字面量**: 支持字符和字节字符
- **数字字面量**: 支持整数、浮点数，包括类型后缀和进制前缀
- **标识符和关键字**: 使用 Unicode 标准识别标识符
- **操作符**: 支持所有 Rust 操作符，包括复合赋值操作符
- **生命周期**: 识别生命周期参数 (`'a`, `'static` 等)

## 使用方法

词法分析器通常通过 `RustLexer` 在解析 Rust 源代码时使用：

```rust
use oak_rust::{RustLexer, RustLanguage};
use oak_core::source::StringSource;

let language = RustLanguage::default();
let lexer = RustLexer::new(&language);
let source = StringSource::new("fn main() { let x = 42; }");

// 词法分析将在解析过程中自动进行
```

## Rust 语言特性支持

词法分析器支持 Rust 语言的所有主要特性：

- **原始字符串**: `r"string"`, `r#"string"#`, `r##"string"##`
- **字节字符串**: `b"bytes"`, `br"raw bytes"`
- **字符字面量**: `'a'`, `'\n'`, `'\u{1F600}'`
- **数字字面量**: `42`, `3.14`, `0x1A`, `0o755`, `0b1010`, `42u32`, `3.14f64`
- **生命周期**: `'a`, `'static`, `'_`
- **嵌套块注释**: `/* outer /* inner */ outer */`
- **文档注释**: `///`, `//!`, `/** */`, `/*! */`

## 错误处理

词法分析器提供全面的错误处理：

- **无效字符**: 识别源代码中的无效字符
- **未终止的字符串**: 检测未正确关闭的字符串字面量
- **无效的数字格式**: 识别格式错误的数字字面量
- **位置信息**: 为所有错误提供精确的源代码位置

## 性能特性

- **增量词法分析**: 支持增量更新，只重新分析更改的部分
- **零拷贝**: 尽可能避免字符串复制
- **Unicode 支持**: 完整支持 Unicode 标识符和字符串
- **内存效率**: 优化的内存使用模式

## 设计原则

1. **完整性**: 支持完整的 Rust 语法规范
2. **准确性**: 精确匹配 Rust 编译器的词法分析行为
3. **性能**: 高效的词法分析算法
4. **可扩展性**: 易于添加新的语言特性支持