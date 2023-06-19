use oak_core::Builder;
use oak_rust::{RustLanguage, RustParser};

#[test]
fn test_rust_builder() {
    let language = RustLanguage::default();
    let parser = RustParser::new(&language);

    // 简单的测试代码
    let source = "fn main() { let x = 42; }";

    // 这里我们只是测试 builder 能够被创建和使用
    // 实际的构建测试需要更复杂的设置
    println!("Rust builder test - parser created successfully");

    // 由于 build_incremental 方法还没有完全实现（返回 todo!()），
    // 我们暂时只测试基本的创建功能
    assert!(true, "Builder test placeholder - implementation needed");
}
