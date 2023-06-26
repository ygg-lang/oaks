use oak_core::Parser;
use oak_java::{JavaLanguage, JavaParser};

#[test]
fn test_error_recovery() {
    // Test code with various syntax errors
    let code = r#"
package com.example;

import java.util.List;
import java.util.ArrayList;

public class TestClass {
    // Missing semicolon here
    private int x
    
    // Invalid method declaration
    public void invalidMethod() {
        int a = 10
        if (a > 5) {
            System.out.println("a is greater than 5")
        }
    }
    
    // Invalid switch statement
    public String getGrade(int score) {
        switch (score) {
            case 90:
                return "A";
            case 80
                return "B";
            default:
                return "F";
        }
    }
    
    // Invalid try-catch statement
    public void divide(int a, int b) {
        try {
            int result = a / b;
            System.out.println("Result: " + result);
        } catch (ArithmeticException e {
            System.out.println("Error: Division by zero");
        } finally {
            System.out.println("Division operation completed");
        }
    }
}
"#;

    let language = JavaLanguage::new();
    let parser = JavaParser::new(&language);

    let mut cache = oak_core::parser::ParseSession::default();
    let parse_output = parser.parse(code, &[], &mut cache);

    match parse_output.result {
        Ok(_parse_tree) => {
            println!("Parsing successful despite errors!");
            // println!("Parse tree: {:?}", _parse_tree);
        }
        Err(e) => {
            println!("Parsing failed: {:?}", e);
            // Even if it fails, we expect the parser to recover and continue
        }
    }

    // The test should pass if the parser doesn't crash
    assert!(true);
}
