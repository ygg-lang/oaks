# WebAssembly 文本格式 (WAT) 处理模块

这个模块提供了完整的 WAT (WebAssembly Text) 格式处理功能，包括：
- **词法分析**: 将 WAT 文本分解为词法单元 (tokens)
- **语法分析**: 将词法单元解析为抽象语法树 (AST)
- **编译**: 将 AST 编译为 WASM 二进制结构
- **反编译**: 将 WASM 结构转换回 WAT 文本

## 模块组件

### `ast` 模块

定义 WAT 抽象语法树的所有节点类型：
- `Module`: 模块定义
- `Func`: 函数定义
- `Export`: 导出定义
- `Import`: 导入定义
- `Memory`: 内存定义
- `Table`: 表定义
- `Global`: 全局变量定义
- `Instruction`: WebAssembly 指令

### `lexer` 模块

词法分析器，将 WAT 文本转换为词法单元：
- 关键字识别 (`module`, `func`, `export`, 等)
- 标识符和名称解析
- 数值字面量处理
- 字符串字面量处理
- 注释和空白字符处理

### `parser` 模块

语法分析器，将词法单元解析为 AST：
- 递归下降解析
- 错误恢复和报告
- 语法验证
- 位置信息跟踪

### `compiler` 模块

编译器，将 AST 编译为 WASM 结构：
- 类型检查
- 符号解析
- 指令编码
- 模块生成

### `writer` 模块

写入器，将 AST 转换回 WAT 文本：
- 格式化输出
- 注释生成
- 代码美化

## 使用示例

### 基本解析和编译

```rust,no_run
use wasi_assembler::formats::wat::{WatParser, WatCompiler};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wat_source = r#"
        (module
            (func $add (param $a i32) (param $b i32) (result i32)
                local.get $a
                local.get $b
                i32.add
            )
            (export "add" (func $add))
        )
    "#;
    
    // 解析 WAT 文本
    let mut parser = WatParser::new();
    let ast = parser.parse(wat_source)?;
    
    // 编译为 WASM 结构
    let mut compiler = WatCompiler::new();
    let wasm_module = compiler.compile(ast)?;
    Ok(())
}
```

### 错误处理

```rust,no_run
use wasi_assembler::formats::wat::{WatParser, WatError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = WatParser::new();
    match parser.parse("(module (func $invalid") {
        Ok(ast) => {
            // 解析成功
        }
        Err(WatError::UnexpectedToken { expected, found, location }) => {
            eprintln!("语法错误: 期望 {:?}, 找到 {:?} 在位置 {:?}", expected, found, location);
        }
        Err(WatError::UnexpectedEof) => {
            eprintln!("意外结束: 输入不完整");
        }
        Err(e) => {
            eprintln!("解析错误: {}", e);
        }
    }
    Ok(())
}
```