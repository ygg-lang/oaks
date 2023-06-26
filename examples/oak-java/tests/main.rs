use oak_core::Parser;
use oak_java::{JavaLanguage, JavaParser};

#[test]
fn test_java_parser() {
    let code = r#"
package com.example;

import java.util.List;
import java.util.ArrayList;

/**
 * A test class for Java parser
 */
public class TestClass {
    // Class field
    private int x;
    public static final String CONSTANT = "test";
    
    // Constructor
    public TestClass() {
        this.x = 0;
    }
    
    // Method with parameters and return value
    public int add(int a, int b) {
        return a + b;
    }
    
    // Method with if-else statement
    public String getGrade(int score) {
        if (score >= 90) {
            return "A";
        } else if (score >= 80) {
            return "B";
        } else if (score >= 70) {
            return "C";
        } else {
            return "D";
        }
    }
    
    // Method with loop statements
    public void printNumbers(int n) {
        for (int i = 0; i < n; i++) {
            System.out.println(i);
        }
        
        int j = 0;
        while (j < n) {
            System.out.println(j);
            j++;
        }
        
        do {
            System.out.println(j);
            j++;
        } while (j < n * 2);
    }
    
    // Method with switch statement
    public String getMonthName(int month) {
        switch (month) {
            case 1:
                return "January";
            case 2:
                return "February";
            case 3:
                return "March";
            default:
                return "Invalid month";
        }
    }
    
    // Method with try-catch statement
    public void divide(int a, int b) {
        try {
            int result = a / b;
            System.out.println("Result: " + result);
        } catch (ArithmeticException e) {
            System.out.println("Error: Division by zero");
        } finally {
            System.out.println("Division operation completed");
        }
    }
    
    // Method with array operations
    public int[] createArray(int size) {
        int[] array = new int[size];
        for (int i = 0; i < size; i++) {
            array[i] = i * 2;
        }
        return array;
    }
    
    // Method with lambda expression
    public void processList(List<Integer> list) {
        list.forEach(item -> System.out.println(item));
    }
}
"#;

    let language = JavaLanguage::new();
    let parser = JavaParser::new(&language);

    let mut cache = oak_core::parser::ParseSession::default();
    let parse_output = parser.parse(code, &[], &mut cache);

    match parse_output.result {
        Ok(_parse_tree) => {
            println!("Parsing successful!");
            // println!("Parse tree: {:?}", _parse_tree);
        }
        Err(e) => {
            println!("Parsing failed: {:?}", e);
            panic!("Parsing failed");
        }
    }
}
