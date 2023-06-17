#[test]
fn test_basic_verilog() {
    let content = include_str!("basic.v");
    assert!(!content.is_empty());
    assert!(content.contains("module"));
}
