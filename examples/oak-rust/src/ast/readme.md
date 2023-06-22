# Rust 抽象语法树 (AST) 模块

这个模块定义了 Rust 语言的抽象语法树结构，用于表示解析后的 Rust 代码。
AST 节点对应于 Rust 语言中的各种构造，如函数、结构体、枚举、模块、表达式等。

## AST 节点类型

### 顶层项目

- **`Function`**: 函数定义
- **`Struct`**: 结构体定义
- **`Enum`**: 枚举定义
- **`Module`**: 模块定义
- **`UseItem`**: use 语句
- **`Trait`**: trait 定义
- **`Impl`**: impl 块
- **`TypeAlias`**: 类型别名
- **`Const`**: 常量定义
- **`Static`**: 静态变量定义

### 类型系统

- **`Type`**: 类型表示（路径、引用、元组、数组、切片、函数指针）
- **`Identifier`**: 标识符
- **`Param`**: 函数参数
- **`Field`**: 结构体字段

### 语句和表达式

- **`Statement`**: 语句（let、表达式语句、return、break、continue）
- **`Expr`**: 表达式（标识符、字面量、二元运算、函数调用、字段访问、控制流等）
- **`Block`**: 代码块
- **`Pattern`**: 模式匹配模式

### 控制流

- **`If`**: if 表达式
- **`While`**: while 循环
- **`For`**: for 循环
- **`Loop`**: loop 循环
- **`Match`**: match 表达式
- **`MatchArm`**: match 分支

## 使用示例

```rust,ignore
use oak_rust::ast::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建简单的 Rust 程序 AST
    let root = RustRoot {
        items: vec![
            Item::Function(Function {
                name: Identifier {
                    name: "main".to_string(),
                    span: 0..4,
                },
                params: vec![],
                return_type: None,
                body: Block {
                    statements: vec![],
                    span: 5..7,
                },
                span: 0..7,
            })
        ],
    };
    
    println!("Created Rust AST with {} items", root.items.len());
    Ok(())
}
```

## 设计原则

1. **完整性**: 支持完整的 Rust 语法结构
2. **可扩展性**: 易于添加新的 AST 节点类型
3. **类型安全**: 使用 Rust 的类型系统确保 AST 的有效性
4. **性能**: 高效的内存使用和访问模式
5. **位置信息**: 每个节点都包含源代码位置信息，便于错误报告和工具支持