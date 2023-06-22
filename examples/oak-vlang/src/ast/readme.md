# VLang 抽象语法树 (AST) 模块

这个模块定义了 V 语言的抽象语法树结构，用于表示解析后的 V 代码。

## AST 节点类型

### 顶级节点

- **`VRoot`**: V 源代码的根节点，包含模块名、导入和项目列表。
- **`VItem`**: 顶级项目，可以是结构体、函数、枚举或常量。

### 类型定义

- **`VStruct`**: 结构体定义。
- **`VEnum`**: 枚举定义。
- **`VField`**: 结构体字段。

### 函数

- **`VFunction`**: 函数或方法定义。
- **`VReceiver`**: 方法的接收者。
- **`VParam`**: 函数参数。

### 其他

- **`VConst`**: 常量定义。

## 使用示例

```rust,ignore
use oak_vlang::ast::*;

fn main() {
    let root = VRoot {
        module_name: "main".to_string(),
        imports: vec!["os".to_string()],
        items: vec![
            VItem::Function(VFunction {
                name: "main".to_string(),
                is_pub: true,
                receiver: None,
                params: vec![],
                return_type: None,
                body: vec!["println('hello world')".to_string()],
            }),
        ],
    };
}
```
