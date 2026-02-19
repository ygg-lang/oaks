use oak_core::{Parser, parser::ParseSession};
use oak_java::{JavaLanguage, JavaParser};
use std::time::Instant;

#[test]
fn test_java_parser_performance() {
    // Generate a large Java file for testing
    let mut large_java_file = String::new();

    // Add package declaration
    large_java_file.push_str(
        "package com.example;

",
    );

    // Add imports
    large_java_file.push_str(
        "import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.HashMap;

",
    );

    // Add a large class with many methods
    large_java_file.push_str(
        "public class LargeClass {
",
    );

    // Add many fields
    for i in 0..1000 {
        large_java_file.push_str(&format!(
            "    private int field{};
",
            i
        ));
    }

    // Add many methods
    for i in 0..1000 {
        large_java_file.push_str(&format!(
            "    public void method{}() {{
        for (int j = 0; j < 100; j++) {{
            field{} += j;
        }}
    }}

",
            i, i
        ));
    }

    large_java_file.push_str(
        "}
",
    );

    println!("Generated Java file size: {} bytes", large_java_file.len());

    let language = JavaLanguage {};
    let parser = JavaParser::new(&language);
    let mut session = ParseSession::new(1024);

    // Measure parsing time
    let start = Instant::now();
    let result = parser.parse(large_java_file.as_str(), &[], &mut session);
    let duration = start.elapsed();

    println!("Parsing took: {:?}", duration);
    println!("Parse result: {:?}", result.result.is_ok());

    // Ensure the parser doesn't panic
    assert!(result.result.is_ok());
}
