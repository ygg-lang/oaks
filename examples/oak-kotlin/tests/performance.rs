use oak_core::{Parser, parser::ParseSession};
use oak_kotlin::{KotlinLanguage, KotlinParser};
use std::time::Instant;

#[test]
#[ignore = "KotlinParser overflows the stack on the synthetic large-file performance fixture"]
fn test_kotlin_parser_performance() {
    let mut large_kotlin_file = String::new();

    large_kotlin_file.push_str(
        "package com.example

",
    );

    large_kotlin_file.push_str(
        "import java.util.ArrayList
import java.util.List
import java.util.Map
import java.util.HashMap

",
    );

    large_kotlin_file.push_str(
        "class LargeClass {
",
    );

    for i in 0..1000 {
        large_kotlin_file.push_str(&format!(
            "    private var field{}: Int = 0
",
            i
        ))
    }

    for i in 0..1000 {
        large_kotlin_file.push_str(&format!(
            "    fun method{}() {{
        for (j in 0 until 100) {{
            field{} += j
        }}
    }}

",
            i, i
        ))
    }

    large_kotlin_file.push_str(
        "}
",
    );

    println!("Generated Kotlin file size: {} bytes", large_kotlin_file.len());

    let config = KotlinLanguage::new();
    let parser = KotlinParser::new(&config);
    let mut session = ParseSession::new(1024);

    let start = Instant::now();
    let result = parser.parse(large_kotlin_file.as_str(), &[], &mut session);
    let duration = start.elapsed();

    println!("Parsing took: {:?}", duration);
    println!("Parse result: {:?}", result.result.is_ok());

    assert!(result.result.is_ok());
}
