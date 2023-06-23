# 馃洜锔?J Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-J`.

## 馃殾 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing an J package specification:

```rust,no_run
use oak_j::{JParser, JLanguage};
use oak_core::{SourceText, Parser, parser::ParseSession};

fn main() {
    // 1. 准备源代码
    let code = "a =: 1 + 2";
    let source = SourceText::new(code);

    // 2. 初始化解析器
    let config = JLanguage::default();
    let parser = JParser::new(&config);

    // 3. 执行解析
    let mut session = ParseSession::new(1024);
    let result = parser.parse(&source, &[], &mut session);

    // 4. 处理结果
    if result.result.is_ok() {
        println!("解析成功！");
    }
}
```

## 🔍 核心 API 用法

### 1. 语法树遍历
解析成功后，你可以使用内置的访问者模式或手动遍历 Green/Red Tree。

### 2. 增量解析
当源代码发生微小变化时，无需重新解析整个文档：
```rust,no_run
use oak_j::{JParser, JLanguage};
use oak_core::{SourceText, Parser, parser::ParseSession};

// 假设已经有了解析器实例 parser
# let config = JLanguage::default();
# let parser = JParser::new(&config);
// 假设你已经有了旧的解析结果 result 和新的源代码 new_source
# let new_source = SourceText::new("a =: 2");
let mut session = ParseSession::new(1024);
// 在实际场景中，session 会保留旧的树用于增量对比
let new_result = parser.parse(&new_source, &[], &mut session);
```

### 3. 诊断信息 (Diagnostics)
`oak-j` 提供了丰富的错误上下文：
```rust,no_run
# use oak_j::{JParser, JLanguage};
# use oak_core::{SourceText, Parser, parser::ParseSession};
# let config = JLanguage::default();
# let parser = JParser::new(&config);
# let source = SourceText::new("a =:");
# let mut session = ParseSession::new(1024);
# let result = parser.parse(&source, &[], &mut session);
for diag in result.diagnostics {
    println!("{:?}", diag);
}
```

## 馃彈锔?Architecture Overview

- **Lexer**: Tokenizes J source text into a stream of tokens, handling keywords (case-insensitive), operators, and numeric literals.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle J's structural declarations and expression precedence.
- **AST**: A strongly-typed syntax abstraction layer designed for building high-performance J analysis tools and IDEs.

## 馃敆 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various J edge cases and language versions.
