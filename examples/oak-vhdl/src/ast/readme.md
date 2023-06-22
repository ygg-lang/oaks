# VHDL 抽象语法树 (AST) 模块

这个模块定义了 VHDL 的抽象语法树结构，用于表示解析后的 VHDL 代码。

## AST 节点类型

### 根节点

- **`VhdlRoot`**: VHDL 文件的根节点，包含多个设计单元

### 设计单元

- **`DesignUnit`**: VHDL 设计单元，可以是实体、架构体或包
- **`EntityDeclaration`**: 实体声明，定义端口
- **`ArchitectureBody`**: 架构体，定义实体的具体实现
- **`PackageDeclaration`**: 包声明，定义可重用的类型和函数

### 端口与信号

- **`PortDeclaration`**: 端口声明（in, out, inout 等）
- **`SignalDeclaration`**: 信号声明
- **`PortDirection`**: 端口方向枚举

## 使用示例

```rust
use oak_vhdl::ast::*;

fn main() {
    // 创建简单的 VHDL 实体 AST
    let entity = EntityDeclaration {
        name: "counter".to_string(),
        ports: vec![
            PortDeclaration {
                name: "clk".to_string(),
                direction: PortDirection::In,
                data_type: "std_logic".to_string(),
            },
            PortDeclaration {
                name: "count".to_string(),
                direction: PortDirection::Out,
                data_type: "std_logic_vector(3 downto 0)".to_string(),
            },
        ],
    };
    
    let root = VhdlRoot {
        units: vec![DesignUnit::Entity(entity)],
    };
}
```

## 设计原则

1. **完整性**: 支持完整的 VHDL 语法
2. **可扩展性**: 易于添加新的 AST 节点类型
3. **类型安全**: 使用 Rust 的类型系统确保 AST 的有效性
4. **性能**: 高效的内存使用和访问模式
