# WASI WAT (WebAssembly Text Format) 抽象语法树 (AST) 模块

这个模块定义了 WASI WAT Component Model 的抽象语法树结构，用于表示解析后的 WAT 代码。
AST 节点对应于 WAT 语言中的各种构造，如组件、模块、导入、导出、类型定义等。

## AST 节点类型

### 模块级别

- **`Module`**: 核心模块定义
- **`Component`**: 组件定义（WASM Component Model）
- **`Import`**: 导入定义
- **`Export`**: 导出定义
- **`Type`**: 类型定义

### 函数级别

- **`Func`**: 函数定义
- **`FuncType`**: 函数类型定义
- **`Param`**: 函数参数
- **`Result`**: 函数返回值
- **`Local`**: 局部变量

### 表达式级别

- **`Instruction`**: WebAssembly 指令
- **`Block`**: 块结构
- **`Loop`**: 循环结构
- **`If`**: 条件结构
- **`Call`**: 函数调用

## 使用示例

```rust,no_run
use oak_wat::ast::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建简单的模块 AST
    let module = WatModule {
        name: Some("my_module".to_string()),
        items: vec![
            WatItem::Import(WatImport {
                module: "env".to_string(),
                name: "print".to_string(),
                kind: WatImportKind::Function(WatFunctionType {
                    params: vec![],
                    results: vec![WatType::I32],
                }),
            }),
            WatItem::Function(WatFunction {
                name: Some("main".to_string()),
                params: vec![],
                result: Some(WatType::I32),
                locals: vec![],
                body: vec![
                    WatInstruction::Simple("i32.const 42".to_string()),
                    WatInstruction::Call { function: "print".to_string() },
                ],
            }),
            WatItem::Export(WatExport {
                name: "main".to_string(),
                kind: WatExportKind::Function("main".to_string()),
            }),
        ],
    };
    
    Ok(())
}
```

## 设计原则

1. **完整性**: 支持完整的 WebAssembly 和 Component Model 语法
2. **可扩展性**: 易于添加新的 AST 节点类型
3. **类型安全**: 使用 Rust 的类型系统确保 AST 的有效性
4. **性能**: 高效的内存使用和访问模式