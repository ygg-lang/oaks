# WAT 词法分析器模块

WebAssembly 文本格式 (WAT) 的词法分析器，将源代码转换为词法单元 (tokens)。

## 功能特性

- **关键字识别**: 识别 WAT 关键字 (`module`, `func`, `export` 等)
- **标识符解析**: 处理标识符和名称
- **数值字面量**: 解析各种数值类型
- **字符串字面量**: 处理字符串常量
- **注释处理**: 支持行注释和块注释

## 词法单元类型

### 关键字
- 模块关键字: `module`, `func`, `export`, `import`
- 类型关键字: `param`, `result`, `local`
- 指令关键字: `i32.add`, `local.get`, `call`

### 字面量
- 数值: `123`, `0xFF`, `1.5`
- 字符串: `"hello"`, `$name`
- 标识符: `$func_name`

## 使用示例

```rust
use oak_wat::lexer::WatLexer;

let wat_source = r#"
    (module
        (func $add (param $a i32) (param $b i32) (result i32)
            local.get $a
            local.get $b
            i32.add)
    )
"#;

let mut lexer = WatLexer::new();
let tokens = lexer.tokenize(wat_source);

for token in tokens {
    println!("{:?}: {:?}", token.token_type, token.lexeme);
}
```

## 错误处理

词法分析器提供详细的错误信息：
- 非法字符
- 未结束的字符串
- 无效的数值格式
- 位置信息跟踪