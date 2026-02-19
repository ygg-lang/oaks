use oak_csharp::parse;

#[test]
fn test_top_level_statements() {
    let code = r#"
        Console.WriteLine("Hello, World!");
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse top-level statements");
}

#[test]
fn test_file_scoped_namespace() {
    let code = r#"
        namespace Test;
        
        public class Program {
            public static void Main() {
                Console.WriteLine("Hello");
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse file-scoped namespace");
}

#[test]
fn test_record_structs() {
    let code = r#"
        public record struct Point(int X, int Y);
        
        public class Program {
            public static void Main() {
                var point = new Point(10, 20);
                Console.WriteLine($"X: {point.X}, Y: {point.Y}");
            }
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse record structs");
}

#[test]
fn test_csharp12_features() {
    let code = r#"
        // Top-level statements
        using System;
        
        Console.WriteLine("Hello from top-level");
        
        // File-scoped namespace
        namespace Test;
        
        // Record struct
        public record struct Point(int X, int Y);
        
        // Class with primary constructor
        public class Person(string name, int age) {
            public string Name { get; set; } = name;
            public int Age { get; set; } = age;
        }
        
        // Inline array
        public struct Buffer {
            public fixed int Values[10];
        }
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse C# 12 features");
}
