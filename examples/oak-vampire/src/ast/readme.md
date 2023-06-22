# Vampire 抽象语法树 (AST) 模块

这个模块定义了 Vampire 定理证明器的抽象语法树结构。

## AST 节点类型

- **`VampireRoot`**: 根节点，包含多个公式
- **`VampireFormula`**: 公式，包含名称、角色和公式文本
- **`VampireInclude`**: 包含指令，用于引入其他文件

## 使用示例

```rust,no_run
#![feature(new_range_api)]
use oak_vampire::ast::*;
use core::range::Range;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = VampireRoot {
        span: Range { start: 0, end: 100 },
        formulas: vec![
            VampireFormula {
                span: Range { start: 0, end: 50 },
                name: "f1".to_string(),
                role: "conjecture".to_string(),
                formula: "p(a)".to_string(),
            }
        ],
    };
    
    Ok(())
}
```

## 设计原则

1. **完整性**: 支持完整的 Vampire 语法
2. **可扩展性**: 易于添加新的 AST 节点类型
3. **类型安全**: 使用 Rust 的类型系统确保 AST 的有效性
4. **性能**: 高效的内存使用和访问模式