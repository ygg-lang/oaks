#[test]
fn test_rust_formatter() {
    use oak_rust::RustFormatter;

    let formatter = RustFormatter::new();
    let source = "fn main(){let x=42;println!(\"x={}\",x);}";
    let formatted = formatter.format(source);

    assert!(!formatted.is_empty(), "Formatter should produce output");

    println!("Rust formatter test passed");
}

#[test]
fn test_rust_formatter_with_config() {
    use oak_rust::RustFormatter;

    let formatter = RustFormatter::with_config("    ".to_string(), 80);

    let source = "struct Point{x:f64,y:f64}impl Point{fn new(x:f64,y:f64)->Self{Point{x,y}}}";
    let formatted = formatter.format(source);

    assert!(!formatted.is_empty(), "Formatter with config should produce output");

    println!("Rust formatter with config test passed");
}
